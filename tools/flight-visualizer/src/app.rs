use crate::data::{Acceleration, BmeData, Data, DataRecord, Orientation, RollPitch, SignalStrength};
use crate::ui;
use cansat_core::Measurements;
use eframe::Frame;
use egui::Context;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct FlightVisualizerApp {
    data: Arc<Mutex<Data>>,
}

impl Default for FlightVisualizerApp {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(Data::default())),
        }
    }
}

impl FlightVisualizerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut slf: Self = Default::default();
        slf.spawn_input_consumer();

        slf
    }

    fn spawn_input_consumer(&mut self) {
        let data = Arc::clone(&self.data);
        std::thread::spawn(move || Self::consume_input(data));
    }

    fn consume_input(data_arc: Arc<Mutex<Data>>) {
        for line in std::io::stdin().lines() {
            match line {
                Err(_) => break,
                Ok(line) => Self::process_input_line(&data_arc, line),
            }
        }
    }

    fn process_input_line(data_arc: &Arc<Mutex<Data>>, line: String) {
        println!("[STDIN][PROCESSING]### {}", line);

        let mut reader = serde_csv_core::Reader::<200>::new();
        match reader.deserialize_from_slice::<Measurements>(line.as_bytes()) {
            Ok((measurements, _)) => {
                Self::process_measurements(data_arc, measurements);
            }
            Err(_) => {
                match Self::process_signal_strength(line) {
                    Ok((signal_strength, noise_level)) =>
                        data_arc.lock().unwrap().set_signal_strength(SignalStrength::new(
                            signal_strength,
                            noise_level,
                        )),
                    Err(e) => println!("[STDIN][  ERROR   ]### this is not a measurements or signal strength: {}", e)
                }
            }
        }
    }

    fn process_signal_strength(line: String) -> Result<(i32, i32),String> {
        static SIGNAL_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"Signal strength: (-?[0-9]+) dBm, Noise level: (-?[0-9]+) dB"#).unwrap());

        let captures =
            SIGNAL_REGEX.captures(&line).ok_or("Failed to capture signal strength")?;

        let signal_strength:i32 =
            captures
                .get(1)
                .map(|g| g.as_str().parse())
                .transpose()
                .map_err(|_| "Failed to parse signal strength".to_string())?
                .unwrap();

        let noise_level:i32 =
            captures
                .get(2)
                .map(|g| g.as_str().parse())
                .transpose()
                .map_err(|_| "Failed to parse noise level".to_string())?
                .unwrap();

        Ok((signal_strength, noise_level))
    }

    fn process_measurements(data_arc: &Arc<Mutex<Data>>, measurements: Measurements) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut data = data_arc.lock().unwrap();
        eprintln!("{:?}", measurements);
        data.push(DataRecord::new(
            time,
            BmeData::new(
                measurements.temperature.map(|v| v.as_celsius() as f64),
                measurements.pressure.map(|v| v.as_pascals() as f64),
                measurements.altitude.map(|v| v.as_meters() as f64),
            ),
            measurements.gyro
                .map(|g| Orientation::new_some(g.0 as f64, g.1 as f64, g.2 as f64))
                .unwrap_or(Orientation::new_none()),
            measurements.acceleration
                .map(|a| Acceleration::new_some(a.0 as f64, a.1 as f64, a.2 as f64))
                .unwrap_or(Acceleration::new_none()),
            measurements.rollpitch
                .map(|rp| RollPitch::new_some(rp.0 as f64, rp.1 as f64))
                .unwrap_or(RollPitch::new_none()),
        ));
    }
}
impl eframe::App for FlightVisualizerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::draw_ui(ctx, &self.data.lock().unwrap());
        ctx.request_repaint_after(Duration::from_millis(33));
    }
}

// impl std::ops::Drop for FlightVisualizerApp {
//     fn drop(&mut self) {
//         if let Some(thread) = self.input_processing_thread.take() {
//             // somehow break stdin().lines() loop
//             println!("If program does not stop, please press Ctrl+D to exit");
//             thread.join().unwrap();
//         }
//     }
// }
