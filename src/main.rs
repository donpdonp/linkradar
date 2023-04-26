mod app;

use eframe::egui;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(40.0, 440.0)),
        ..Default::default()
    };
    eframe::run_native(
        "LM",
        options,
        Box::new(|cc| Box::new(app::MyApp::new(cc))),
    )
}

