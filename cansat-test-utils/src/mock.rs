//! Mock types.

use embedded_hal::{
    nb,
    serial::{nb::Read, ErrorType},
};
use std::convert::Infallible;

/// Mock type implementing `embedded_hal::serial` traits.
pub struct Serial<I> {
    // Data that will be received from uart
    rx_data: I,
}

impl<I> Serial<I> {
    /// Creates an instance of serial mock that will output `data` byte by byte.
    ///
    /// # Examples
    ///
    /// ```
    /// pub use cansat_test_utils::mock::Serial;
    ///
    /// let _ = Serial::new(b"my_data");
    /// let _ = Serial::new(vec![0x00, 0x01]);
    /// ```
    pub fn new(data: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            rx_data: data.into_iter(),
        }
    }
}

impl<I> ErrorType for Serial<I> {
    type Error = Infallible;
}

impl<I: Iterator<Item = u8>> Read for Serial<I> {
    /// Reads a single byte from the serial.
    ///
    /// # Examples
    ///
    /// ```
    /// use cansat_test_utils::mock::Serial;
    /// use embedded_hal::{nb, serial::nb::Read};
    ///
    /// let mut uart = Serial::new([0x00, 0x01]);
    /// assert_eq!(Ok(0x00), uart.read());
    /// assert_eq!(Ok(0x01), uart.read());
    /// assert_eq!(Err(nb::Error::WouldBlock), uart.read());
    /// ```
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.rx_data.next().ok_or(nb::Error::WouldBlock)
    }
}
