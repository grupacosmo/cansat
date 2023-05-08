//! Gps device driver using [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits.
#![no_std]

mod double_buf;

use core::fmt::{Debug, Display};
use double_buf::DoubleBuf;
use embedded_hal::{nb, serial};
use heapless::Vec;

/// Maximum length of an NMEA message including $ and [CR][LF].
pub const MAX_NMEA_LEN: usize = 256;

/// Gps driver.
///
/// It implements double buffering to ensure that you can always read the latest message received.
/// `MAX_NMEA_LEN` is a maximum length of an NMEA message including $ and [CR][LF].
///
/// # Examples
/// ```
/// # use cansat_test_utils::mock;
/// # let uart = cansat_test_utils::mock::Serial::new([b'\r', b'\n']);
/// use cansat_gps::Gps;
/// use heapless::Vec;
///
/// let mut gps: Gps<_, 82> = Gps::new(uart);
///
/// let msg = loop {
///     let (b, is_new_msg) = gps.read_serial().unwrap();
///     
///     if is_new_msg {
///         break gps.last_nmea().unwrap();
///     }
/// };
/// ```
pub struct Gps<Serial, const MAX_NMEA_LEN: usize> {
    serial: Serial,
    bufs: DoubleBuf<u8, MAX_NMEA_LEN>,
    has_nmea: bool,
}

#[derive(Debug)]
pub enum Error<SerialError> {
    Serial(SerialError),
    Overflow(u8),
}

impl<SerialError> From<SerialError> for Error<SerialError> {
    fn from(value: SerialError) -> Self {
        Self::Serial(value)
    }
}

impl<SerialError> defmt::Format for Error<SerialError>
where
    SerialError: Display,
{
    fn format(&self, fmt: defmt::Formatter) {
        use defmt::{write, Display2Format};
        match self {
            Self::Serial(e) => write!(fmt, "Serial error: {}", Display2Format(&e)),
            Self::Overflow(e) => write!(fmt, "Buffer overflow: {}", e),
        }
    }
}

impl<Serial, const MAX_NMEA_LEN: usize> Gps<Serial, MAX_NMEA_LEN> {
    pub fn new(serial: Serial) -> Self {
        Self {
            serial,
            bufs: Default::default(),
            has_nmea: false,
        }
    }

    /// Reads last received NMEA message.
    pub fn last_nmea(&self) -> Option<Vec<u8, MAX_NMEA_LEN>> {
        self.has_nmea.then(|| self.bufs.read().clone())
    }
}

impl<Serial, const MAX_NMEA_LEN: usize> Gps<Serial, MAX_NMEA_LEN>
where
    Serial: serial::nb::Read,
{
    /// Reads a single character from serial in a blocking mode and stores it in an internal buffer.
    /// On success, returns the read byte and a flag indicating whether a message was terminated.
    pub fn read_serial(&mut self) -> Result<(u8, bool), Error<Serial::Error>> {
        let b = nb::block!(self.serial.read())?;
        let write_buf = self.bufs.write();
        write_buf.push(b).map_err(Error::Overflow)?;

        let is_terminated = write_buf.ends_with(b"\r\n");

        if is_terminated {
            self.bufs.swap();
            self.bufs.write().clear();
            self.has_nmea = true;
        }

        Ok((b, is_terminated))
    }
}

impl<Serial, const MAX_NMEA_LEN: usize> Gps<Serial, MAX_NMEA_LEN>
where
    Serial: serial::nb::Write,
{
    pub fn send(&mut self, input: &[u8]) -> Result<(), Error<Serial::Error>> {
        for &b in input {
            nb::block!(self.serial.write(b))?;
        }
        Ok(())
    }
}
