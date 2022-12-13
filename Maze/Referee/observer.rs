use std::{
    collections::VecDeque,
    fs::File,
    sync::{Arc, Mutex},
};

use common::{
    board::Slide,
    color::Color,
    gem::GEM_IMGS,
    grid::Grid as CGrid,
    state::{FullPlayerInfo, PublicPlayerInfo, State},
    tile::{CompassDirection, ConnectorShape, PathOrientation, Tile},
};
use egui::{Align, Color32, Grid, Image, Layout, RichText, Ui, Vec2};
use egui_extras::RetainedImage;

use lazy_static::lazy_static;

use crate::json::JsonRefereeState;

// static declarations for the pictures of the tiles
lazy_static! {
    static ref CROSSROADS_IMG: RetainedImage = RetainedImage::from_image_bytes(
        "crossroads.png",
        include_bytes!("../Resources/connectors/crossroads.png"),
    )
    .unwrap();
    static ref PATH_IMG: RetainedImage = RetainedImage::from_image_bytes(
        "path.png",
        include_bytes!("../Resources/connectors/path.png"),
    )
    .unwrap();
    static ref FORK_IMG: RetainedImage = RetainedImage::from_image_bytes(
        "fork.png",
        include_bytes!("../Resources/connectors/fork.png"),
    )
    .unwrap();
    static ref CORNER_IMG: RetainedImage = RetainedImage::from_image_bytes(
        "corner.png",
        include_bytes!("../Resources/connectors/corner.png"),
    )
    .unwrap();
    static ref EMPTY_IMG: RetainedImage = RetainedImage::from_image_bytes(
        "empty.png",
        include_bytes!("../Resources/connectors/empty.png"),
    )
    .unwrap();
    static ref PLAYER_IMG: RetainedImage =
        RetainedImage::from_image_bytes("player.png", include_bytes!("../Resources/player.png"),)
            .unwrap();
    static ref HOME_IMG: RetainedImage =
        RetainedImage::from_image_bytes("home.png", include_bytes!("../Resources/home.png"),)
            .unwrap();
}

/// Converts the given `RetainedImage` into an `Image` widget in `ui`
fn retained_img_to_image(from: &RetainedImage, ui: &Ui) -> Image {
    Image::new(from.texture_id(ui.ctx()), from.size_vec2())
}

/// Returns an image of a player with the given `color`
fn player_image_with_color(ui: &egui::Ui, color: &Color, size: Vec2) -> Image {
    let player = &PLAYER_IMG;
    Image::new(player.texture_id(ui.ctx()), size).tint(to_color_32(color))
}

/// Returns an image of a home with the given `color`
fn home_image_with_color(ui: &egui::Ui, color: &Color, size: Vec2) -> Image {
    let home = &HOME_IMG;
    Image::new(home.texture_id(ui.ctx()), size).tint(to_color_32(color))
}

/// Converts a `common::Color` into a `Color32`
fn to_color_32(color: &Color) -> Color32 {
    Color32::from_rgb(color.code.0, color.code.1, color.code.2)
}

// size, in pixels, of a cell
const CELL_SIZE: f32 = 30.0;
const CELL_SIZE_2D: Vec2 = Vec2::new(CELL_SIZE, CELL_SIZE);

/// struct for holding information about a Tile that's being rendered
/// `home_colors` is a vector of all the colors of homes on this tile
/// `player_colors` is a vector of all the colors of players on this tile
#[derive(Debug, Clone)]
struct TileWidget {
    tile: Tile,
    home_color: Option<Color>,
    player_colors: Vec<Color>,
}

impl TileWidget {
    /// Returns a `Path` image if you can go `North` from `self.tile`, `empty` otherwise
    fn north_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::North) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
    /// Returns a `Path` image if you can go `South` from `self.tile`, `empty` otherwise
    fn south_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::South) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
    /// Returns a `Path` image if you can go `South` from `self.tile`, `empty` otherwise
    fn east_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::East) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }
    /// Returns a `Path` image if you can go `South` from `self.tile`, `empty` otherwise
    fn west_path(&self) -> &RetainedImage {
        if self.tile.connector.connected_to(CompassDirection::West) {
            &PATH_IMG
        } else {
            &EMPTY_IMG
        }
    }

    /// Returns an `Image` widget in the given `UI` representing `self.tile.connector`
    fn center_image(&self, ui: &Ui) -> Image {
        retained_img_to_image(self.center_ret_img(), ui)
            .rotate(self.center_img_rotation().to_radians(), Vec2::splat(0.5))
    }

    /// Returns the `RetainedImage` corresponding to `self.tile.connector`
    fn center_ret_img(&self) -> &RetainedImage {
        match self.tile.connector {
            ConnectorShape::Path(_) => &PATH_IMG,
            ConnectorShape::Corner(_) => &CORNER_IMG,
            ConnectorShape::Fork(_) => &FORK_IMG,
            ConnectorShape::Crossroads => &CROSSROADS_IMG,
        }
    }

    /// Returns the amount that our `RetainedImage`
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

    /// Renders all homes in `self.home_colors` onto `ui`
    fn render_homes(&self, ui: &mut egui::Ui) {
        if let Some(col) = &self.home_color {
            ui.add(home_image_with_color(ui, col, CELL_SIZE_2D));
        }
    }

    /// Renders all players in `self.player_colors` onto `ui`
    fn render_players(&self, ui: &mut egui::Ui, id: &str) {
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
                        ui.add(player_image_with_color(ui, col, CELL_SIZE_2D * 0.5));
                    })
            });
    }

    fn gem_images(&self, ui: &Ui) -> (Image, Image) {
        let gem_size = CELL_SIZE_2D * 0.8;
        (
            Image::new(GEM_IMGS[&self.tile.gems.0].texture_id(ui.ctx()), gem_size),
            Image::new(GEM_IMGS[&self.tile.gems.1].texture_id(ui.ctx()), gem_size),
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

    // creates main grid for the tile
    Grid::new(format!("{} main", id))
        .min_col_width(0.0)
        .spacing(Vec2::new(0.0, 0.0))
        .show(ui, |ui| {
            ui.add_sized(CELL_SIZE_2D, gem1_img);
            widget.north_path().show(ui);
            widget.render_homes(ui);
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

/// Returns a `common::Grid<TileWidget>` containing all the `Tile` information in `state`.
/// This includes the home and player locations, but not the goal locations
fn widget_grid(state: &State<FullPlayerInfo>) -> CGrid<TileWidget> {
    let mut tiles: CGrid<TileWidget> = state
        .board
        .grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|tile| TileWidget {
                    tile: tile.clone(),
                    player_colors: vec![],
                    home_color: None,
                })
                .collect::<Box<[TileWidget]>>()
        })
        .collect::<Box<[_]>>()
        .into();

    // updates all `TileWidget`s to include player home and goal information
    state.player_info.iter().for_each(|pi| {
        tiles[pi.position()].player_colors.push(pi.color());
        tiles[pi.home()].home_color = Some(pi.color());
    });

    tiles
}

// Render's the `board` inside of a state
fn render_board(ui: &mut egui::Ui, state: &State<FullPlayerInfo>) {
    let tiles: CGrid<TileWidget> = widget_grid(state);

    // create board grid
    Grid::new("board_grid")
        .spacing(Vec2::new(0.0, 0.0))
        .min_col_width(0.0)
        .min_row_height(0.0)
        .show(ui, |ui| {
            tiles.iter().enumerate().fold((), |_, (row_idx, row)| {
                row.iter().enumerate().fold((), |_, (col_idx, tile)| {
                    render_tile(ui, tile.clone(), &format!("({}, {})", col_idx, row_idx))
                });
                ui.end_row();
            })
        });
}

/// Renders the given `Slide` as a label
fn render_slide(ui: &mut egui::Ui, state: &State<FullPlayerInfo>) {
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
fn render_state_info(ui: &mut egui::Ui, state: &State<FullPlayerInfo>) {
    let spare_tile_widget = TileWidget {
        tile: state.board.spare.clone(),
        player_colors: vec![],
        home_color: None,
    };

    let spare_text = RichText::new("Spare Tile:").heading().strong();
    let last_text = RichText::new("Last Slide:").heading().strong();
    let curr_player_text = RichText::new("Current Player").heading().strong();
    let no_players_text = RichText::new("No Players in Game!").heading().strong();

    ui.vertical_centered(|ui| {
        ui.add_space(CELL_SIZE * 2.0);
        ui.label(spare_text);
        render_tile(ui, spare_tile_widget, "spare");

        ui.add_space(CELL_SIZE * 2.0);
        ui.label(last_text);
        render_slide(ui, state);

        ui.add_space(CELL_SIZE * 2.0);
        if state.player_info.is_empty() {
            ui.label(no_players_text);
        } else {
            ui.label(curr_player_text);
            let curr_pl = player_image_with_color(ui, &state.player_info[0].color(), CELL_SIZE_2D);
            ui.add_sized(CELL_SIZE_2D * 0.5, curr_pl);
        }
    });
}

/// Render `state` onto the `ui`
fn render_state(ui: &mut egui::Ui, state: &State<FullPlayerInfo>) {
    // create grid for the state
    Grid::new("state_grid")
        .spacing(Vec2::new(25.0, 0.0))
        .show(ui, |ui| {
            render_board(ui, state);
            ui.vertical(|ui| render_state_info(ui, state));
        });
}

/// Trait describing types that can observe games run by a `Referee`
pub trait Observer {
    /// Recieves a state from the referee to render
    fn recieve_state(&mut self, state: State<FullPlayerInfo>);

    /// Indicates to the Observer that the game has ended and no more `State`s will be sent
    fn game_over(&mut self);
}

/// Contains all information needed for an ObserverGUI to render the game
///
/// Uses `Arc` and `Mutex` so the Observer is thread-safe :)
#[derive(Debug, Default, Clone)]
pub struct ObserverGUI {
    /// `VecDeque` holding all the states the `ObserverGUI` has recieved
    states: Arc<Mutex<VecDeque<State<FullPlayerInfo>>>>,
    /// Flag indicating if the `Referee` has told the `ObserverGUI` the game has ended
    game_over: Arc<Mutex<bool>>,
}

impl Observer for ObserverGUI {
    /// Recie
    fn recieve_state(&mut self, state: State<FullPlayerInfo>) {
        self.states.lock().unwrap().push_back(state);
    }

    fn game_over(&mut self) {
        *self.game_over.lock().unwrap() = true;
    }
}

/// Writes the `JsonRefereeState` representation of `state` to a path the user chooses
fn save_json_state(state: State<FullPlayerInfo>) {
    let path = std::env::current_dir().unwrap();
    if let Some(path) = rfd::FileDialog::new()
        .set_directory(&path)
        .add_filter("json", &[".json"])
        .set_file_name("state.json")
        .save_file()
    {
        let jrs: JsonRefereeState = state.into();
        serde_json::to_writer_pretty(File::create(path).unwrap(), &jrs)
            .expect("Writing to json failed!");
    };
}

// Allows `ObserverGUI`s to be rendered as as an `eframe::App`.
impl eframe::App for ObserverGUI {
    /// Updates the contents of our `ObserverGUI` window
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // aquire the lock to `self.states`
            let mut states = self.states.lock().unwrap();

            // if there are states to render, render the first state
            if !states.is_empty() {
                render_state(ui, &states[0]);
            }

            // draw the buttons below the state
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                // if we have a next state, display a "Next" button
                if states.len() > 1 {
                    // if the "Next" button is clicked, pop the first state from `self.states`
                    if ui.button("Next").clicked() {
                        states.pop_front();
                    }
                } else {
                    ui.label("No more states to render!");
                };

                // if we have a state to save, display a save button
                if !states.is_empty() && ui.button("Save").clicked() {
                    save_json_state(states[0].clone());
                }
            });
        });
    }
}
