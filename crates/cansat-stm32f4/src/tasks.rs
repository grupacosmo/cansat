use crate::app;
use accelerometer::RawAccelerometer;
use cansat_core::{
    nmea::{FixType, NmeaGga},
    quantity::{Pressure, Temperature},
    Measurements,
};
use heapless::Vec;
use rtic::Mutex;
use stm32f4xx_hal::prelude::*;

pub fn idle(mut ctx: app::idle::Context) -> ! {
    let mut writer = csv_core::Writer::new();

    loop {
        let measurements = read_measurements(&mut ctx);

        defmt::info!("{}", measurements);

        let csv_record: Vec<u8, 1024> = match serde_csv_core::to_vec(&mut writer, &measurements) {
            Ok(r) => r,
            Err(e) => {
                defmt::error!(
                    "Failed to create csv byte record: {}",
                    defmt::Display2Format(&e)
                );
                continue;
            }
        };

        let sd_logger = &mut ctx.local.sd_logger;

        if let Some(sd_logger) = sd_logger {
            sd_logger.write(&csv_record).unwrap();
        }
    }
}

fn read_measurements(ctx: &mut app::idle::Context) -> Measurements {
    let i2c1_devices = &mut ctx.local.i2c1_devices;
    let delay = &mut ctx.local.delay;
    let gps = &mut ctx.shared.gps;
    let tracker = &mut ctx.local.tracker;

    let mut data = Measurements::default();

    if let Some(bme280) = &mut i2c1_devices.bme280 {
        match bme280.measure(delay) {
            Ok(m) => {
                let temperature = Temperature::from_celsius(m.temperature);
                let pressure = Pressure::from_pascals(m.pressure);
                let altitude = cansat_core::calculate_altitude(pressure);

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
        }
    }

    if let Some(mut nmea) = gps.lock(|gps| gps.last_nmea()) {
        let clrf_len = 2;
        nmea.truncate(nmea.len().saturating_sub(clrf_len));
        let nmea_gga: NmeaGga = nmea.into();

        match nmea_gga.get_fix_type() {
            Ok(fix_type) if fix_type != FixType::Invalid => {
                ctx.shared.is_fixed.lock(|f: &mut bool| *f = true);
            }
            Ok(_) => {
                defmt::error!("Obtained fix is invalid",);
            }
            Err(e) => {
                defmt::error!("Could not obtain fix: {}", defmt::Debug2Format(&e));
            }
        }

        data.nmea = Some(nmea_gga);
    }

    if let Some(lis3dh) = &mut i2c1_devices.lis3dh {
        match lis3dh.accel_raw() {
            Ok(accel) => {
                let orientation = tracker.update(accel);
                data.acceleration = Some(accel);
                data.orientation = Some(orientation);
            }
            Err(e) => {
                defmt::error!("Could not read acceleration: {}", defmt::Debug2Format(&e));
            }
        }
    }

    data
}

/// USART3 interrupt handler that reads data into the gps working buffer
pub fn gps_irq(ctx: app::gps_irq::Context) {
    let mut gps = ctx.shared.gps;
    if let Err(e) = gps.lock(|gps| gps.read_serial()) {
        defmt::error!("Failed to read gps' serial: {}", e);
    };
}

/// Toggles led every second
pub fn blink(ctx: app::blink::Context) {
    let led = ctx.local.led;
    led.toggle();
    defmt::debug!("Blink");
    app::blink::spawn_after(1.secs()).unwrap();
}

/// Toggle buzzer every second
pub fn buzz(mut ctx: app::buzz::Context) {
    let buzzer = ctx.local.buzzer;
    buzzer.toggle();

    let mut is_fixed = false;

    ctx.shared.is_fixed.lock(|f| {
        is_fixed = *f;
    });

    if is_fixed {
        defmt::debug!("Buzz with fix");
        app::buzz::spawn_after(1.secs()).unwrap();
    } else {
        defmt::debug!("Buzz without fix");
        app::buzz::spawn_after(3.secs()).unwrap();
    }
}
