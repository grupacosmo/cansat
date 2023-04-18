#![no_std]

mod parse;

use embedded_hal::{self, nb, serial};

#[derive(Debug)]
pub enum Error<SerialError>
where
    SerialError: serial::Error,
{
    Delay,
    Serial(SerialError),
    Parse,
    Overflow,
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

    fn drain(&mut self) -> Result<(), Error<Serial::Error>> {
        loop {
            match self.serial.read() {
                Ok(_) => (),
                Err(nb::Error::WouldBlock) => break,
                Err(nb::Error::Other(e)) => return Err(Error::Serial(e)),
            }
        }
        Ok(())
    }

    fn write_all(&mut self, cmd: &[u8]) -> Result<(), Error<Serial::Error>> {
        for &b in cmd {
            nb::block!(self.serial.write(b)).map_err(Error::Serial)?;
        }

        Ok(())
    }

    fn read_all(&mut self, buffer: &mut [u8]) -> Result<usize, Error<Serial::Error>> {
        let mut i = 0;

        loop {
            let b = nb::block!(self.serial.read()).map_err(Error::Serial)?;

            match buffer.get_mut(i) {
                Some(i) => *i = b,
                None => {
                    self.drain()?;
                    return Err(Error::Overflow);
                }
            }
            i += 1;

            let response_end = buffer[..i].ends_with(b"\r\n");
            if response_end {
                break;
            }
        }

        Ok(i)
    }

    pub fn send(
        &mut self,
        cmd: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, Error<Serial::Error>> {
        self.write_all(cmd)?;
        let reps_len = self.read_all(response_buffer)?;
        Ok(reps_len)
    }
}
