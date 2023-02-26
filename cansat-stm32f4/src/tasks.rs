use super::app::{bme_measure, gps_irq, heartbeat, log_nmea, sdmmc_log};
use cansat_core::Pressure;
use defmt::Debug2Format;
use rtic::Mutex;
use stm32f4xx_hal::prelude::*;

pub fn log_nmea(ctx: log_nmea::Context) {
    let mut gps = ctx.shared.gps;
    let msg = gps.lock(|gps| gps.last_nmea()).unwrap();
    defmt::info!("{=[u8]:a}", &msg);
}

/// USART3 interrupt handler that reads data into the gps working buffer
pub fn gps_irq(ctx: gps_irq::Context) {
    let mut gps = ctx.shared.gps;
    let (_, is_terminator) = gps.lock(|gps| gps.read_uart()).unwrap();
    if is_terminator {
        log_nmea::spawn().unwrap();
    }
}

/// Toggles led every second
pub fn heartbeat(ctx: heartbeat::Context) {
    let led = ctx.local.led;
    led.toggle();
    defmt::debug!("Blink");
    heartbeat::spawn_after(1.secs()).unwrap();
}

pub fn bme_measure(ctx: bme_measure::Context) {
    let bme = ctx.local.bme280;
    let delay = ctx.local.delay;
    let measurements = match bme.measure(delay) {
        Ok(m) => m,
        Err(e) => {
            defmt::error!("Could not read bme280 measurements: {}", Debug2Format(&e));
            return;
        }
    };

    let altitude = cansat_core::calculate_altitude(Pressure::from_pascals(measurements.pressure));
    defmt::info!("Altitude = {} meters above sea level", altitude);
    defmt::info!("Relative Humidity = {}%", measurements.humidity);
    defmt::info!("Temperature = {} deg C", measurements.temperature);
    defmt::info!("Pressure = {} pascals", measurements.pressure);

    bme_measure::spawn_after(5.secs()).unwrap();
}

pub fn sdmmc_log(ctx: sdmmc_log::Context) {
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
