use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::spawn;
use std::time::Duration;

use eframe::egui;
use eframe::egui::Context;
use eframe::epaint;

pub struct MyApp {
    times: u32,
    pub app_receiver: Receiver<bool>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>, ping_receiver: Receiver<bool>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let (app_sender, app_receiver) = channel::<bool>();
        let ectx = cc.egui_ctx.clone();
        let app = MyApp {
            times: 4,
            app_receiver,
        };
        spawn(move || netloop(ping_receiver, ectx, app_sender));
        app
    }
}

fn netloop(ping_receiver: Receiver<bool>, ectx: Context, send: Sender<bool>) {
    loop {
        match ping_receiver.try_recv() {
            Ok(result) => {
                send.send(result).unwrap();
            }
            Err(_) => {}
        }
        ectx.request_repaint();
        thread::sleep(Duration::from_millis(100));
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.app_receiver.try_recv() {
                Ok(result) => match result { true => self.times = 0,
                    false => {}
                },
                Err(_) => {}
            }
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
            self.times += 2;
        });
    }
}
