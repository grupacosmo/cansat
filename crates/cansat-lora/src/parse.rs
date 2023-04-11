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

pub fn response(input: &[u8]) -> IResult<&[u8], Response> {
    map(pair(header, content), |(header, content)| Response {
        header,
        content,
    })(input)
}

fn header(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, _prefix) = tag("+")(input).map_nom_err(|_: Error| Error::NoPrefix)?;
    let (input, header) = take_while1(|c| (b'A'..=b'Z').contains(&c))(input)
        .map_nom_err(|_: Error| Error::BadCommand)?;
    let (input, _delimiter) = tag(": ")(input).map_nom_err(|_: Error| Error::NoDelimiter)?;

    Ok((input, header))
}

fn content(input: &[u8]) -> IResult<&[u8], ResponseContent> {
    let (input, data) = alt((
        map(error, ResponseContent::Error),
        map(data, ResponseContent::Data),
    ))(input)?;
    let (input, _) = line_ending(input).map_nom_err(|_: Error| Error::NoTerminator)?;
    Ok((input, data))
}

fn error(input: &[u8]) -> IResult<&[u8], i8> {
    let (input, _) = tag("ERROR(")(input)?;
    let (input, code) = cut(i8)(input).map_nom_err(|_: Error| Error::BadErrorCode)?;
    let (input, _) = cut(tag(")"))(input).map_nom_err(|_: Error| Error::UnclosedErrorParen)?;
    Ok((input, code))
}

fn data(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(|c| b"\r\n".contains(&c))(input)
}

/// Extension for `Result<T, nom::Err<E>>` to simplify error mapping.
trait MapNomErrExt {
    type Unwrapped;
    type Wrapped<E2>;

    fn map_nom_err<E2, F>(self, f: F) -> Self::Wrapped<E2>
    where
        F: Fn(Self::Unwrapped) -> E2;
}

impl<T, E1> MapNomErrExt for Result<T, nom::Err<E1>> {
    type Unwrapped = E1;
    type Wrapped<E> = Result<T, nom::Err<E>>;

    fn map_nom_err<E2, F>(self, f: F) -> Self::Wrapped<E2>
    where
        F: Fn(Self::Unwrapped) -> E2,
    {
        self.map_err(|e| e.map(f))
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
