mod app;
mod pingdb;

use std::sync::mpsc::{channel, Receiver};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    log::info!("linkradar");

    let (app_sender, app_receiver) = channel::<()>();

    let pingdb = pingdb::Pingdb::new(app_sender);
    apploop(app_receiver)
}

fn apploop(app_receiver: Receiver<()>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(40.0, 440.0)),
        ..Default::default()
    };

    eframe::run_native("LM", options, Box::new(|cc| Box::new(app::MyApp::new(cc, app_receiver))))
}
