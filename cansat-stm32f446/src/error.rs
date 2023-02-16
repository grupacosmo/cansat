//! Error reporting utilities such as [`Report`], [`Error`] and [`WrapErr`].
//!
//! This module is a `nostd` alternative to [eyre](https://docs.rs/eyre/latest/eyre/index.html) crate.
//!
//! # Examples
//! [`WrapErr`] extends the [`Result<T, E>`] type with an [`wrap_err`][WrapErr::wrap_err] method
//! that maps it's `E` type into the [`error::Report`][Report]. `E` has to implement [`Into<error::Error>`] trait.
//! ```
//! use error::WrapErr;
//!
//! fn init() -> Result<(), error::Report> {
//!     let sensor = Sensor::new().wrap_err("Failed to initialize sensor")?;
//!     sensor.measure().wrap_err("Failed to measure using sensor")?;
//!     Ok(())
//! }
//! ```

use stm32f4xx_hal::{i2c, serial::config::InvalidConfig};

/// Extension trait for [`Result`] that maps the error to [`Report`].
///
/// # Examples
/// ```
/// init().wrap_err("Initialization failed")?;
/// ```
pub trait WrapErr<T, E> {
    fn wrap_err(self, description: &'static str) -> Result<T, Report>;
}

impl<T, E: Into<Error>> WrapErr<T, E> for Result<T, E> {
    fn wrap_err(self, description: &'static str) -> Result<T, Report> {
        self.map_err(|e| Report {
            description,
            cause: e.into(),
        })
    }
}

/// Error report consisting of an description and an [`Error`].
#[derive(Debug)]
pub struct Report {
    pub description: &'static str,
    pub cause: Error,
}

impl<E: Into<Error>> From<E> for Report {
    fn from(e: E) -> Self {
        Report {
            description: "",
            cause: e.into(),
        }
    }
}

impl defmt::Format for Report {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(f, "{}\n    Caused by: {}", self.description, self.cause);
    }
}

/// Aggregate for all the possible errors.
#[derive(Debug)]
pub enum Error {
    Bme280(bme280::Error<i2c::Error>),
    SerialConfig(InvalidConfig),
}

impl defmt::Format for Error {
    fn format(&self, f: defmt::Formatter<'_>) {
        match self {
            Error::Bme280(e) => defmt::write!(f, "Bme280 failure - {}", defmt::Debug2Format(e)),
            Error::SerialConfig(_) => defmt::write!(f, "Invalid serial configuration"),
        }
    }
}

impl From<bme280::Error<i2c::Error>> for Error {
    fn from(e: bme280::Error<i2c::Error>) -> Self {
        Error::Bme280(e)
    }
}

impl From<InvalidConfig> for Error {
    fn from(e: InvalidConfig) -> Self {
        Error::SerialConfig(e)
    }
}
