use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{i8, line_ending},
    combinator::{cut, map},
    sequence::pair,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Response<'a> {
    pub header: &'a [u8],
    pub content: ResponseContent<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResponseContent<'a> {
    Data(&'a [u8]),
    Error(i8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BadCommand,
    BadErrorCode,
    NoDelimiter,
    NoPrefix,
    NoTerminator,
    UnclosedErrorParen,
    Unknown,
}

pub type IResult<I, O> = nom::IResult<I, O, Error>;

impl<I> nom::error::ParseError<I> for Error {
    fn from_error_kind(_input: I, _kind: nom::error::ErrorKind) -> Self {
        Self::Unknown
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

type NomError<'a> = nom::error::Error<&'a [u8]>;

pub fn response(input: &[u8]) -> IResult<&[u8], Response> {
    map(pair(header, content), |(header, content)| Response {
        header,
        content,
    })(input)
}

fn header(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let result = tag("+")(input);
    let (input, _prefix) = map_nom_err(result, |_: NomError| Error::NoPrefix)?;

    let result = take_while1(|c| (b'A'..b'Z').contains(&c))(input);
    let (input, header) = map_nom_err(result, |_: NomError| Error::BadCommand)?;

    let result = tag(": ")(input);
    let (input, _delimiter) = map_nom_err(result, |_: NomError| Error::NoDelimiter)?;

    Ok((input, header))
}

fn content(input: &[u8]) -> IResult<&[u8], ResponseContent> {
    let (input, data) = alt((
        map(error, ResponseContent::Error),
        map(data, ResponseContent::Data),
    ))(input)?;

    let result = line_ending(input);
    let (input, _) = map_nom_err(result, |_: NomError| Error::NoTerminator)?;

    Ok((input, data))
}

fn error(input: &[u8]) -> IResult<&[u8], i8> {
    let (input, _) = tag("ERROR(")(input)?;

    let result = cut(i8)(input);
    let (input, code) = map_nom_err(result, |_: NomError| Error::BadErrorCode)?;

    let result = cut(tag(")"))(input);
    let (input, _) = map_nom_err(result, |_: NomError| Error::UnclosedErrorParen)?;

    Ok((input, code))
}

fn data(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(|c| b"\r\n".contains(&c))(input)
}

pub fn map_nom_err<I, O, E1, E2, F>(r: nom::IResult<I, O, E1>, f: F) -> nom::IResult<I, O, E2>
where
    F: Fn(E1) -> E2,
{
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(e.map(f)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn error() {
        let (rest, response) = response(b"+DR: ERROR(-1)\r\n").unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            response,
            Response {
                header: b"DR",
                content: ResponseContent::Error(-1)
            }
        );
    }

    #[test]
    fn data() {
        let (rest, response) = response(b"+DR: not an error\r\n").unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            response,
            Response {
                header: b"DR",
                content: ResponseContent::Data(b"not an error")
            }
        );
    }

    #[test]
    fn no_prefix() {
        let error = response(b"DR: not an error\r\n").unwrap_err();
        assert_eq!(error, nom::Err::Error(Error::NoPrefix));
    }

    #[test]
    fn bad_command() {
        let error = response(b"+: not an error\r\n").unwrap_err();
        assert_eq!(error, nom::Err::Error(Error::BadCommand));
    }

    #[test]
    fn unclodes_error_paren() {
        let error = response(b"+DR: ERROR(-1\r\n").unwrap_err();
        assert_eq!(error, nom::Err::Failure(Error::UnclosedErrorParen));
    }

    #[test]
    fn bad_error_code() {
        let error = response(b"+DR: ERROR(code)\r\n").unwrap_err();
        assert_eq!(error, nom::Err::Failure(Error::BadErrorCode));
    }

    #[test]
    fn no_delimiter() {
        let error = response(b"+DR not an error\r\n").unwrap_err();
        assert_eq!(error, nom::Err::Error(Error::NoDelimiter));
    }

    #[test]
    fn no_terminator() {
        let error = response(b"+DR: not an error").unwrap_err();
        assert_eq!(error, nom::Err::Error(Error::NoTerminator));
    }

    #[test]
    fn no_error_terminator() {
        let error = response(b"+DR: ERROR(-1)").unwrap_err();
        assert_eq!(error, nom::Err::Error(Error::NoTerminator));
    }
}
