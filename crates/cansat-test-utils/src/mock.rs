//! Mock types.
//!
//! Mock objects are simulated objects that mimic the behaviour of real objects in controlled ways, as part of a software testing initiative.

use embedded_hal::{nb, serial};
use std::convert::Infallible;

/// Mock type implementing `embedded_hal::serial` traits.
pub struct Serial<I> {
    // Data that will be sent by serial on `read()`
    pub tx_data: I,
    // Buffer for data received on serial on `write()`
    pub rx_data: Vec<u8>,
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
            tx_data: data.into_iter(),
            rx_data: vec![],
        }
    }
}

impl<I> serial::ErrorType for Serial<I> {
    type Error = Infallible;
}

impl<I: ExactSizeIterator<Item = u8>> serial::nb::Read for Serial<I> {
    /// Reads a single byte from the serial.
    ///
    /// # Examples
    ///
    /// ```
    /// use cansat_test_utils::mock::Serial;
    /// use embedded_hal::{nb, serial::nb::Read};
    ///
    /// let mut serial = Serial::new([0x00, 0x01]);
    /// assert_eq!(serial.read(), Ok(0x00));
    /// assert_eq!(serial.read(), Ok(0x01));
    /// assert_eq!(serial.read(), Err(nb::Error::WouldBlock));
    /// ```
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.tx_data.next().ok_or(nb::Error::WouldBlock)
    }
}

impl<I> serial::nb::Write for Serial<I> {
    /// Write a single byte to the serial.
    ///
    /// # Examples
    ///
    /// ```
    /// use cansat_test_utils::mock::Serial;
    /// use embedded_hal::{nb, serial::nb::Read};
    ///
    /// let mut serial = Serial::new([]);
    /// assert_eq!(serial.write(0x12), Ok());
    /// assert_eq!(serial.tx_data, 0x12);
    /// ```
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.rx_data.push(word);
        Ok(())
    }

    /// Does nothing
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}
