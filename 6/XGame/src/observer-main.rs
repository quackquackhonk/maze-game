use std::io::{stdin, stdout};

use egui::Vec2;
use referee::observer::{Observer, ObserverGUI};
use xgames::*;
fn main() {
    let height = 700.0;
    let width = 700.0;
    let observer = ObserverGUI::default();
    let options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(width, height)),
        resizable: false,
        ..Default::default()
    };

    read_and_write_json(
        stdin().lock(),
        &mut stdout().lock(),
        vec![Box::new(observer.clone())],
    );

    eframe::run_native("Observer", options, Box::new(|_cc| Box::new(observer)));
}
