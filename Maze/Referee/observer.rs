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

impl Widget for TileWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = egui::vec2(20.0, 20.0);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if response.clicked() {
            println!("click!");
        }

        if ui.is_rect_visible(rect) {
            let connector_img = match self.tile.connector {
                ConnectorShape::Path(_) => &*PATH_IMG,
                ConnectorShape::Corner(_) => &*CORNER_IMG,
                ConnectorShape::Fork(_) => &*FORK_IMG,
                ConnectorShape::Crossroads => &*CROSSROADS_IMG,
            };

            let connector_rot: f32 = match self.tile.connector {
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

            let west_path = self.west_path();
            let west_path = egui::Image::new(west_path.texture_id(ui.ctx()), west_path.size_vec2())
                .rotate(90.0_f32.to_radians(), Vec2::splat(0.5));
            let east_path = self.east_path();
            let east_path = egui::Image::new(east_path.texture_id(ui.ctx()), east_path.size_vec2())
                .rotate(90.0_f32.to_radians(), Vec2::splat(0.5));

            // creates player grid
            let player_img = &PLAYER_IMG;
            let home_img = &HOME_IMG;
            let player_grid = Grid::new("player_grid")
                .min_col_width(0.0)
                .min_row_height(0.0)
                .spacing(Vec2::new(0.0, 0.0));
            let home_grid = Grid::new("home_grid")
                .min_col_width(0.0)
                .min_row_height(0.0)
                .spacing(Vec2::new(0.0, 0.0));

            // creates main grid for the tile
            Grid::new("main_grid")
                .min_col_width(0.0)
                .spacing(Vec2::new(0.0, 0.0))
                .show(ui, |ui| {
                    ui.add(Image::new(
                        GEM_IMGS[&self.tile.gems.0].texture_id(ui.ctx()),
                        Vec2::new(CELL_SIZE, CELL_SIZE),
                    ));
                    self.north_path().show(ui);
                    home_grid.show(ui, |ui| {
                        self.home_colors.iter().enumerate().for_each(|(idx, col)| {
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
                        self.player_colors
                            .iter()
                            .enumerate()
                            .for_each(|(idx, col)| {
                                if idx != 0 && idx % 2 == 0 {
                                    ui.end_row();
                                }
                                ui.add(
                                    Image::new(
                                        player_img.texture_id(ui.ctx()),
                                        Vec2::new(15.0, 15.0),
                                    )
                                    .tint(Color32::from_rgb(col.code.0, col.code.1, col.code.2)),
                                );
                            })
                    });
                    self.south_path().show(ui);
                    ui.add(Image::new(
                        GEM_IMGS[&self.tile.gems.1].texture_id(ui.ctx()),
                        Vec2::new(CELL_SIZE, CELL_SIZE),
                    ));
                });

            // ui.add(Image::new(image.texture_id(ui.ctx()), image.size_vec2()));
        }

        response
    }
}

pub struct BoardWidget {
    state: State,
}

impl Widget for BoardWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}

pub struct Observer;

impl eframe::App for Observer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        use CompassDirection::*;
        use ConnectorShape::*;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(TileWidget {
                tile: Tile {
                    connector: Fork(East),
                    gems: (Gem::bulls_eye, Gem::blue_pear_shape).into(),
                },
                home_colors: vec![
                    ColorName::Red.into(),
                    ColorName::Blue.into(),
                    ColorName::Black.into(),
                    ColorName::Purple.into(),
                ],
                player_colors: vec![
                    ColorName::Red.into(),
                    ColorName::Blue.into(),
                    ColorName::Green.into(),
                    ColorName::White.into(),
                ],
            })
        });
    }
}
