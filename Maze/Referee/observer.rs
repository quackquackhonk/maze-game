use common::{
    gem::Gem,
    gem::GEM_IMGS,
    tile::{CompassDirection, ConnectorShape, PathOrientation, Tile},
    Color, ColorName, State,
};
use egui::{Color32, Grid, Image, Vec2, Widget};
use egui_extras::RetainedImage;

use lazy_static::lazy_static;
lazy_static! {
    static ref CROSSROADS_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "crossroads.png",
        include_bytes!("../Resources/connectors/crossroads.png"),
    )
    .unwrap();
    static ref PATH_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "path.png",
        include_bytes!("../Resources/connectors/path.png"),
    )
    .unwrap();
    static ref FORK_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "fork.png",
        include_bytes!("../Resources/connectors/fork.png"),
    )
    .unwrap();
    static ref CORNER_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "corner.png",
        include_bytes!("../Resources/connectors/corner.png"),
    )
    .unwrap();
    static ref EMPTY_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "empty.png",
        include_bytes!("../Resources/connectors/empty.png"),
    )
    .unwrap();
    static ref PLAYER_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "player.png",
        include_bytes!("../Resources/player.png"),
    )
    .unwrap();
    static ref HOME_IMG: RetainedImage = egui_extras::RetainedImage::from_image_bytes(
        "home.png",
        include_bytes!("../Resources/home.png"),
    )
    .unwrap();
}

const CELL_SIZE: f32 = 30.0;

/// struct for holding information about a Tile that's being rendered
/// `home_colors` is a vector of all the colors of homes on this tile
/// `player_colors` is a vector of all the colors of players on this tile
struct TileWidget {
    tile: Tile,
    home_colors: Vec<Color>,
    player_colors: Vec<Color>,
}

impl TileWidget {
    fn north_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::North) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
    fn south_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::South) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
    fn east_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::East) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
    fn west_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::West) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
}

fn render_tile(ui: &mut egui::Ui, widget: TileWidget, id: &str) {
    let connector_img = match widget.tile.connector {
        ConnectorShape::Path(_) => &*PATH_IMG,
        ConnectorShape::Corner(_) => &*CORNER_IMG,
        ConnectorShape::Fork(_) => &*FORK_IMG,
        ConnectorShape::Crossroads => &*CROSSROADS_IMG,
    };

    let connector_rot: f32 = match widget.tile.connector {
        ConnectorShape::Path(PathOrientation::Vertical) => 0.0,
        ConnectorShape::Path(PathOrientation::Horizontal) => 90.0,
        ConnectorShape::Corner(CompassDirection::North) => 0.0,
        ConnectorShape::Corner(CompassDirection::East) => 90.0,
        ConnectorShape::Corner(CompassDirection::South) => 180.0,
        ConnectorShape::Corner(CompassDirection::West) => 270.0,
        ConnectorShape::Fork(CompassDirection::North) => 0.0,
        ConnectorShape::Fork(CompassDirection::East) => 90.0,
        ConnectorShape::Fork(CompassDirection::South) => 180.0,
        ConnectorShape::Fork(CompassDirection::West) => 270.0,
        ConnectorShape::Crossroads => 0.0,
    };

    let west_path = widget.west_path();
    let west_path = egui::Image::new(west_path.texture_id(ui.ctx()), west_path.size_vec2())
        .rotate(90.0_f32.to_radians(), Vec2::splat(0.5));
    let east_path = widget.east_path();
    let east_path = egui::Image::new(east_path.texture_id(ui.ctx()), east_path.size_vec2())
        .rotate(90.0_f32.to_radians(), Vec2::splat(0.5));

    // creates player grid
    let player_img = &PLAYER_IMG;
    let home_img = &HOME_IMG;
    let player_grid = Grid::new(format!("{} players", id))
        .min_col_width(0.0)
        .min_row_height(0.0)
        .spacing(Vec2::new(0.0, 0.0));
    let home_grid = Grid::new(format!("{} homes", id))
        .min_col_width(0.0)
        .min_row_height(0.0)
        .spacing(Vec2::new(0.0, 0.0));

    // creates main grid for the tile
    Grid::new(format!("{} main", id))
        .min_col_width(0.0)
        .spacing(Vec2::new(0.0, 0.0))
        .show(ui, |ui| {
            ui.add(Image::new(
                GEM_IMGS[&widget.tile.gems.0].texture_id(ui.ctx()),
                Vec2::new(CELL_SIZE, CELL_SIZE),
            ));
            widget.north_path().show(ui);
            home_grid.show(ui, |ui| {
                widget
                    .home_colors
                    .iter()
                    .enumerate()
                    .for_each(|(idx, col)| {
                        if idx != 0 && idx % 2 == 0 {
                            ui.end_row();
                        }
                        ui.add(
                            Image::new(home_img.texture_id(ui.ctx()), Vec2::new(15.0, 15.0))
                                .tint(Color32::from_rgb(col.code.0, col.code.1, col.code.2)),
                        );
                    })
            });
            ui.end_row();

            ui.add(west_path);
            ui.add(
                Image::new(
                    connector_img.texture_id(ui.ctx()),
                    connector_img.size_vec2(),
                )
                .rotate(connector_rot.to_radians(), Vec2::splat(0.5)),
            );
            ui.add(east_path);
            ui.end_row();

            player_grid.show(ui, |ui| {
                widget
                    .player_colors
                    .iter()
                    .enumerate()
                    .for_each(|(idx, col)| {
                        if idx != 0 && idx % 2 == 0 {
                            ui.end_row();
                        }
                        ui.add(
                            Image::new(player_img.texture_id(ui.ctx()), Vec2::new(15.0, 15.0))
                                .tint(Color32::from_rgb(col.code.0, col.code.1, col.code.2)),
                        );
                    })
            });
            widget.south_path().show(ui);
            ui.add(Image::new(
                GEM_IMGS[&widget.tile.gems.1].texture_id(ui.ctx()),
                Vec2::new(CELL_SIZE, CELL_SIZE),
            ));
        });

    // ui.add(Image::new(image.texture_id(ui.ctx()), image.size_vec2()));
}

fn render_state(ui: &mut egui::Ui, state: &State) {
    Grid::new("state_grid").show(ui, |ui| {
        state
            .board
            .grid
            .iter()
            .enumerate()
            .for_each(|(row_idx, row)| {
                row.iter().enumerate().for_each(|(col_idx, tile)| {
                    render_tile(
                        ui,
                        TileWidget {
                            tile: tile.clone(),
                            home_colors: vec![],
                            player_colors: vec![],
                        },
                        &format!("({}, {})", col_idx, row_idx),
                    )
                });
                ui.end_row();
            })
    });
}

pub struct Observer;

impl eframe::App for Observer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| render_state(ui, &State::default()));
    }
}
