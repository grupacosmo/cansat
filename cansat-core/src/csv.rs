pub use csv_core::{WriteResult, Writer, WriterBuilder};

use crate::Measurements;

pub enum Error {
    Overflow
}

pub fn measurement_to_record(input: &Measurements, output: &mut [u8]) -> Result<usize, Error> {
    let mut writer = Writer::new();
    let mut nwritten = 0;

    let (result, _, n) = write_measurements(&mut writer, input, output);
    nwritten += n;

    if result == WriteResult::OutputFull {
        return Err(Error::Overflow);
    }

    let (result, n) = writer.finish(output);
    nwritten += n;

    if result == WriteResult::OutputFull {
        return Err(Error::Overflow);
    }

    Ok(nwritten)
}

pub fn write_measurements(
    writer: &mut Writer,
    input: &Measurements,
    output: &mut [u8],
) -> (WriteResult, usize, usize) {
    let mut consumed = 0;
    let mut written = 0;

    let (r, c, w) = write_option_f32(writer, input.temperature, output);
    consumed += c;
    written += w;
    if let WriteResult::OutputFull = r {
        return (r, consumed, written);
    }

    let (r, w) = writer.delimiter(output);
    written += w;
    if let WriteResult::OutputFull = r {
        return (r, consumed, written);
    }

    let (r, c, w) = write_option_f32(writer, input.pressure, output);
    consumed += c;
    written += w;
    if let WriteResult::OutputFull = r {
        return (r, consumed, written);
    }

    let (r, w) = writer.delimiter(output);
    written += w;
    if let WriteResult::OutputFull = r {
        return (r, consumed, written);
    }

    let (r, c, w) = write_option_f32(writer, input.altitude, output);
    consumed += c;
    written += w;
    if let WriteResult::OutputFull = r {
        return (r, consumed, written);
    }

    let (r, w) = writer.delimiter(output);
    written += w;
    if let WriteResult::OutputFull = r {
        return (r, consumed, written);
    }

    if let Some(nmea) = &input.nmea {
        let (r, c, w) = writer.field(nmea, output);
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

pub fn write_option_f32(
    w: &mut Writer,
    f: Option<f32>,
    out: &mut [u8],
) -> (WriteResult, usize, usize) {
    let mut buf = ryu::Buffer::new();
    if let Some(f) = f {
        let f = buf.format(f);
        w.field(f.as_bytes(), out)
    } else {
        (WriteResult::InputEmpty, 0, 0)
    }
}
