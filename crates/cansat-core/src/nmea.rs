use nmea::sentences::{FixType, GgaData};
use nmea::ParseResult;
use serde::Serialize;

#[cfg(feature = "defmt")]
use defmt::Format;

#[cfg_attr(feature = "defmt", derive(Format))]
#[derive(Debug)]
pub enum Error<'a> {
    ParsingFailed(#[cfg_attr(feature = "defmt", defmt(Debug2Format))] nmea::Error<'a>),
    InvalidCommand,
}

#[derive(Serialize, Debug)]
pub struct NmeaGga(GgaData);

impl NmeaGga {
    pub fn try_new(bytes: &[u8]) -> Result<NmeaGga, Error> {
        let ParseResult::GGA(gga) = nmea::parse_bytes(bytes).map_err(Error::ParsingFailed)? else {
            return Err(Error::InvalidCommand)
        };

        Ok(Self(gga))
    }

    pub fn get_fix(&self) -> bool {
        self.0
            .fix_type
            .map(|ft| FixType::Invalid != ft)
            .unwrap_or(false)
    }
}

#[cfg(feature = "defmt")]
impl Format for NmeaGga {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "fix_time: {:?}, fix_type: {:?}, latitude: {:?}, 
             fix_satelites: {:?}, hdop: {:?}, altitude: {:?}, 
             geoid_separation: {:?}",
            self.0.fix_time.as_ref().map(defmt::Debug2Format),
            self.0.fix_type.as_ref().map(defmt::Debug2Format),
            self.0.latitude,
            self.0.fix_satellites,
            self.0.hdop,
            self.0.altitude,
            self.0.geoid_separation
        );
    }
}
