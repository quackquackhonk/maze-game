use std::ops::Deref;

use common::{
    gem::Gem,
    tile::{ConnectorShape::Crossroads, Tile},
};
use egui::{Grid, Widget};

struct TileWidget(Tile);
impl Deref for TileWidget {
    type Target = Tile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Widget for TileWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        Grid::new("tile").show(ui, |ui| {
            ui.label("G");
            ui.label("N");
            ui.label("H");
            ui.end_row();

            ui.label("W");
            ui.label("+");
            ui.label("E");
            ui.end_row();

            ui.label("A");
            ui.label("S");
            ui.label("G");
        });
        ui.allocate_response(
            egui::Vec2 { x: 20., y: 20. },
            egui::Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        )
    }
}

pub struct Observer;

impl eframe::App for Observer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(TileWidget(Tile {
                connector: Crossroads,
                gems: (Gem::aplite, Gem::dumortierite).into(),
            }))
        });
    }
}
