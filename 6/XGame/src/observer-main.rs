use std::{
    io::{stdin, stdout},
    thread,
};

use egui::Vec2;
use referee::observer::ObserverGUI;
use xgames::*;

fn main() {
    let height = 700.0;
    let width = 800.0;
    let observer = ObserverGUI::default();
    let options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(width, height)),
        resizable: false,
        ..Default::default()
    };

    let observer_clone = observer.clone();
    let guard = thread::spawn(|| {
        read_and_write_json(
            stdin().lock(),
            &mut stdout().lock(),
            vec![Box::new(observer_clone)],
        )
        .expect("Test harness failed");
    });

    eframe::run_native("Observer", options, Box::new(move |_cc| Box::new(observer)));
    guard.join().unwrap();
}
