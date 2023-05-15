use crate::{ParseError as Error, Response, ResponseContent};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{i8, line_ending},
    combinator::{cut, map},
    sequence::{delimited, pair, terminated},
};

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
    let prefix = map_err(tag("+"), |_: Error| Error::NoPrefix);
    let header = map_err(take_while1(|c: u8| c.is_ascii_uppercase()), |_: Error| {
        Error::BadCommand
    });
    let delimiter = map_err(tag(": "), |_: Error| Error::NoDelimiter);
    delimited(prefix, header, delimiter)(input)
}

fn content(input: &[u8]) -> IResult<&[u8], ResponseContent> {
    let content = alt((
        map(error, ResponseContent::Error),
        map(data, ResponseContent::Data),
    ));
    let terminator = map_err(line_ending, |_: Error| Error::NoTerminator);
    terminated(content, terminator)(input)
}

fn error(input: &[u8]) -> IResult<&[u8], i8> {
    let opening = tag("ERROR(");
    let code = map_err(cut(i8), |_: Error| Error::BadErrorCode);
    let closing = map_err(cut(tag(")")), |_: Error| Error::UnclosedErrorParen);
    delimited(opening, code, closing)(input)
}

fn data(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(|c| b"\r\n".contains(&c))(input)
}

/// Parser combinator that maps error
fn map_err<I, O, E1, E2, F, G>(mut parser: F, mut f: G) -> impl FnMut(I) -> nom::IResult<I, O, E2>
where
    F: nom::Parser<I, O, E1>,
    G: FnMut(E1) -> E2,
{
    move |input: I| parser.parse(input).map_err(|e| e.map(&mut f))
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
