//! Platform agnostic library for cansat. It uses traits from [embedded-hal](https://crates.io/crates/embedded-hal) to abstract away hardware components.
#![deny(unsafe_code)]
#![no_std]

pub mod unit;

use unit::Pressure;

const SEA_LVL_PRESSURE: Pressure = Pressure::from_pascals(101300.0);

// TODO: make it weather dependent
pub fn calculate_altitude(pressure: Pressure) -> f32 {
    let base = pressure / SEA_LVL_PRESSURE;
    let exponent = 1.0 / 5.255;
    44330.0 * (1.0 - (libm::powf(base, exponent)))
}
