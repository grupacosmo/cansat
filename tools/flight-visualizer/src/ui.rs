use crate::data::{Data, DataRecord};
use eframe::egui;
use eframe::egui::Context;
use egui::plot::{Line, Plot, PlotPoints, Points};
use egui::{Color32, Pos2, Stroke, Ui, Vec2};
use nalgebra::{Matrix3, Vector2, Vector3};
use once_cell::sync::Lazy;
use time::format_description::FormatItem;
use time::OffsetDateTime;

type DataRecordFieldGetter = fn(&DataRecord) -> Option<f64>;

pub fn draw_ui(ctx: &Context, data: &Data) {
    // ctx.set_debug_on_hover(true);
    egui::CentralPanel::default().show(ctx, |ui| {
        let available_height = ui.available_height();
        ui.horizontal(|ui| {
            ui.set_height(available_height); // why?
            ui_left_panel(ui, data);
            ui.separator();
            ui_right_panel(ui, data);
        });
    });
}

fn ui_left_panel(ui: &mut Ui, data: &Data) {
    let available_width = ui.available_width();
    ui.vertical(|ui| {
        ui.set_width(available_width * 0.33);
        ui.heading("Position Change");

        let full_height = ui.available_height() * 0.95;
        draw_graph(
            ui,
            "height",
            data,
            |d| d.bme.height,
            UiGraphSettings::new_with_height(f64::NAN, f64::NAN, full_height * 0.5),
        );

        draw_graph(
            ui,
            "dZ",
            data,
            |d| d.acceleration.z,
            UiGraphSettings::new_with_height(0.0, 0.0, full_height * 0.25),
        );

        draw_graph(
            ui,
            "dXY",
            data,
            extract_horizontal_velocity,
            UiGraphSettings::new_with_height(0.0, 0.0, full_height * 0.25),
        );
    });
}

fn ui_right_panel(ui: &mut Ui, data: &Data) {
    ui.vertical(|ui| {
        ui.set_width(ui.available_width());
        ui_right_top_info_panel(ui, data);
        ui_3d_panel(ui, data);
        ui_right_bottom_info_panel(ui, data);
    });
}

fn ui_right_top_info_panel(ui: &mut Ui, data: &Data) {
    let available_height = ui.available_height();
    ui.horizontal(|ui| {
        ui.set_height(available_height * 0.25);

        ui_info_box(ui, data);
        ui_orientation_box(ui, data);
    });
}

fn ui_info_box(ui: &mut Ui, data: &Data) {
    let available_width = ui.available_width();
    ui.vertical(|ui| {
        ui.set_width(available_width * 0.5);
        ui.label(format!("{:?}", data.signal_strength()));
        ui.separator();

        let last_data = data.last_data();
        if last_data.is_none() {
            ui.label("No data");
            return;
        }
        let last_data = last_data.unwrap();
        ui.label(format!("Time: {}", timestamp_formatter(last_data.time)));
        ui.label(format!("Temperature: {:?}", last_data.bme.temperature));
        ui.label(format!("Pressure: {:?}", last_data.bme.pressure));
        ui.label(format!("Height: {:?}", last_data.bme.height));
        ui.label(format!("{:?}", last_data.orientation));
        ui.label(format!("{:?}", last_data.acceleration));
        ui.label(format!("Roll {:?}", last_data.rollpitch.roll));
        ui.label(format!("Pitch {:?}", last_data.rollpitch.pitch));
    });
}

fn ui_orientation_box(ui: &mut Ui, data: &Data) {
    ui.vertical(|ui| {
        let single_graph_height = ui.available_height() * 0.33;
        draw_graph(
            ui,
            "x",
            data,
            |d| d.orientation.x,
            UiGraphSettings::new_with_height(-1.0, 1.0, single_graph_height),
        );

        draw_graph(
            ui,
            "y",
            data,
            |d| d.orientation.y,
            UiGraphSettings::new_with_height(-1.0, 1.0, single_graph_height),
        );

        draw_graph(
            ui,
            "z",
            data,
            |d| d.orientation.z,
            UiGraphSettings::new_with_height(-1.0, 1.0, single_graph_height),
        );
    });
}

fn ui_3d_panel(ui: &mut Ui, data: &Data) {
    let h = ui.available_height() * 0.75;

    let (mut response, painter) =
        ui.allocate_painter(Vec2::new(ui.available_width(), h), egui::Sense::click());
    let rect = response.rect;
    painter.rect_filled(rect, 0.0, Color32::from_rgb(0, 0, 0));

    let last_data = data.last_data();
    if last_data.is_none() {
        return;
    }
    let last_data = last_data.unwrap();

    // model
    // x,y,z
    // z-up
    let points = [
        Vector3::new(0.5, 0.5, 2.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
    ]
    .map(|p| p - Vector3::new(0.5, 0.5, 0.5));

    let rot_z = last_data.rollpitch.roll.unwrap_or(0.0);
    let rot_x = last_data.rollpitch.pitch.unwrap_or(0.0) + std::f64::consts::PI / 2.0;
    let rot_y = 0.0f64;

    // transform points
    let r1 = Matrix3::from([
        [rot_z.cos(), -rot_z.sin(), 0.0],
        [rot_z.sin(), rot_z.cos(), 0.0],
        [0.0, 0.0, 1.0],
    ]);
    let r2 = Matrix3::from([
        [1.0, 0.0, 0.0],
        [0.0, rot_x.cos(), -rot_x.sin()],
        [0.0, rot_x.sin(), rot_x.cos()],
    ]);
    let r3 = Matrix3::from([
        [rot_y.cos(), 0.0, rot_y.sin()],
        [0.0, 1.0, 0.0],
        [-rot_y.sin(), 0.0, rot_y.cos()],
    ]);

    // transform 3d -> 2d
    let r = r1 * r2 * r3;
    let offset = Vector3::new(5.0, 0.0, 0.0);
    let scale2d = 150.0;
    let offset2d = Vector2::new(rect.min.x as f64 + 250.0, rect.min.y as f64 + 100.0);
    let points = points.map(|p| r * p + offset);
    let points2d = points.map(|p| (p.yz() / p.x) * scale2d + offset2d);

    // drawing
    let stroke = Stroke::new(1.0, Color32::from_rgb(255, 0, 0));
    for point_a in &points2d {
        for point_b in &points2d {
            if point_a == point_b {
                continue;
            }
            painter.line_segment(
                [
                    Pos2::new(point_a.x as f32, point_a.y as f32),
                    Pos2::new(point_b.x as f32, point_b.y as f32),
                ],
                stroke,
            );
        }
    }

    response.mark_changed();
}

fn ui_right_bottom_info_panel(ui: &mut Ui, data: &Data) {
    let available_height = ui.available_height();
    ui.horizontal(|ui| {
        ui.set_height(available_height);
        let single_graph_width = ui.available_width() * 0.33;
        draw_graph(
            ui,
            "temperature",
            data,
            |d| d.bme.temperature,
            UiGraphSettings::new_with_width(f64::NAN, f64::NAN, single_graph_width),
        );

        draw_graph(
            ui,
            "pressure",
            data,
            |d| d.bme.pressure,
            UiGraphSettings::new_with_width(f64::NAN, f64::NAN, single_graph_width),
        );

        draw_graph(
            ui,
            "height",
            data,
            |d| d.bme.height,
            UiGraphSettings::new_with_width(f64::NAN, f64::NAN, single_graph_width),
        );
    });
}

fn draw_graph(
    ui: &mut Ui,
    name: &str,
    data: &Data,
    field_getter: DataRecordFieldGetter,
    graph_settings: UiGraphSettings,
) {
    ui.vertical(|ui| {
        graph_settings.apply_component_width(ui);
        graph_settings.apply_component_height(ui);

        ui.horizontal(|ui| {
            ui.label(name);
            let value = data.last_data().and_then(field_getter).unwrap_or(f64::NAN);
            ui.label(format!("{:.2}", value));
        });

        let mut plot = Plot::new(name).x_axis_formatter(|value, _| timestamp_formatter(value));

        if !graph_settings.max_y.is_nan() && !graph_settings.min_y.is_nan() {
            plot = plot
                .include_y(graph_settings.min_y)
                .include_y(graph_settings.max_y);
        }

        plot.show(ui, |plot_ui| {
            let value_iter = data
                .data_iter()
                .filter(|d| field_getter(d).is_some())
                .map(|d| data_to_point(d, &field_getter));

            plot_ui.line(Line::new(PlotPoints::from_iter(value_iter)).name(name));
        });
    });
}

fn data_to_point(data: &DataRecord, field_getter: &DataRecordFieldGetter) -> [f64; 2] {
    [data.time, field_getter(data).unwrap()]
}

fn timestamp_formatter(sec: f64) -> String {
    static FORMAT: Lazy<Vec<FormatItem<'_>>> =
        Lazy::new(|| time::format_description::parse("[hour]:[minute]:[second]").unwrap());
    let datetime = OffsetDateTime::from_unix_timestamp(sec as i64).unwrap();

    datetime.format(&FORMAT).unwrap()
}

fn extract_horizontal_velocity(d: &DataRecord) -> Option<f64> {
    match (d.acceleration.x, d.acceleration.y) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}

struct UiGraphSettings {
    component_width: Option<f32>,
    component_height: Option<f32>,
    min_y: f64,
    max_y: f64,
}

impl UiGraphSettings {
    pub fn new_with_width(min_y: f64, max_y: f64, component_width: f32) -> Self {
        Self {
            component_width: Some(component_width),
            component_height: None,
            min_y,
            max_y,
        }
    }

    pub fn new_with_height(min_y: f64, max_y: f64, component_height: f32) -> Self {
        Self {
            component_width: None,
            component_height: Some(component_height),
            min_y,
            max_y,
        }
    }

    pub fn apply_component_width(&self, ui: &mut Ui) {
        if let Some(w) = self.component_width {
            ui.set_width(w);
        }
    }

    pub fn apply_component_height(&self, ui: &mut Ui) {
        if let Some(h) = self.component_height {
            ui.set_height(h);
        }
    }
}
