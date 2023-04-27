mod app;
mod pingdb;

use eframe::egui;
use std::sync::mpsc::{channel, Receiver};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    log::info!("linkradar");

    let (app_sender, app_receiver) = channel::<bool>();

    let pingdb = pingdb::Pingdb::new(app_sender);
    apploop(app_receiver)
}

fn apploop(app_receiver: Receiver<bool>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(40.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "LM",
        options,
        Box::new(|cc| Box::new(app::MyApp::new(cc, app_receiver))),
    )
}
