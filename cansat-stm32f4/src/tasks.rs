use crate::app;
use accelerometer::RawAccelerometer;
use cansat_core::{
    quantity::{Pressure, Temperature},
    Measurements,
};
use rtic::Mutex;
use stm32f4xx_hal::prelude::*;

pub fn idle(mut ctx: app::idle::Context) -> ! {
    loop {
        let measurements = read_measurements(&mut ctx);

        let mut buf = [0; 1024];

        let nwritten = match measurements.to_csv_record(&mut buf) {
            Ok(record) => record,
            Err(_) => {
                defmt::error!("CSV buffer overflow");
                continue;
            }
        };

        let csv_record = &buf[..nwritten];

        let sd_logger = &mut ctx.local.sd_logger;
        sd_logger.write(csv_record).unwrap();
    }
}

fn read_measurements(ctx: &mut app::idle::Context) -> Measurements {
    let i2c1_devices = &mut ctx.local.i2c1_devices;
    let delay = &mut ctx.local.delay;
    let gps = &mut ctx.shared.gps;
    let tracker = &mut ctx.local.tracker;

    let mut data = Measurements::default();

    match i2c1_devices.bme280.measure(delay) {
        Ok(m) => {
            let temperature = Temperature::from_celsius(m.temperature);
            let pressure = Pressure::from_pascals(m.pressure);
            let altitude = cansat_core::calculate_altitude(Pressure::from_pascals(m.pressure));

            defmt::info!(
                "Temperature: {}°C\r\nPressure: {}hPa\n\rAltitude: {}km",
                temperature.as_celsius(),
                pressure.as_hectos(),
                altitude.as_kilos()
            );

            data.temperature = Some(temperature);
            data.pressure = Some(pressure);
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
