//! Gps device driver.
//!
//! It implements double buffering to ensure that you can always read latest message.
#![deny(unsafe_code)]
#![no_std]

use core::fmt::Debug;
use embedded_hal::{
    nb,
    serial::{self, nb::Read},
};
use heapless::Vec;

pub struct Gps<Uart> {
    uart: Uart,
    bufs: [Vec<u8, 128>; 2],
    current_buf_idx: usize,
    has_nmea: bool,
}

#[derive(Debug)]
pub enum Error<UartError>
where
    UartError: serial::Error,
{
    Uart(UartError),
    Overflow(u8),
}

impl<Uart> Gps<Uart> {
    pub fn new(uart: Uart) -> Self {
        Self {
            uart,
            bufs: Default::default(),
            current_buf_idx: 0,
            has_nmea: false,
        }
    }

    /// Reads last received NMEA message.
    pub fn last_nmea(&self) -> Option<Vec<u8, 128>> {
        self.has_nmea
            .then(|| self.bufs[self.current_buf_idx ^ 1].clone())
    }
}

impl<Uart> Gps<Uart>
where
    Uart: Read,
{
    /// Reads a single character from UART and stores it in an internal buffer.
    /// On success, returns the read byte and a flag indicating whether a message was terminated.
    pub fn read_uart(&mut self) -> Result<(u8, bool), Error<Uart::Error>> {
        let new_b = nb::block!(self.uart.read()).map_err(Error::Uart)?;

        let is_terminated = {
            let current = &mut self.bufs[self.current_buf_idx];
            let last_b = current.last().cloned();
            current.push(new_b).map_err(Error::Overflow)?;
            last_b == Some(b'\r') && new_b == b'\n'
        };

        if is_terminated {
            self.current_buf_idx ^= 1;
            self.bufs[self.current_buf_idx].clear();
            self.has_nmea = true;
        }

        Ok((new_b, is_terminated))
    }
}
