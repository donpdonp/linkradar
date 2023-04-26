use eframe::egui;
use eframe::egui::ImageData::Color;
use eframe::egui::Shape::Circle;
use eframe::egui::{Color32, Pos2, Shape, Stroke};
use eframe::epaint::CircleShape;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    times: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        MyApp { times: 4 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let circle = CircleShape {
                center: Pos2 {
                    x: self.times as f32,
                    y: 0.0,
                },
                radius: 40.0,
                fill: Color32::RED,
                stroke: Stroke::new(14.0, Color32::YELLOW),
            };
            ui.painter().add(Shape::Circle(circle));
            self.times += 1;
        });
    }
}
