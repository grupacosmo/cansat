use derive_more::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Copy, PartialOrd, Add, Sub, Mul, Div)]
pub struct Pressure(f32);

impl Pressure {
    pub const fn from_pascals(value: f32) -> Self {
        Self(value)
    }

    pub fn from_hectos(value: f32) -> Self {
        Self(value * 100.0)
    }

    pub const fn as_pascals(&self) -> f32 {
        self.0
    }

    pub fn as_hectos(&self) -> f32 {
        self.0 / 100.
    }
}

impl core::ops::Div for Pressure {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Add, Sub, Mul, Div)]
pub struct Temperature(f32);

impl Temperature {
    pub const fn from_celsius(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kelvins(value: f32) -> Self {
        Self(value - 273.15)
    }

    pub const fn as_celsius(&self) -> f32 {
        self.0
    }

    pub fn as_kelvins(&self) -> f32 {
        self.0 + 273.15
    }
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Add, Sub, Mul, Div)]
pub struct Distance(f32);

impl Distance {
    pub const fn from_meters(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kilos(value: f32) -> Self {
        Self(value * 1000.)
    }

    pub const fn as_meters(&self) -> f32 {
        self.0
    }

    pub fn as_kilos(&self) -> f32 {
        self.0 / 1000.
    }
}

impl core::ops::Div for Distance {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}
