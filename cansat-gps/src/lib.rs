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
    buf_1: Vec<u8, 512>,
    buf_2: Vec<u8, 512>,
    last_msg_location: LastMsgLocation,
}

enum LastMsgLocation {
    None,
    Buf1,
    Buf2,
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
            buf_1: Vec::new(),
            buf_2: Vec::new(),
            last_msg_location: LastMsgLocation::None,
        }
    }

    /// Reads last received NMEA message.
    pub fn last_nmea(&self) -> Option<Vec<u8, 512>> {
        match self.last_msg_location {
            LastMsgLocation::Buf1 => Some(self.buf_1.clone()),
            LastMsgLocation::Buf2 => Some(self.buf_2.clone()),
            LastMsgLocation::None => None,
        }
    }
}

impl<Uart> Gps<Uart>
where
    Uart: Read,
{
    /// Reads a single character from UART and stores it in an internal buffer.
    /// On success, returns the read character.
    pub fn read_uart(&mut self) -> Result<u8, Error<Uart::Error>> {
        let b = nb::block!(self.uart.read()).map_err(Error::Uart)?;
        match self.last_msg_location {
            LastMsgLocation::Buf1 => {
                self.buf_2.push(b).map_err(Error::Overflow)?;
                if b == b'\n' {
                    self.last_msg_location = LastMsgLocation::Buf2;
                    self.buf_1.clear();
                }
            }
            LastMsgLocation::Buf2 | LastMsgLocation::None => {
                self.buf_1.push(b).map_err(Error::Overflow)?;
                if b == b'\n' {
                    self.last_msg_location = LastMsgLocation::Buf1;
                    self.buf_2.clear();
                }
            }
        }
        Ok(b)
    }
}
