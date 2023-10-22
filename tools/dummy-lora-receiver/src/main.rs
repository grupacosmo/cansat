use cansat_core::quantity::{Distance, Pressure, Temperature};
use cansat_core::Measurements;
use noise::{NoiseFn, Perlin};

const TEMPERATURE_SEED: f32 = 10000.0;
const PRESSURE_SEED: f32 = 1993.0;
const HEIGHT_SEED: f32 = 42357.0;
const DELAY: u64 = 1000;

const TEMPERATURE_MIN: f32 = 27.0;
const TEMPERATURE_MAX: f32 = 28.0;

const PRESSURE_MIN: f32 = 9900.0;
const PRESSURE_MAX: f32 = 10000.0;

const HEIGHT_MIN_INC: f32 = 0.5;
const HEIGHT_MAX_INC: f32 = 2.0;

fn main() {
    let perlin = Perlin::new(now() as u32);
    let mut height: f32 = 150.0;

    let mut inc = 0;

    loop {
        inc += 1;
        let time = now();
        let temperature = get_val(
            &perlin,
            time,
            0.0001,
            TEMPERATURE_SEED,
            TEMPERATURE_MIN,
            TEMPERATURE_MAX,
        );
        let pressure = get_val(
            &perlin,
            time,
            0.000001,
            PRESSURE_SEED,
            PRESSURE_MIN,
            PRESSURE_MAX,
        );
        let height_inc = get_val(
            &perlin,
            time,
            0.01,
            HEIGHT_SEED,
            HEIGHT_MIN_INC,
            HEIGHT_MAX_INC,
        );

        height += height_inc;

        let measurements = Measurements {
            temperature: if (inc % 3) == 0 {
                None
            } else {
                Some(Temperature::from_celsius(temperature))
            },
            pressure: if (inc % 4) != 0 {
                None
            } else {
                Some(Pressure::from_pascals(pressure))
            },
            altitude: Some(Distance::from_meters(height)),
            nmea: None,
            acceleration: None,
            gyro: None,
            rollpitch: None,
        };

        // println!(">|{}|{}|{}|dummy", temperature, pressure, height);
        let mut writer = serde_csv_core::Writer::new();

        let mut data = [0u8; 200];
        let len = writer.serialize_to_slice(&measurements, &mut data).unwrap();
        print!(
            "{}",
            &data[..len].iter().map(|b| *b as char).collect::<String>()
        );
        std::thread::sleep(std::time::Duration::from_millis(DELAY));
    }
}

fn get_val(perlin: &Perlin, time: u32, time_scale: f32, seed: f32, min: f32, max: f32) -> f32 {
    let val = perlin.get([scale(time as f32, time_scale) as f64, seed as f64]) as f32;

    map(val, -1.0, 1.0, min, max)
}

fn map(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

fn scale(value: f32, scale: f32) -> f32 {
    value * scale
}

fn now() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}
