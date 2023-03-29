use crate::app;
use accelerometer::{vector, Orientation, RawAccelerometer};
use cansat_core::quantity::Pressure;
use cansat_gps::MAX_NMEA_LEN;
use heapless::Vec;
use rtic::Mutex;
use stm32f4xx_hal::prelude::*;

#[derive(Default)]
struct RecordData {
    temperature: Option<f32>,
    pressure: Option<f32>,
    altitude: Option<f32>,
    nmea: Option<Vec<u8, MAX_NMEA_LEN>>,
    acceleration: Option<vector::I16x3>,
    orientation: Option<Orientation>,
}

pub fn idle(mut ctx: app::idle::Context) -> ! {
    let mut csv_writer = csv_core::WriterBuilder::new().build();
    let mut buf: Vec<u8, 1000> = Vec::new();

    loop {
        let data = read_record_data(&mut ctx);

        write_option_f32(&mut csv_writer, data.temperature, &mut buf);
        csv_writer.delimiter(&mut buf);

        write_option_f32(&mut csv_writer, data.pressure, &mut buf);
        csv_writer.delimiter(&mut buf);

        write_option_f32(&mut csv_writer, data.altitude, &mut buf);
        csv_writer.delimiter(&mut buf);

        if let Some(nmea) = data.nmea {
            csv_writer.field(&nmea, &mut buf);
        }
        csv_writer.delimiter(&mut buf);

        // TODO:
        // write acceleration and orientation

        csv_writer.terminator(&mut buf);
        csv_writer.finish(&mut buf);

        let sd_logger = &mut ctx.local.sd_logger;
        sd_logger.write(&buf).unwrap();
    }
}

fn read_record_data(ctx: &mut app::idle::Context) -> RecordData {
    let i2c1_devices = &mut ctx.local.i2c1_devices;
    let delay = &mut ctx.local.delay;
    let gps = &mut ctx.shared.gps;
    let tracker = &mut ctx.local.tracker;

    let mut data = RecordData::default();

    match i2c1_devices.bme280.measure(delay) {
        Ok(m) => {
            let altitude = cansat_core::calculate_altitude(Pressure::from_pascals(m.pressure));

            defmt::info!(
                "Temperature: {}°C\r\nPressure: {}Pa\n\rAltitude: {}m",
                m.temperature,
                m.pressure,
                altitude
            );

            data.temperature = Some(m.temperature);
            data.pressure = Some(m.pressure);
            data.altitude = Some(altitude);
        }
        Err(e) => {
            defmt::error!(
                "Could not read bme280 measurements: {}",
                defmt::Debug2Format(&e)
            );
        }
    };

    if let Some(nmea) = gps.lock(|gps| gps.last_nmea()) {
        defmt::info!("NMEA: {=[u8]:a}", &nmea);
        data.nmea = Some(nmea);
    }

    match i2c1_devices.lis3dh.accel_raw() {
        Ok(accel) => {
            let orientation = tracker.update(accel);

            defmt::info!(
                "Acceleration: {}, {}, {}\r\nOrientation: {}",
                accel.x,
                accel.y,
                accel.z,
                defmt::Debug2Format(&orientation)
            );

            data.acceleration = Some(accel);
            data.orientation = Some(orientation);
        }
        Err(e) => {
            defmt::error!("Could not read acceleration: {}", defmt::Debug2Format(&e));
        }
    }

    data
}

fn write_option_f32(w: &mut csv_core::Writer, f: Option<f32>, out: &mut [u8]) {
    let mut buf = ryu::Buffer::new();
    if let Some(f) = f {
        let f = buf.format(f);
        w.field(f.as_bytes(), out);
    }
}

/// USART3 interrupt handler that reads data into the gps working buffer
pub fn gps_irq(ctx: app::gps_irq::Context) {
    let mut gps = ctx.shared.gps;
    if let Err(e) = gps.lock(|gps| gps.read_serial()) {
        defmt::error!(
            "Failed to read from gps' serial: {}",
            defmt::Debug2Format(&e)
        );
    };
}

/// Toggles led every second
pub fn blink(ctx: app::blink::Context) {
    let led = ctx.local.led;
    led.toggle();
    defmt::debug!("Blink");
    app::blink::spawn_after(1.secs()).unwrap();
}
