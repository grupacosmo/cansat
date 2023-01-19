//! Platform agnostic library for cansat. It uses traits from [embedded-hal](https://crates.io/crates/embedded-hal) to abstract away hardware components.
#![deny(unsafe_code)]
#![no_std]

use libm::powf;

// TODO: make it weather dependent
pub fn calculate_altitude(pressure: f32) -> f32 {
    let base = pressure / 101300.; //in Pa
    let power = 1. / 5.255;
    44330. * (1. - (powf(base, power)))
}