use crate::{app, error::Error, startup::LoraError};
use cansat_core::{
    nmea::NmeaGga,
    quantity::{Pressure, Temperature},
    Measurements,
};
use cansat_lora::ResponseContent;
use futures::FutureExt as _;
use rtic::Mutex;
use rtic_monotonics::systick::Systick;
use stm32f4xx_hal::prelude::*;

pub async fn measure(mut ctx: app::measure::Context<'_>) {
    loop {
        let future = async {
            let mut writer = csv_core::WriterBuilder::new()
                .delimiter(b',')
                .quote(b'\'')
                .build();

            let measurements = read_measurements(&mut ctx);
            defmt::info!("{}", measurements);

            let csv_record = match serde_csv_core::to_vec(&mut writer, &measurements) {
                Ok(r) => r,
                Err(e) => {
                    defmt::error!(
                        "Failed to create csv byte record: {}",
                        defmt::Display2Format(&e)
                    );
                    return;
                }
            };
            ctx.shared.csv_record.lock(|csv| {
                *csv = csv_record;
                let sd_logger = &mut ctx.local.sd_logger;

                if let Some(sd_logger) = sd_logger {
                    sd_logger.write(csv).unwrap();
                }
            });
            Systick::delay(250.millis()).await;
        };
        // timeout
        futures::select_biased! {
            _ = future.fuse() => {},
            _ = Systick::delay(10.secs()).fuse() => {}
        };
    }
}

fn read_measurements(ctx: &mut app::measure::Context) -> Measurements {
    let i2c1_devices = &mut ctx.local.i2c1_devices;
    let delay = &mut ctx.local.delay;
    let gps = &mut ctx.shared.gps;
    let _tracker = &mut ctx.local.tracker;

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
        let nmea_gga = NmeaGga::try_new(&nmea);

        match nmea_gga {
            Ok(gga) => {
                ctx.shared.is_fixed.lock(|f: &mut bool| *f = gga.get_fix());
                data.nmea = Some(gga);
            }
            Err(e) => {
                defmt::error!(
                    "Could not read NMEA GGA command: {}",
                    defmt::Debug2Format(&e)
                );
            }
        }
    }

    if let Some(mpu) = &mut i2c1_devices.mpu {
        data.rollpitch = mpu.get_acc_angles().ok().map(|v| (v.x, v.y));
        data.gyro = mpu.get_gyro().ok().map(|v| (v.x, v.y, v.z));
        data.acceleration = mpu.get_acc().ok().map(|v| (v.x, v.y, v.z));
    }

    data
}

pub async fn send_meas(ctx: app::send_meas::Context<'_>) {
    let lora = ctx.local.lora;
    let mut csv_record = ctx.shared.csv_record;
    loop {
        csv_record.lock(|csv| {
            if let Some(lora) = lora {
                if !csv.is_empty() {
                    send_lora_package(lora, &csv[..csv.len() - 1]).unwrap();
                }
            }
        });
        Systick::delay(1.secs()).await;
    }
}

fn send_lora_package(lora: &mut crate::Lora, csv: &[u8]) -> Result<(), Error> {
    let mut command: heapless::Vec<u8, 256> = heapless::Vec::new();
    command.extend_from_slice(b"AT+TEST=TXLRSTR, \"").unwrap();
    command.extend_from_slice(csv).unwrap();
    command.extend_from_slice(b"\"\r\n").unwrap();

    let mut response: [u8; 255] = [0; 255];

    defmt::info!("{=[u8]:a}", command);
    lora.send(&command)?;

    for _ in 1..=2 {
        let nread = lora.receive(&mut response)?;

        let response = cansat_lora::parse_response(&response[..nread]).map_err(LoraError::Parse)?;

        if let ResponseContent::Error(ec) = response.content {
            return Err(Error::Response(ec));
        }
    }

    Ok(())
}

/// USART3 interrupt handler that reads data into the gps working buffer
pub fn gps_irq(ctx: app::gps_irq::Context) {
    let mut gps = ctx.shared.gps;
    gps.lock(|gps| gps.read_serial()).unwrap();
}

/// Toggles led every second
pub async fn blink(ctx: app::blink::Context<'_>) {
    let led = ctx.local.led;
    loop {
        led.toggle();
        defmt::debug!("Blink");
        Systick::delay(1.secs()).await;
    }
}

/// Toggle buzzer every second
pub async fn buzz(mut ctx: app::buzz::Context<'_>) {
    let buzzer = ctx.local.buzzer;
    let mut is_fixed = false;

    loop {
        

        ctx.shared.is_fixed.lock(|f| {
            is_fixed = *f;
        });

        if is_fixed {
            buzzer.toggle();
            defmt::debug!("Buzz with GPS fix");
            Systick::delay(50.millis()).await;
            buzzer.toggle();
            Systick::delay(500.millis()).await;
            buzzer.toggle();
            Systick::delay(50.millis()).await;
            buzzer.toggle();
            Systick::delay(2.secs()).await;
        } else {
            defmt::debug!("Buzz without GPS fix");
            buzzer.toggle();
            Systick::delay(500.millis()).await;
            buzzer.toggle();
            Systick::delay(4.secs()).await;
        }
    }
}
