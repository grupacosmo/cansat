//! Platform agnostic library for cansat. It uses traits from [embedded-hal](https://crates.io/crates/embedded-hal) to abstract away hardware components.
#![deny(unsafe_code)]
#![no_std]

use libm::powf;

pub struct Pressure {
    value: f32,
}

impl Pressure {
    pub fn from_pascals(_value: f32) -> Pressure {
        Pressure { value: _value }
    }
    pub fn from_hectos(_value: f32) -> Pressure {
        Pressure { value: _value * 100. }
    }
    pub fn as_pascals(&self) -> f32 {
        self.value
    }
    pub fn as_hectos(&self) -> f32 {
        self.value / 100.
    }
}

// TODO: make it weather dependent
pub fn calculate_altitude(pressure: Pressure) -> f32 {
    let base = pressure.as_pascals() / 101300.; //in Pa
    let power = 1. / 5.255;
    44330. * (1. - (powf(base, power)))
}
