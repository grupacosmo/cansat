#![no_std]

use embedded_hal::{self, delay, nb, serial};

pub enum Error<SerialError>
where
    SerialError: serial::Error,
{
    Delay,
    ResponseError,
    Serial(SerialError),
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

    fn write_all(&mut self, cmd: &[u8]) -> Result<(), Error<Serial::Error>> {
        for &b in cmd {
            nb::block!(self.serial.write(b)).map_err(Error::Serial)?;
        }

        Ok(())
    }

    fn read_all(&mut self, buffer: &mut [u8]) -> Result<usize, Error<Serial::Error>> {
        let mut ptr = 0;

        loop {
            let b = nb::block!(self.serial.read()).map_err(Error::Serial)?;

            buffer[ptr] = b;
            ptr = (ptr + 1) % buffer.len();

            let response_end = buffer.ends_with(b"\r\n");
            if response_end {
                break;
            }
        }

        Ok(ptr)
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
        self.write_all(cmd)?;
        delay.delay_ms(20).map_err(|_| Error::Delay)?;
        let reps_len = self.read_all(response_buffer)?;

        if response_buffer.starts_with(b"ERR") {
            return Err(Error::ResponseError);
        }

        Ok(reps_len)
    }
}
