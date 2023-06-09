use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::spawn;
use std::time::Duration;

use fastping_rs::{PingResult, Pinger};

pub struct Pingdb {}

impl Pingdb {
    pub(crate) fn new(app_sender: Sender<f32>) -> Self {
        let (pinger, results) = match Pinger::new(None, Some(56)) {
            Ok((pinger, results)) => (pinger, results),
            Err(e) => panic!("Error creating pinger: {}", e),
        };
        pinger.add_ipaddr("4.2.2.2");
        pinger.run_pinger();

        spawn(move || netloop(results, app_sender));

        Pingdb {}
    }
}

fn netloop(results: Receiver<PingResult>, send: Sender<f32>) {
    loop {
        let mut speed = 0.0;
        match results.recv() {
            Ok(result) => match result {
                PingResult::Idle { addr } => {
                    //log::error!("Idle Address {}.", addr);
                }
                PingResult::Receive { addr, rtt } => {
                    log::info!("Receive from Address {} in {:?}.", addr, rtt);
                    speed = rtt.as_secs_f32().abs();
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
        send.send(speed).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
