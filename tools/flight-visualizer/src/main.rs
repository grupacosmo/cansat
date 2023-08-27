use flight_visualizer::app::FlightVisualizerApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        maximized: false,
        initial_window_size: Some(egui::Vec2::new(1600.0, 900.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Flight Visualizer",
        options,
        Box::new(|cc| Box::new(FlightVisualizerApp::new(cc))),
    )
}
