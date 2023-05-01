use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::spawn;
use std::time::Duration;

use eframe::egui;
use eframe::egui::Context;
use eframe::epaint;

pub struct MyApp {
    height: u32,
    momentum: f32,
    pub app_receiver: Receiver<f32>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>, ping_receiver: Receiver<f32>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let (app_sender, app_receiver) = channel::<f32>();
        let ectx = cc.egui_ctx.clone();
        let app = MyApp {
            height: 0,
            momentum: 0.0,
            app_receiver,
        };
        spawn(move || netloop(ping_receiver, ectx, app_sender));
        app
    }
}

fn netloop(ping_receiver: Receiver<f32>, ectx: Context, send: Sender<f32>) {
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
            let win_size = frame.info().window_info.size;
            //log::info!("height {} momentum {}", self.height, self.momentum);
            match self.app_receiver.try_recv() {
                Ok(result) => self.momentum = ((result * 1000.0) / win_size.y) * win_size.y,
                Err(_) => {}
            }
            self.momentum -= 1.7;

            let delta = self.momentum;
            if delta >= 0.0 {
                self.height += delta as u32
            } else {
                let d = (2.0 * delta).abs() as u32;
                self.height -= d.min(self.height)
            }

            if self.height < 10 {
                self.height = 10;
                self.momentum = 0.0;
            }
            let circle = epaint::CircleShape {
                center: egui::Pos2 {
                    x: win_size.x / 2.0,
                    y: win_size.y - self.height as f32,
                },
                radius: win_size.x * 0.2,
                fill: egui::Color32::RED,
                stroke: egui::Stroke::new(4.0, egui::Color32::YELLOW),
            };
            ui.painter().add(egui::Shape::Circle(circle));
        });
    }
}
