//! Platform agnostic library for cansat.
#![deny(unsafe_code)]
#![no_std]

pub mod quantity;

use quantity::{Pressure, Temperature, Angle};

const SEA_LVL_PRESSURE: Pressure = Pressure::from_pascals(101300.25);
const GRAVITATIONAL_CONST: f32 = 0.0065;

// TODO: make it weather dependent
pub fn calculate_altitude(pressure: Pressure) -> f32 {
    let base = pressure / SEA_LVL_PRESSURE;
    let exponent = 1.0 / 5.255;
    44330.0 * (1.0 - (libm::powf(base, exponent)))
}

pub fn calculate_altitude_with_temperature(temperature: Temperature, pressure: Pressure) -> f32 {
    let base = SEA_LVL_PRESSURE / pressure.as_pascals();
    let exponent = 1. / 5.257;
    ((libm::powf(base.as_pascals(), exponent) - 1.) * temperature.as_kelvins())
        / GRAVITATIONAL_CONST
}

pub fn roll_rotation(y: f32, z: f32) -> Angle {
    Angle::from_radians(libm::atan2f(y, z))
}

pub fn pitch_rotation(x: f32, y: f32, z: f32) -> Angle {
    Angle::from_radians(libm::atan2f(-x, libm::sqrtf(y * y + z * z)))
}
