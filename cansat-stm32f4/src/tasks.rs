use crate::app;
use cansat_core::quantity::{Pressure, Temperature};
use rtic::Mutex;
use stm32f4xx_hal::prelude::*;

pub fn idle(ctx: app::idle::Context) -> ! {
    let bme = ctx.local.bme280;
    let delay = ctx.local.delay;
    let sd_logger = ctx.local.sd_logger;
    let mut gps = ctx.shared.gps;
    loop {
        match bme.measure(delay) {
            Ok(m) => {
                let temperature = Temperature::from_celsius(m.temperature);
                let pressure = Pressure::from_pascals(m.pressure);

                let altitude = cansat_core::calculate_altitude(pressure);
                
                defmt::info!("Altitude = {} meters above sea level", altitude.as_meters());
                defmt::info!("Relative Humidity = {}%", m.humidity);
                defmt::info!("Temperature = {} deg C", temperature.as_celsius());
                defmt::info!("Pressure = {} hPa", pressure.as_hectos());
            }
            Err(e) => {
                defmt::error!(
                    "Could not read bme280 measurements: {}",
                    defmt::Debug2Format(&e)
                );
            }
        };

        if let Some(msg) = gps.lock(|gps| gps.last_nmea()) {
            defmt::info!("{=[u8]:a}", &msg);
            let _ = sd_logger.write(&msg);
        }
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
