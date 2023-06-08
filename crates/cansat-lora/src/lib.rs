#![no_std]

mod parser;

use core::fmt::Debug;
use embedded_hal::{self, nb, serial};

#[derive(Debug, PartialEq, Eq, Clone, Copy, derive_more::From)]
pub enum Error<SerialError> {
    Serial(SerialError),
    #[from]
    Parse(ParseError),
    Overflow,
}

#[cfg(feature = "defmt")]
impl<SerialError: Debug> defmt::Format for Error<SerialError> {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::Serial(e) => {
                let e = defmt::Debug2Format(&e);
                defmt::write!(fmt, "Communication with serial failed: {}", e)
            }
            Self::Parse(e) => defmt::write!(fmt, "Failed to parse response: {}", &e),
            Self::Overflow => defmt::write!(fmt, "Overflow"),
        }
    }
}

pub struct Lora<Serial> {
    serial: Serial,
}

impl<Serial> Lora<Serial>
where
    Serial: serial::nb::Write<u8> + serial::nb::Read<u8>,
{
    pub fn new(serial: Serial) -> Self {
        Self { serial }
    }

    pub fn into_serial(self) -> Serial {
        self.serial
    }

    /// Drains serial until it hits `\r\n`
    fn drain(&mut self, mut last_byte: Option<u8>) -> Result<(), Error<Serial::Error>> {
        loop {
            let b = nb::block!(self.serial.read()).map_err(Error::Serial)?;
            if last_byte == Some(b'\r') && b == b'\n' {
                break Ok(());
            }
            last_byte = Some(b);
        }
    }

    pub fn send(&mut self, cmd: &[u8]) -> Result<(), Error<Serial::Error>> {
        for &b in cmd {
            nb::block!(self.serial.write(b)).map_err(Error::Serial)?;
        }

        Ok(())
    }

    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, Error<Serial::Error>> {
        let mut i = 0;
        loop {
            let is_overflow = i == buffer.len();
            if is_overflow {
                // ignore error if communication failed
                let _ = self.drain(buffer.last().cloned());
                break Err(Error::Overflow);
            }

            let b = nb::block!(self.serial.read()).map_err(Error::Serial)?;

            buffer[i] = b;
            i += 1;

            let response_end = buffer[..i].ends_with(b"\r\n");
            if response_end {
                break Ok(i);
            }
        }
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParseError {
    Incomplete,
    BadCommand,
    BadErrorCode,
    NoDelimiter,
    NoPrefix,
    NoTerminator,
    UnclosedErrorParen,
    Unknown,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ParseError {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            Self::Incomplete => defmt::write!(fmt, "Incomplete response"),
            Self::BadCommand => defmt::write!(fmt, "Invalid command header"),
            Self::BadErrorCode => defmt::write!(fmt, "Invalid error code"),
            Self::NoDelimiter => defmt::write!(fmt, "Missing the `:` delimiter"),
            Self::NoPrefix => defmt::write!(fmt, "Missing `+` before the header"),
            Self::NoTerminator => defmt::write!(fmt, "Missing message terminator"),
            Self::UnclosedErrorParen => defmt::write!(fmt, "Unclosed error parentheses"),
            Self::Unknown => defmt::write!(fmt, "Unknown"),
        }
    }
}

pub fn parse_response(input: &[u8]) -> Result<Response, ParseError> {
    match parser::response(input) {
        Ok((_i, o)) => Ok(o),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::Incomplete),
    }
}
