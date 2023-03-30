pub use csv_core::{WriteResult, Writer, WriterBuilder};

use crate::Measurements;

pub trait Write {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize);
}

impl Write for &[u8] {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        writer.field(self, out)
    }
}

impl Write for f32 {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        let mut buf = ryu::Buffer::new();
        let f = buf.format(*self);
        f.as_bytes().write(writer, out)
    }
}

impl Write for f64 {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        let mut buf = ryu::Buffer::new();
        let f = buf.format(*self);
        f.as_bytes().write(writer, out)
    }
}

impl<T: Write> Write for Option<T> {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        if let Some(t) = self {
            t.write(writer, out)
        } else {
            (WriteResult::InputEmpty, 0, 0)
        }
    }
}

impl Write for Measurements {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        let mut consumed = 0;
        let mut written = 0;

        let (r, c, w) = self.temperature.map(|x| x.as_celsius()).write(writer, out);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(&mut out[written..]);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, c, w) = self
            .pressure
            .map(|x| x.as_pascals())
            .write(writer, &mut out[written..]);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(&mut out[written..]);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, c, w) = self
            .altitude
            .map(|x| x.as_meters())
            .write(writer, &mut out[written..]);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(&mut out[written..]);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, c, w) = self.nmea.as_deref().write(writer, &mut out[written..]);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(&mut out[written..]);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        if self.acceleration.is_some() {
            todo!("write acceleration");
        }

        let (r, w) = writer.delimiter(&mut out[written..]);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        if self.orientation.is_some() {
            todo!("write orientation");
        }

        (WriteResult::InputEmpty, consumed, written)
    }
}
