//! Platform agnostic library for cansat.
#![deny(unsafe_code)]
#![no_std]

pub mod csv;
pub mod quantity;

use accelerometer::vector;
use csv::Write;
use heapless::Vec;
use quantity::{Angle, Distance, Pressure, Temperature};

const SEA_LVL_PRESSURE: Pressure = Pressure::from_pascals(101300.25);
const GRAVITATIONAL_CONST: f32 = 0.0065;

#[derive(Default)]
pub struct Measurements {
    pub temperature: Option<Temperature>,
    pub pressure: Option<Pressure>,
    pub altitude: Option<Distance>,
    pub nmea: Option<Vec<u8, 82>>,
    pub acceleration: Option<vector::I16x3>,
    pub orientation: Option<accelerometer::Orientation>,
}

#[derive(Debug)]
pub enum Error {
    Overflow,
}

impl Measurements {
    pub fn to_csv_record(&self, output: &mut [u8]) -> Result<usize, Error> {
        let mut writer = csv::Writer::new();
        let mut nwritten = 0;

        let (result, _, n) = self.write(&mut writer, output);
        nwritten += n;

        if result == csv::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }

        let (result, n) = writer.terminator(&mut output[nwritten..]);
        nwritten += n;

        if result == csv::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }

        let (result, n) = writer.finish(&mut output[nwritten..]);
        nwritten += n;

        if result == csv::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }

        Ok(nwritten)
    }
}

// TODO: make it weather dependent
pub fn calculate_altitude(pressure: Pressure) -> Distance {
    let base = pressure / SEA_LVL_PRESSURE;
    let exponent = 1.0 / 5.255;
    Distance::from_meters(44330.0 * (1.0 - (libm::powf(base, exponent))))
}

pub fn calculate_altitude_with_temperature(
    temperature: Temperature,
    pressure: Pressure,
) -> Distance {
    let base = SEA_LVL_PRESSURE / pressure;
    let exponent = 1.0 / 5.257;
    Distance::from_meters(
        ((libm::powf(base, exponent) - 1.0) * temperature.as_kelvins()) / GRAVITATIONAL_CONST,
    )
}

/// Aircraft roll rotation.
///
/// Allows us to determine the position of the forward roll.
/// A positive rolling motion lifts the left wing and lowers the right wing in the aircraft.
/// https://en.wikipedia.org/wiki/Aircraft_principal_axes#/media/File:Yaw_Axis_Corrected.svg
pub fn roll(y: f32, z: f32) -> Angle {
    Angle::from_radians(libm::atan2f(y, z))
}

/// Aircraft pitch rotation.
///
/// Allows us to determine the position of the aircraft nose relative to center of mass.
/// https://en.wikipedia.org/wiki/Aircraft_principal_axes#/media/File:Yaw_Axis_Corrected.svg
pub fn pitch(x: f32, y: f32, z: f32) -> Angle {
    Angle::from_radians(libm::atan2f(-x, libm::sqrtf(y * y + z * z)))
}
