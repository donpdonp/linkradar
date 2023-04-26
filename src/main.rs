use eframe::egui;
use eframe::epaint;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(40.0, 440.0)),
        ..Default::default()
    };
    eframe::run_native(
        "LM",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    times: u32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
            let win_size = frame.info().window_info.size;
            ui.heading("linkmonitor");
            let circle = epaint::CircleShape {
                center: egui::Pos2 {
                    x: win_size.x/2.0,
                    y: self.times as f32,
                },
                radius: win_size.x*0.2,
                fill: egui::Color32::RED,
                stroke: egui::Stroke::new(4.0, egui::Color32::YELLOW),
            };
            ui.painter().add(egui::Shape::Circle(circle));
            self.times += 1;
        });
    }
}
