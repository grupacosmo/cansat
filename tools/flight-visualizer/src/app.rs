use crate::data::Data;
use crate::ui;
use eframe::Frame;
use egui::Context;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct FlightVisualizerApp {
    data: Arc<Mutex<Data>>,
    input_processing_thread: Option<std::thread::JoinHandle<()>>,
}

impl Default for FlightVisualizerApp {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(Data::empty())),
            input_processing_thread: None,
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
        let thread = std::thread::spawn(move || Self::consume_input(data));
        self.input_processing_thread = Some(thread);
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
        data.time = time;
        data.bme.temperature.push([time, temp]);
        data.bme.pressure.push([time, pressure]);
        data.bme.height.push([time, height]);
    }
}
impl eframe::App for FlightVisualizerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ui::draw_ui(ctx, &self.data.lock().unwrap());
        ctx.request_repaint_after(Duration::from_millis(33))
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
