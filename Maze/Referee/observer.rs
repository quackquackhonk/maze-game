use std::{
    collections::VecDeque,
    fs::File,
    sync::{Arc, Mutex},
};

use common::{
    board::Slide,
    gem::GEM_IMGS,
    grid::Grid as CGrid,
    tile::{CompassDirection, ConnectorShape, PathOrientation, Tile},
    Color, State,
};
use egui::{Align, Color32, Grid, Image, Label, Layout, RichText, Ui, Vec2};
use egui_extras::RetainedImage;

use lazy_static::lazy_static;

use crate::json::JsonRefereeState;
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
const CELL_SIZE_2D: Vec2 = Vec2::new(CELL_SIZE, CELL_SIZE);

/// struct for holding information about a Tile that's being rendered
/// `home_colors` is a vector of all the colors of homes on this tile
/// `player_colors` is a vector of all the colors of players on this tile
#[derive(Debug, Clone)]
struct TileWidget {
    tile: Tile,
    home_colors: Vec<Color>,
    player_colors: Vec<Color>,
}

fn retained_img_to_image(from: &RetainedImage, ui: &Ui) -> Image {
    Image::new(from.texture_id(ui.ctx()), from.size_vec2())
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

    fn center_image(&self, ui: &Ui) -> Image {
        retained_img_to_image(self.center_ret_img(), ui)
            .rotate(self.center_img_rotation().to_radians(), Vec2::splat(0.5))
    }

    fn center_ret_img(&self) -> &RetainedImage {
        match self.tile.connector {
            ConnectorShape::Path(_) => &*PATH_IMG,
            ConnectorShape::Corner(_) => &*CORNER_IMG,
            ConnectorShape::Fork(_) => &*FORK_IMG,
            ConnectorShape::Crossroads => &*CROSSROADS_IMG,
        }
    }

    fn center_img_rotation(&self) -> f32 {
        match self.tile.connector {
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
        }
    }

    fn render_homes(&self, ui: &mut egui::Ui, id: &str) {
        let home_img = &HOME_IMG;
        Grid::new(format!("{} homes", id))
            .min_col_width(0.0)
            .min_row_height(0.0)
            .spacing(Vec2::new(0.0, 0.0))
            .show(ui, |ui| {
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
    }

    fn render_players(&self, ui: &mut egui::Ui, id: &str) {
        let player_img = &PLAYER_IMG;
        Grid::new(format!("{} players", id))
            .min_col_width(0.0)
            .min_row_height(0.0)
            .spacing(Vec2::new(0.0, 0.0))
            .show(ui, |ui| {
                self.player_colors
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
    }

    fn gem_images(&self, ui: &Ui) -> (Image, Image) {
        (
            Image::new(
                GEM_IMGS[&self.tile.gems.0].texture_id(ui.ctx()),
                Vec2::new(CELL_SIZE * 0.8, CELL_SIZE * 0.8),
            ),
            Image::new(
                GEM_IMGS[&self.tile.gems.1].texture_id(ui.ctx()),
                Vec2::new(CELL_SIZE * 0.8, CELL_SIZE * 0.8),
            ),
        )
    }
}

fn render_tile(ui: &mut egui::Ui, widget: TileWidget, id: &str) {
    let center_img = widget.center_image(ui);

    let west_path = retained_img_to_image(widget.west_path(), ui)
        .rotate(90.0_f32.to_radians(), Vec2::splat(0.5));
    let east_path = retained_img_to_image(widget.east_path(), ui)
        .rotate(90.0_f32.to_radians(), Vec2::splat(0.5));

    let (gem1_img, gem2_img) = widget.gem_images(ui);
    // creates player grid
    // creates main grid for the tile
    Grid::new(format!("{} main", id))
        .min_col_width(0.0)
        .spacing(Vec2::new(0.0, 0.0))
        .show(ui, |ui| {
            ui.add_sized(CELL_SIZE_2D, gem1_img);
            widget.north_path().show(ui);
            widget.render_homes(ui, id);
            ui.end_row();

            ui.add(west_path);
            ui.add(center_img);
            ui.add(east_path);
            ui.end_row();

            widget.render_players(ui, id);
            widget.south_path().show(ui);
            ui.add_sized(CELL_SIZE_2D, gem2_img);
        });
}

// Render's the `board` inside of a state
fn render_board(ui: &mut egui::Ui, state: &State) {
    // create a `common::Grid` of `TileWidget`s
    let mut tile_widget_grid: CGrid<TileWidget> = state
        .board
        .grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| TileWidget {
                    tile: tile.clone(),
                    player_colors: vec![],
                    home_colors: vec![],
                })
                .collect::<Box<[TileWidget]>>()
        })
        .collect::<Box<[_]>>()
        .into();

    // update `TileWidget` to include player home and goal information
    state.player_info.iter().for_each(|pi| {
        tile_widget_grid[pi.position]
            .player_colors
            .push(pi.color.clone());
        tile_widget_grid[pi.home].home_colors.push(pi.color.clone());
    });

    // create board grid
    Grid::new("board_grid")
        .spacing(Vec2::new(0.0, 0.0))
        .min_col_width(0.0)
        .min_row_height(0.0)
        .show(ui, |ui| {
            tile_widget_grid
                .iter()
                .enumerate()
                .fold((), |_, (row_idx, row)| {
                    row.iter().enumerate().fold((), |_, (col_idx, tile)| {
                        render_tile(ui, tile.clone(), &format!("({}, {})", col_idx, row_idx))
                    });
                    ui.end_row();
                })
        });
}

/// Renders the given `Slide` as a label
fn render_slide(ui: &mut egui::Ui, state: &State) {
    let slide_text = match state.previous_slide {
        None => RichText::new("No Last Slide").strong(),
        Some(Slide {
            index,
            direction: CompassDirection::North,
        }) => RichText::new(format!("Column {} Up", index)).strong(),
        Some(Slide {
            index,
            direction: CompassDirection::South,
        }) => RichText::new(format!("Column {} Down", index)).strong(),
        Some(Slide {
            index,
            direction: CompassDirection::East,
        }) => RichText::new(format!("Row {} Right", index)).strong(),
        Some(Slide {
            index,
            direction: CompassDirection::West,
        }) => RichText::new(format!("Row {} Left", index)).strong(),
    };
    ui.label(slide_text);
}

/// Renders the spare tile and the last slide onto the `ui`
fn render_state_info(ui: &mut egui::Ui, state: &State) {
    let spare_tile_widget = TileWidget {
        tile: state.board.extra.clone(),
        player_colors: vec![],
        home_colors: vec![],
    };

    let spare_text = RichText::new("Spare Tile:").heading().strong();
    let last_text = RichText::new("Last Slide:").heading().strong();

    ui.vertical_centered(|ui| {
        ui.add_space(CELL_SIZE * 3.0);
        ui.label(spare_text);
        render_tile(ui, spare_tile_widget, "spare");
        ui.add_space(CELL_SIZE * 3.0);
        ui.label(last_text);
        render_slide(ui, state);
    });
}

fn render_state(ui: &mut egui::Ui, state: &State) {
    // create grid for the state
    Grid::new("state_grid")
        .spacing(Vec2::new(25.0, 0.0))
        .show(ui, |ui| {
            render_board(ui, state);
            ui.vertical(|ui| render_state_info(ui, state));
        });
}

pub trait Observer {
    /// Recieves a state from the referee to render
    fn recieve_state(&mut self, state: State);

    /// Indicates to the Observer that the game has ended and no more `State`s will be sent
    fn game_over(&mut self);
}

/// Contains all information needed for an ObserverGUI to render the game
#[derive(Debug, Default, Clone)]
pub struct ObserverGUI {
    states: Arc<Mutex<VecDeque<State>>>,
    game_over: Arc<Mutex<bool>>,
}

impl Observer for ObserverGUI {
    fn recieve_state(&mut self, state: State) {
        self.states.lock().unwrap().push_back(state);
    }

    fn game_over(&mut self) {
        *self.game_over.lock().unwrap() = true;
    }
}

impl eframe::App for ObserverGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut states = self.states.lock().unwrap();
            if !states.is_empty() {
                render_state(ui, &states[0]);
            }

            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if states.len() > 1 {
                    if ui.button("Next").clicked() {
                        states.pop_front();
                    }
                } else {
                    ui.label("No more states to render!");
                };

                if !states.is_empty() && ui.button("Save").clicked() {
                    let path = std::env::current_dir().unwrap();
                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(&path)
                        .add_filter("json", &[".json"])
                        .set_file_name("state.json")
                        .save_file()
                    {
                        let jrs: JsonRefereeState = states[0].clone().into();
                        serde_json::to_writer_pretty(File::create(path).unwrap(), &jrs)
                            .expect("Writing to json failed!");
                    };
                }
                drop(states);
            });
        });
    }
}
