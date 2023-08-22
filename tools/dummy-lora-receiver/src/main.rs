use noise::{NoiseFn, Perlin};

const TEMPERATURE_SEED: f64 = 10000.0;
const PRESSURE_SEED: f64 = 1993.0;
const HEIGHT_SEED: f64 = 42357.0;
const DELAY: u64 = 1000;

const TEMPERATURE_MIN: f64 = 27.0;
const TEMPERATURE_MAX: f64 = 28.0;

const PRESSURE_MIN: f64 = 9900.0;
const PRESSURE_MAX: f64 = 10000.0;

const HEIGHT_MIN_INC: f64 = 0.5;
const HEIGHT_MAX_INC: f64 = 2.0;

fn main() {
    let perlin = Perlin::new(now() as u32);
    let mut height: f64 = 150.0;

    loop {
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
        println!(">|{}|{}|{}|dummy", temperature, pressure, height);

        std::thread::sleep(std::time::Duration::from_millis(DELAY));
    }
}

fn get_val(perlin: &Perlin, time: u64, time_scale: f64, seed: f64, min: f64, max: f64) -> f64 {
    let val = perlin.get([scale(time as f64, time_scale), seed]);

    map(val, -1.0, 1.0, min, max)
}

fn map(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

fn scale(value: f64, scale: f64) -> f64 {
    value * scale
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
