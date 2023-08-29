use crate::data::{Acceleration, BmeData, Data, DataRecord, Orientation};
use crate::ui;
use cansat_core::Measurements;
use eframe::Frame;
use egui::Context;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
            Err(e) => {
                println!("[STDIN][  ERROR   ]### {}", e);
            }
        }
    }

    fn process_measurements(data_arc: &Arc<Mutex<Data>>, measurements: Measurements) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut data = data_arc.lock().unwrap();

        data.push(DataRecord::new(
            time,
            BmeData::new(
                measurements.temperature.map(|v| v.as_celsius() as f64),
                measurements.pressure.map(|v| v.as_pascals() as f64),
                measurements.altitude.map(|v| v.as_meters() as f64),
            ),
            Orientation::new(
                Some(f64::sin(time / 8.0)),
                Some(f64::sin(time / 15.0)),
                Some(f64::sin(time / 2.0)),
            ),
            Acceleration::new(Some(0.0), Some(0.0), Some(0.0)),
        ));
    }
}
impl eframe::App for FlightVisualizerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::draw_ui(ctx, &self.data.lock().unwrap());
        ctx.request_repaint_after(Duration::from_millis(1000));
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
