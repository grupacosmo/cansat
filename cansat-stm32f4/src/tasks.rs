use crate::app;
use cansat_core::unit::Pressure;
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
                let altitude = cansat_core::calculate_altitude(Pressure::from_pascals(m.pressure));
                defmt::info!("Altitude = {} meters above sea level", altitude);
                defmt::info!("Relative Humidity = {}%", m.humidity);
                defmt::info!("Temperature = {} deg C", m.temperature);
                defmt::info!("Pressure = {} pascals", m.pressure);
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
    if let Err(e) = gps.lock(|gps| gps.read_uart()) {
        defmt::error!("Failed to read gps uart: {}", defmt::Debug2Format(&e));
    };
}

/// Toggles led every second
pub fn blink(ctx: app::blink::Context) {
    let led = ctx.local.led;
    led.toggle();
    defmt::debug!("Blink");
    app::blink::spawn_after(1.secs()).unwrap();
}
