use crate::app;
use cansat_core::unit::Pressure;
use rtic::Mutex;
use stm32f4xx_hal::prelude::*;

pub fn idle(ctx: app::idle::Context) -> ! {
    let bme = ctx.local.bme280;
    let delay = ctx.local.delay;
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

pub fn sdmmc_log(ctx: app::sdmmc_log::Context) {
    let controller = ctx.local.controller;
    let filename = ctx.local.filename;
    let mut volume = match controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
        Ok(volume) => volume,
        Err(e) => defmt::panic!("Failed to get volume: {}", e),
    };

    let root_dir = controller.open_root_dir(&volume).unwrap();

    let mut f = controller
        .open_file_in_dir(
            &mut volume,
            &root_dir,
            filename,
            embedded_sdmmc::Mode::ReadWriteCreateOrAppend,
        )
        .unwrap();
    let num_written = controller
        .write(&mut volume, &mut f, "test".as_bytes())
        .unwrap();
    defmt::info!("Written: {} bytes\n", num_written);
    if let Err(e) = controller.close_file(&volume, f) {
        defmt::panic!("Failed to close file: {}", e);
    }
}
