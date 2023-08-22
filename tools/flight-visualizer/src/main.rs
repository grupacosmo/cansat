use flight_visualizer::app::FlightVisualizerApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Flight Visualizer",
        options,
        Box::new(|cc| Box::new(FlightVisualizerApp::new(cc))),
    )
}
