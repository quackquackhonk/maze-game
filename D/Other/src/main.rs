use std::io;
use std::io::BufRead;

use eframe::egui;
use egui::{FontId, RichText, Vec2};

#[derive(Default)]
struct Game {
    corners: Vec<String>,
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("main_grid").striped(true).show(ui, |ui| {
                for row in &self.corners {
                    for corner in row.chars() {
                        ui.label(
                            RichText::from(String::from(corner)).font(FontId::monospace(40.0)),
                        );
                    }
                    ui.end_row();
                }
            });
        });

        // mouse input
        let pos = ctx.input().pointer.press_origin();
        if let Some(mouse_pos) = pos {
            println!("[{}, {}]", mouse_pos.x, mouse_pos.y);
            frame.close();
        }
    }
}

fn main() {
    let mut corners: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines().flatten() {
        let mut line = line.trim().chars();
        line.next();
        line.next_back();
        corners.push(line.as_str().to_owned());
    }

    let cell_size = 50.0;
    let height = cell_size * corners.len() as f32;
    let width = cell_size * corners[0].chars().count() as f32;
    let test_game = Game { corners };
    let options = eframe::NativeOptions {
        initial_window_size: Option::from(Vec2::new(width, height)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native("TAHBPL D", options, Box::new(|_cc| Box::new(test_game)));
}
