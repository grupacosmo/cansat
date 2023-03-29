//! Platform agnostic library for cansat.
#![deny(unsafe_code)]
#![no_std]

pub mod quantity;

use quantity::{Angle, Distance, Pressure, Temperature};

const SEA_LVL_PRESSURE: Pressure = Pressure::from_pascals(101300.25);
const GRAVITATIONAL_CONST: f32 = 0.0065;

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
