/// Ergonomic [`csv_core::Writer`] adapter
#[derive(Default)]
pub struct Writer(pub csv_core::Writer);

#[derive(Debug)]
pub enum Error {
    /// Output buffer overflow
    Overflow,
    /// Byte conversion failure
    Conversion,
}

impl Writer {
    pub fn new() -> Self {
        Self(csv_core::Writer::new())
    }

    pub fn field(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize, Error> {
        let (r, _, w) = self.0.field(input, output);
        if let csv_core::WriteResult::OutputFull = r {
            return Err(Error::Overflow);
        }
        Ok(w)
    }

    pub fn delimiter(&mut self, output: &mut [u8]) -> Result<usize, Error> {
        let (r, w) = self.0.delimiter(output);
        if let csv_core::WriteResult::OutputFull = r {
            return Err(Error::Overflow);
        }
        Ok(w)
    }

    pub fn terminator(&mut self, output: &mut [u8]) -> Result<usize, Error> {
        let (r, w) = self.0.terminator(output);
        if let csv_core::WriteResult::OutputFull = r {
            return Err(Error::Overflow);
        }
        Ok(w)
    }

    pub fn finish(&mut self, output: &mut [u8]) -> Result<usize, Error> {
        let (r, w) = self.0.finish(output);
        if let csv_core::WriteResult::OutputFull = r {
            return Err(Error::Overflow);
        }
        Ok(w)
    }
}

pub fn to_byte_record(v: &impl Write, output: &mut [u8]) -> Result<usize, Error> {
    let mut writer = Writer::new();
    let mut nwritten = 0;

    nwritten += v.write(&mut writer, output)?;
    nwritten += writer.terminator(&mut output[nwritten..])?;
    nwritten += writer.finish(&mut output[nwritten..])?;

    Ok(nwritten)
}

pub trait Write {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> Result<usize, Error>;
}

impl Write for &[u8] {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> Result<usize, Error> {
        writer.field(self, out)
    }
}

impl Write for f32 {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> Result<usize, Error> {
        let mut buf = ryu::Buffer::new();
        let f = buf.format(*self);
        f.as_bytes().write(writer, out)
    }
}

impl<T: Write> Write for Option<T> {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> Result<usize, Error> {
        if let Some(t) = self {
            t.write(writer, out)
        } else {
            Ok(0)
        }
    }
}
