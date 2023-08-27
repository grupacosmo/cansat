use crate::data::{Acceleration, BmeData, Data, DataRecord, Orientation};
use crate::ui;
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
            data: Arc::new(Mutex::new(Data::new())),
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
        if !line.starts_with(">|") {
            println!("[STDIN][ IGNORED ]### {}", line);
            return;
        }
        println!("[STDIN][PROCESSED]### {}", line);

        let line_split: Vec<&str> = line.split('|').collect();
        if line_split.len() < 3 {
            println!("wrong data: {}", line);
            return;
        }
        let temp: f64 = line_split[1].parse().unwrap();
        let pressure: f64 = line_split[2].parse().unwrap();
        let height: f64 = line_split[3].parse().unwrap();

        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut data = data_arc.lock().unwrap();
        data.push(DataRecord::new(
            time,
            BmeData::new(temp, pressure, height),
            Orientation::new(
                f64::sin(time / 8.0),
                f64::sin(time / 15.0),
                f64::sin(time / 2.0),
            ),
            Acceleration::new(0.0, 0.0, 0.0),
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
