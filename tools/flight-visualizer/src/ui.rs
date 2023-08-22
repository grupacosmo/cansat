use crate::data;
use eframe::egui;
use eframe::egui::Context;
use eframe::emath::Align;
use egui::plot::{Line, Plot, PlotPoints};
use egui::{Layout, Ui};

pub fn draw_ui(ctx: &Context, data: &data::Data) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Flight Visualizer");
        ui.separator();
        ui.heading("Data");
        ui.horizontal(|ui| {
            ui.label("Time:");
            ui.label(format!("{}", data.time));
        });
        ui.separator();
        draw_ui_bme(ui, data);

        ui.heading("Controls");
        ui.separator();
        ui.heading("Settings");
    });
}

fn draw_ui_bme(ui: &mut Ui, data: &data::Data) {
    ui.heading("BME");
    ui.separator();
    let component_width = ui.available_width() / 3.2;
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        draw_ui_f64_graph(
            ui,
            "Temperature",
            data.bme.temperature.to_vec(),
            UiF64GraphSettings {
                component_width,
                max_y: 27.0,
                min_y: 29.0,
            },
        );
        ui.separator();
        draw_ui_f64_graph(
            ui,
            "Pressure",
            data.bme.pressure.to_vec(),
            UiF64GraphSettings {
                component_width,
                max_y: 9900.0,
                min_y: 10000.0,
            },
        );
        ui.separator();
        draw_ui_f64_graph(
            ui,
            "Height",
            data.bme.height.to_vec(),
            UiF64GraphSettings {
                component_width,
                max_y: f64::NAN,
                min_y: f64::NAN,
            },
        );
    });
}

fn draw_ui_f64_graph(
    ui: &mut Ui,
    name: &str,
    value: Vec<[f64; 2]>,
    graph_settings: UiF64GraphSettings,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(name);
            let last_value = value.last().map(|v| v[1]).unwrap_or(f64::NAN);
            ui.label(format!("{}", last_value));
        });

        let mut plot = Plot::new(name).width(graph_settings.component_width);

        if !graph_settings.max_y.is_nan() && !graph_settings.min_y.is_nan() {
            plot = plot
                .include_y(graph_settings.min_y)
                .include_y(graph_settings.max_y);
        }

        plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(value)).name(name))
        });
    });
}

struct UiF64GraphSettings {
    component_width: f32,
    max_y: f64,
    min_y: f64,
}
