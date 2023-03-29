pub use csv_core::{WriteResult, Writer, WriterBuilder};

use crate::Measurements;

pub trait Write {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize);
}

impl Write for f32 {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        let mut buf = ryu::Buffer::new();
        let f = buf.format(*self);
        writer.field(f.as_bytes(), out)
    }
}

impl Write for f64 {
    fn write(&self, writer: &mut Writer, out: &mut [u8]) -> (WriteResult, usize, usize) {
        let mut buf = ryu::Buffer::new();
        let f = buf.format(*self);
        writer.field(f.as_bytes(), out)
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

        let (r, c, w) = self.temperature.write(writer, out);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(out);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, c, w) = self.pressure.write(writer, out);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(out);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, c, w) = self.altitude.write(writer, out);
        consumed += c;
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        let (r, w) = writer.delimiter(out);
        written += w;
        if let WriteResult::OutputFull = r {
            return (r, consumed, written);
        }

        if let Some(nmea) = &self.nmea {
            let (r, c, w) = writer.field(nmea, out);
            consumed += c;
            written += w;
            if let WriteResult::OutputFull = r {
                return (r, consumed, written);
            }
        }

        // TODO:
        // write acceleration and orientation

        (WriteResult::InputEmpty, consumed, written)
    }
}
