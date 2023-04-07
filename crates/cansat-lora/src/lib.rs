#![no_std]

use embedded_hal::{self, delay, nb, serial};

pub enum Error<SerialError>
where
    SerialError: serial::Error,
{
    Serial(SerialError),
    Delay,
    Overflow(u8),
}

pub struct Lora<Serial> {
    serial: Serial,
}

impl<Serial> Lora<Serial>
where
    Serial: serial::nb::Write<u8> + serial::nb::Read<u8>,
{
    pub fn new(serial: Serial) -> Self {
        Self { serial }
    }

    pub fn send<D>(
        &mut self,
        cmd: &[u8],
        response_buffer: &mut [u8],
        delay: &mut D,
    ) -> Result<usize, Error<Serial::Error>>
    where
        D: delay::blocking::DelayUs,
    {
        for &b in cmd {
            nb::block!(self.serial.write(b)).map_err(Error::Serial)?;
        }

        delay.delay_ms(20).map_err(|_| Error::Delay)?;
        
        let mut resp_ptr = 0;
        loop {
            let b = nb::block!(self.serial.read()).map_err(Error::Serial)?;

            response_buffer[resp_ptr] = b;
            resp_ptr += 1;

            let response_end = response_buffer.ends_with(b"\r\n");
            if response_end {
                break;
            }
        }

        Ok(resp_ptr)
    }
}
