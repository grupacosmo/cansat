//use core::{ops::Deref, time::Duration};

use crate::{app, error::Error, startup::LoraError};
use cansat_core::{
    nmea::NmeaGga,
    quantity::{Pressure, Temperature},
    Measurements,
};
use cansat_lora::ResponseContent;
use mpu6050::PI;
use rtic::Mutex;
use rtic_monotonics::systick::Systick;
use stm32f4xx_hal::prelude::*;

use libm::{self, atan2f};

//pub fn rotate(x_: f32, y_: f32, alpha: f32) -> vector::F32x2{
//    return vector::F32x2{x: x_*alpha.cos() - y_*alpha.sin(), y: x_*alpha.sin() + y_*alpha.cos()};
//}

const RAD_TO_DEG: f32 = 180.0f32 / PI;
const DEG_TO_RAD: f32 = PI / 180.0f32;

pub fn rotate3d(vec: vector::F32x3, angles: vector::F32x3) -> vector::F32x3 {
    let sin_x = libm::sinf(angles.x);
    let sin_y = libm::sinf(angles.y);
    let sin_z = libm::sinf(angles.z);
    let cos_x = libm::cosf(angles.x);
    let cos_y = libm::cosf(angles.y);
    let cos_z = libm::cosf(angles.z);

    vector::F32x3 {
        x: vec.x * cos_y * cos_z
            + vec.y * (sin_x * sin_y * cos_z - cos_x * sin_y)
            + vec.z * (cos_x * sin_y * cos_z + sin_x * sin_z),
        y: vec.x * cos_y * sin_z
            + vec.y * (sin_x * sin_y * sin_z - cos_x * cos_y)
            + vec.z * (cos_x * sin_y * sin_z + sin_x * cos_z),
        z: vec.x * -sin_y + vec.y * sin_x * cos_y + vec.z * cos_x * cos_y,
    }
}

pub fn is_off_course(angle_x: f32, angle_y: f32) -> bool {
    libm::fabsf(angle_x) > 35.0 * DEG_TO_RAD || libm::fabsf(angle_y) > 35.0 * DEG_TO_RAD
}

pub fn idle(mut ctx: app::idle::Context) -> ! {
    let mut writer = csv_core::WriterBuilder::new()
        .delimiter(b',')
        .quote(b'\'')
        .build();

    let mut angle_x = 0.0f32;
    let mut angle_y = 0.0f32;
    let mut angle_z = 0.0f32;

    let mut error_a_x = 0.0f32;
    let mut error_a_y = 0.0f32;
    let mut error_a_z = 0.0f32;

    let mut error_g_x = 0.0f32;
    let mut error_g_y = 0.0f32;
    let mut error_g_z = 0.0f32;

    defmt::info!("calibration start");
    let calibration_precision: i16 = 100;
    for _i in 0..calibration_precision {
        // calibration
        let measurements = read_measurements(&mut ctx);

        if _i % 10 == 0 {
            defmt::info!("{}/{} measurements", _i, calibration_precision);
        }

        //defmt::info!("{}", measurements);

        error_a_x += measurements.acceleration.unwrap().x;
        error_a_y += measurements.acceleration.unwrap().y;
        error_a_z += measurements.acceleration.unwrap().z;

        error_g_x += measurements.gyro.unwrap().x;
        error_g_y += measurements.gyro.unwrap().y;
        error_g_z += measurements.gyro.unwrap().z;
    }
    error_a_x /= f32::from(calibration_precision);
    error_a_y /= f32::from(calibration_precision);
    error_a_z /= f32::from(calibration_precision);

    error_g_x /= f32::from(calibration_precision);
    error_g_y /= f32::from(calibration_precision);
    error_g_z /= f32::from(calibration_precision);

    defmt::info!("calibration end");
    defmt::info!(
        "error avector: {{{}, {}, {}}}",
        error_a_x,
        error_a_y,
        error_a_z
    );
    defmt::info!(
        "error gvector: {{{}, {}, {}}}",
        error_g_x,
        error_g_y,
        error_g_z
    );

    let mut last_measurement =
        <rtic_monotonics::systick::Systick as rtic_monotonics::Monotonic>::now();

    let mut csv_record: heapless::Vec<u8, 512> = heapless::Vec::<u8, 512>::default();
    let part_size = 128;
    let mut csv_record_parts = csv_record.chunks(part_size);
    let mut loop_counter = 0;
    let loop_counter_max = 16;

    // kalman filter
    // https://github.com/TKJElectronics/KalmanFilter

    let kf_q_angle = 0.001f32;
    let kf_q_bias = 0.003f32;
    let kf_r_measure = 0.03f32;

    let mut kf_angle_x = 0.0f32;
    let mut kf_angle_y = 0.0f32;
    let mut kf_bias_x = 0.0f32;
    let mut kf_bias_y = 0.0f32;
    let mut kf_rate_x = 0.0f32;
    let mut kf_rate_y = 0.0f32;

    let mut kf_err_mat_x_00 = 0.0f32;
    let mut kf_err_mat_x_01 = 0.0f32;
    let mut kf_err_mat_x_10 = 0.0f32;
    let mut kf_err_mat_x_11 = 0.0f32;
    let mut kf_err_mat_y_00 = 0.0f32;
    let mut kf_err_mat_y_01 = 0.0f32;
    let mut kf_err_mat_y_10 = 0.0f32;
    let mut kf_err_mat_y_11 = 0.0f32;

    let mut kf_acc_rot_x = 0.0f32;
    let mut kf_acc_rot_y = 0.0f32;
    let mut kf_gyro_rot_x = 0.0f32;
    let mut kf_gyro_rot_y = 0.0f32;

    let mut takeoff_detection_readings = 0;

    loop {
        let measurements = read_measurements(&mut ctx);
        //defmt::info!("{}", measurements);

        let curr_measurement =
            <rtic_monotonics::systick::Systick as rtic_monotonics::Monotonic>::now();

        let dur = curr_measurement
            .checked_duration_since(last_measurement)
            .unwrap();
        last_measurement = curr_measurement;

        let milliseconds = u16::try_from(dur.to_millis()).unwrap();
        let dt: f32 = f32::from(milliseconds) / 1000.0f32;

        //defmt::info!("since last measurement: {}", dur);

        let Some(acc) = measurements.acceleration else {
            panic!("No acceleration data");
        };
        let Some(gyro) = measurements.gyro else {
            panic!("No gyro data");
        };

        //defmt::info!("acc: {}, {}, {}", acc.x, acc.y, acc.z);

        // rotated measurements for complementary filter
        /*
        let cf_acc_x = -acc.z;
        let cf_acc_y = acc.y;
        let cf_acc_z = acc.x;
        let cf_gyro_x = -gyro.z;
        let cf_gyro_y = gyro.y;
        let cf_gyro_z = gyro.x;
        let cf_gyro_err_x = -error_g_z;
        let cf_gyro_err_y = error_g_y;
        let cf_gyro_err_z = error_g_x;

        let cf_acc_rot_x = libm::atanf(cf_acc_y / libm::sqrtf(cf_acc_x*cf_acc_x + cf_acc_z*cf_acc_z));
        let cf_acc_rot_y = -libm::atanf(cf_acc_x / libm::sqrtf(cf_acc_y*cf_acc_y + cf_acc_z*cf_acc_z));
        const CF_ALPHA: f32 = 0.98f32;

        angle_x = CF_ALPHA*(angle_x + (cf_gyro_x - cf_gyro_err_x*0.)*dt) + (1.0f32 - CF_ALPHA)*cf_acc_rot_x;
        angle_y = CF_ALPHA*(angle_y + (cf_gyro_y - cf_gyro_err_y*0.)*dt) + (1.0f32 - CF_ALPHA)*cf_acc_rot_y;
        angle_z += (cf_gyro_z - cf_gyro_err_z)*dt;
        */

        // kf - kalman filter
        let kf_acc_x = acc.z;
        let kf_acc_y = -acc.y;
        let kf_acc_z = -acc.x;
        let kf_gyro_x = -(gyro.z - error_g_z) * RAD_TO_DEG;
        let kf_gyro_y = (gyro.y - error_g_y) * RAD_TO_DEG;
        let kf_gyro_z = (gyro.x - error_g_x); // * RAD_TO_DEG;

        kf_acc_rot_x =
            libm::atanf(kf_acc_y / libm::sqrtf(kf_acc_x * kf_acc_x + kf_acc_z * kf_acc_z))
                * RAD_TO_DEG;
        kf_acc_rot_y =
            -libm::atanf(kf_acc_x / libm::sqrtf(kf_acc_y * kf_acc_y + kf_acc_z * kf_acc_z))
                * RAD_TO_DEG;

        kf_rate_x = kf_gyro_x - kf_bias_x;
        kf_rate_y = kf_gyro_y - kf_bias_y;

        kf_angle_x += kf_rate_x * dt;
        kf_angle_y += kf_rate_y * dt;

        kf_err_mat_x_00 +=
            dt * (dt * kf_err_mat_x_11 - kf_err_mat_x_01 - kf_err_mat_x_10 + kf_q_angle);
        kf_err_mat_x_01 -= dt * kf_err_mat_x_11;
        kf_err_mat_x_10 -= dt * kf_err_mat_x_11;
        kf_err_mat_x_11 += dt * kf_q_bias;
        kf_err_mat_y_00 +=
            dt * (dt * kf_err_mat_y_11 - kf_err_mat_y_01 - kf_err_mat_y_10 + kf_q_angle);
        kf_err_mat_y_01 -= dt * kf_err_mat_y_11;
        kf_err_mat_y_10 -= dt * kf_err_mat_y_11;
        kf_err_mat_y_11 += dt * kf_q_bias;

        let kf_est_err_x = kf_err_mat_x_00 + kf_r_measure;
        let kf_est_err_y = kf_err_mat_y_00 + kf_r_measure;

        let kf_gain_x_0 = kf_err_mat_x_00 / kf_est_err_x;
        let kf_gain_y_0 = kf_err_mat_y_00 / kf_est_err_y;
        let kf_gain_x_1 = kf_err_mat_x_10 / kf_est_err_x;
        let kf_gain_y_1 = kf_err_mat_y_10 / kf_est_err_y;

        let kf_angle_diff_x = kf_acc_rot_x - kf_angle_x;
        let kf_angle_diff_y = kf_acc_rot_y - kf_angle_y;

        kf_angle_x += kf_gain_x_0 * kf_angle_diff_x;
        kf_bias_x += kf_gain_x_1 * kf_angle_diff_x;
        kf_angle_y += kf_gain_y_0 * kf_angle_diff_y;
        kf_bias_y += kf_gain_y_1 * kf_angle_diff_y;

        let kf_err_mat_x_00_temp = kf_err_mat_x_00;
        let kf_err_mat_x_01_temp = kf_err_mat_x_01;
        let kf_err_mat_y_00_temp = kf_err_mat_y_00;
        let kf_err_mat_y_01_temp = kf_err_mat_y_01;

        kf_err_mat_x_00 -= kf_gain_x_0 * kf_err_mat_x_00_temp;
        kf_err_mat_x_01 -= kf_gain_x_0 * kf_err_mat_x_01_temp;
        kf_err_mat_x_10 -= kf_gain_x_1 * kf_err_mat_x_00_temp;
        kf_err_mat_x_11 -= kf_gain_x_1 * kf_err_mat_x_01_temp;
        kf_err_mat_y_00 -= kf_gain_y_0 * kf_err_mat_y_00_temp;
        kf_err_mat_y_01 -= kf_gain_y_0 * kf_err_mat_y_01_temp;
        kf_err_mat_y_10 -= kf_gain_y_1 * kf_err_mat_y_00_temp;
        kf_err_mat_y_11 -= kf_gain_y_1 * kf_err_mat_y_01_temp;

        angle_x = kf_angle_x * DEG_TO_RAD;
        angle_y = kf_angle_y * DEG_TO_RAD;
        angle_z += (kf_gyro_z) * dt;

        let mut acc_err_vec = vector::F32x3 {
            x: error_a_z,
            y: -error_a_y,
            z: -error_a_x,
        };

        acc_err_vec = rotate3d(
            acc_err_vec,
            vector::F32x3 {
                x: -angle_x,
                y: angle_y,
                z: angle_z,
            },
        );

        let acc_x = kf_acc_x - acc_err_vec.x;
        let acc_y = kf_acc_y - acc_err_vec.y;
        let acc_z = kf_acc_z - acc_err_vec.z;

        defmt::info!("linear acceleration:   {{{}, {}, {}}}", acc_x, acc_y, acc_z);
        defmt::info!(
            "measured acceleration: {{{}, {}, {}}}",
            kf_acc_x,
            kf_acc_y,
            kf_acc_z
        );
        defmt::info!(
            "rotated error vector:  {{{}, {}, {}}}",
            acc_err_vec.x,
            acc_err_vec.y,
            acc_err_vec.z
        );
        defmt::info!(
            "angles in degrees:     {{{}, {}, {}}}",
            angle_x * RAD_TO_DEG,
            angle_y * RAD_TO_DEG,
            angle_z * RAD_TO_DEG
        );

        if takeoff_detection_readings >= 0 {
            if acc_z > 0.0f32 {
                takeoff_detection_readings += 1;
            } else {
                takeoff_detection_readings = 0;
            }

            if takeoff_detection_readings == 20 {
                defmt::info!("Cansat has taken off.");
                //loop{}
                takeoff_detection_readings = -1;
            } else {
                defmt::info!("Cansat has not yet taken off.");
            }
        }

        if is_off_course(angle_x, angle_y) {
            defmt::info!("Cansat is off course");
        } else {
            defmt::info!("Cansat is on course");
        }

        if loop_counter == loop_counter_max {
            loop_counter = 0;
            let part = csv_record_parts.next();

            if Option::is_none(&part) {
                defmt::info!("finished writing");
                csv_record = match serde_csv_core::to_vec(&mut writer, &measurements) {
                    Ok(r) => r,
                    Err(e) => {
                        defmt::error!(
                            "Failed to create csv byte record: {}",
                            defmt::Display2Format(&e)
                        );
                        continue;
                    }
                };
                csv_record_parts = csv_record.chunks(part_size);
            } else {
                //*
                let sd_logger = &mut ctx.local.sd_logger;
                if let Some(sd_logger) = sd_logger {
                    sd_logger.write(part.unwrap()).unwrap();
                }
                //*/
                /*
                ctx.shared.csv_record.lock(|_csv| {
                    *_csv = csv_record;
                    let sd_logger = &mut ctx.local.sd_logger;

                    if let Some(sd_logger) = sd_logger {
                        sd_logger.write(part.unwrap()).unwrap();
                    }
                });
                */
            }
        } else {
            loop_counter += 1;
        }
    }
}

fn read_measurements(ctx: &mut app::idle::Context) -> Measurements {
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

    //defmt::info!("{=[u8]:a}", command);
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
    if let Err(e) = gps.lock(|gps| gps.read_serial()) {
        defmt::error!("Failed to read gps' serial: {}", e);
    };
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
        buzzer.toggle();

        ctx.shared.is_fixed.lock(|f| {
            is_fixed = *f;
        });

        if is_fixed {
            defmt::debug!("Buzz with GPS fix");
            Systick::delay(1.secs()).await;
        } else {
            defmt::debug!("Buzz without GPS fix");
            Systick::delay(3.secs()).await;
        }
    }
}
