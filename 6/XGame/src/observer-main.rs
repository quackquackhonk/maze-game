use egui::Vec2;
use referee::observer::Observer;
fn main() {
    let height = 500.0;
    let width = 500.0;
    let observer = Observer;
    let options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(width, height)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native("Observer", options, Box::new(|_cc| Box::new(observer)));
}
