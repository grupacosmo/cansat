use heapless::Vec;
use heapless_bytes::Bytes;
pub use nmea::sentences::{FixType, GgaData};
use nmea::ParseResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct NmeaGga(Bytes<256>);

impl NmeaGga {
    pub fn into_gga(&self) -> Result<GgaData, Error> {
        let parsed = nmea::parse_bytes(self.0.as_slice()).map_err(|_| Error::ParseError)?;

        let ParseResult::GGA(gga) = parsed else {
            return Err(Error::NotGGA);
        };

        Ok(gga)
    }

    pub fn get_fix_type(&self) -> Result<FixType, Error> {
        self.into_gga()?.fix_type.ok_or(Error::FixError)
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError,
    NotGGA,
    FixError,
}

impl From<Vec<u8, 256>> for NmeaGga {
    fn from(value: Vec<u8, 256>) -> Self {
        Self(value.into())
    }
}

struct Ascii<'a>(pub &'a [u8]);

impl<'a> defmt::Format for Ascii<'a> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=[u8]:a}", self.0);
    }
}

impl defmt::Format for NmeaGga {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}", Ascii(self.0.as_ref()));
    }
}
