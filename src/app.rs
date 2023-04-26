use eframe::egui;
use eframe::epaint;
use fastping_rs::{PingResult, Pinger};
use log;
use std::thread;
use std::thread::spawn;
use std::time::Duration;

pub struct MyApp {
    times: u32,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (pinger, results) = match Pinger::new(None, Some(56)) {
            Ok((pinger, results)) => (pinger, results),
            Err(e) => panic!("Error creating pinger: {}", e),
        };
        pinger.add_ipaddr("4.2.2.2");
        pinger.run_pinger();

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let ectx = cc.egui_ctx.clone();

        spawn(move || loop {
            match results.recv() {
                Ok(result) => match result {
                    PingResult::Idle { addr } => {
                        log::error!("Idle Address {}.", addr);
                    }
                    PingResult::Receive { addr, rtt } => {
                        log::info!("Receive from Address {} in {:?}.", addr, rtt);
                    }
                },
                Err(_) => panic!("Worker threads disconnected before the solution was found!"),
            }

            ectx.request_repaint();
            thread::sleep(Duration::from_secs(1));
        });

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
                    x: win_size.x / 2.0,
                    y: self.times as f32,
                },
                radius: win_size.x * 0.2,
                fill: egui::Color32::RED,
                stroke: egui::Stroke::new(4.0, egui::Color32::YELLOW),
            };
            ui.painter().add(egui::Shape::Circle(circle));
            self.times += 1;
        });
    }
}
