//! Platform agnostic library for cansat.
#![deny(unsafe_code)]
#![no_std]

pub mod csv;
pub mod quantity;

use accelerometer::vector;
use heapless::Vec;
use quantity::Pressure;

const SEA_LVL_PRESSURE: Pressure = Pressure::from_pascals(101300.0);

#[derive(Default)]
pub struct Measurements {
    pub temperature: Option<f32>,
    pub pressure: Option<f32>,
    pub altitude: Option<f32>,
    pub nmea: Option<Vec<u8, 82>>,
    pub acceleration: Option<vector::I16x3>,
    pub orientation: Option<accelerometer::Orientation>,
}

// TODO: make it weather dependent
pub fn calculate_altitude(pressure: Pressure) -> f32 {
    let base = pressure / SEA_LVL_PRESSURE;
    let exponent = 1.0 / 5.255;
    44330.0 * (1.0 - (libm::powf(base, exponent)))
}
