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
    pub fn from_celsius(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kelvin(value: f32) -> Self {
        Self(value - 273.15)
    }

    pub fn as_celsius(&self) -> f32 {
        self.0
    }

    pub fn as_kelvin(&self) -> f32 {
        self.0 + 273.15
    }
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Add, Sub, Mul, Div)]
pub struct Metric(f32);

impl Metric {
    pub fn from_meters(value: f32) -> Self {
        Self(value)
    }

    pub fn from_kilometers(value: f32) -> Self {
        Self(value * 1000.)
    }

    pub fn as_meters(&self) -> f32 {
        self.0
    }

    pub fn as_kilometers(&self) -> f32 {
        self.0 / 1000.
    }
}

impl core::ops::Div for Metric {
  type Output = f32;

  fn div(self, rhs: Self) -> Self::Output {
      self.0 / rhs.0
  }
}