use std::{
    cell::RefCell,
    collections::VecDeque,
    fs::{self, File},
    rc::Rc,
};

use common::{
    gem::Gem,
    gem::GEM_IMGS,
    grid::Grid as CGrid,
    tile::{CompassDirection, ConnectorShape, PathOrientation, Tile},
    Color, ColorName, PlayerInfo, State,
};
use egui::{Align, Color32, Grid, Image, Layout, Vec2, Widget};
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

/// struct for holding information about a Tile that's being rendered
/// `home_colors` is a vector of all the colors of homes on this tile
/// `player_colors` is a vector of all the colors of players on this tile
#[derive(Debug, Clone)]
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
    state.player_info.iter().for_each(|pi| {
        tile_widget_grid[pi.position]
            .player_colors
            .push(pi.color.clone());
        tile_widget_grid[pi.home].home_colors.push(pi.color.clone());
    });
    Grid::new("state_grid")
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

pub trait Observer {
    /// Recieves a state from the referee to render
    fn recieve_state(&mut self, state: State);

    /// Indicates to the Observer that the game has ended and no more `State`s will be sent
    fn game_over(&mut self);
}

/// Contains all information needed for an ObserverGUI to render the game
#[derive(Debug, Default, Clone)]
pub struct ObserverGUI {
    states: Rc<RefCell<VecDeque<State>>>,
    game_over: Rc<RefCell<bool>>,
}

impl Observer for ObserverGUI {
    fn recieve_state(&mut self, state: State) {
        self.states.borrow_mut().push_back(state);
    }

    fn game_over(&mut self) {
        *self.game_over.borrow_mut() = true;
    }
}

impl eframe::App for ObserverGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.states.borrow().is_empty() {
                render_state(ui, &self.states.borrow()[0]);
            }
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                let mut st = self.states.borrow_mut();
                if st.len() > 1 {
                    if ui.button("Next").clicked() {
                        st.pop_front();
                    }
                } else {
                    ui.label("No more states to render!");
                };

                if !st.is_empty() && ui.button("Save").clicked() {
                    let path = std::env::current_dir().unwrap();
                    if let Some(path) = rfd::FileDialog::new()
                        .set_directory(&path)
                        .add_filter("json", &[".json"])
                        .set_file_name("state.json")
                        .save_file()
                    {
                        let jrs: JsonRefereeState = st[0].clone().into();
                        serde_json::to_writer_pretty(File::create(path).unwrap(), &jrs)
                            .expect("Writing to json failed!");
                    };
                }
                drop(st);
            });
        });
    }
}
