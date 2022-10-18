use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::{
    board::{Board, Slide},
    gem::Gem,
    tile::{CompassDirection, ConnectorShape, Tile},
    Color, PlayerInfo, State, BOARD_SIZE,
};

#[derive(Debug, Deserialize)]
pub struct JsonBoard {
    connectors: Matrix<Connector>,
    treasures: Matrix<Treasure>,
}

#[derive(Debug, Deserialize)]
pub struct Matrix<T>([Row<T>; BOARD_SIZE]);

#[derive(Debug, Deserialize)]
pub struct Row<T>([T; BOARD_SIZE]);

#[derive(Debug, Deserialize)]
pub enum Connector {
    #[serde(rename(deserialize = "│"))]
    VerticalPath,
    #[serde(rename(deserialize = "─"))]
    HorizontalPath,
    #[serde(rename(deserialize = "┐"))]
    SouthCorner,
    #[serde(rename(deserialize = "└"))]
    NorthCorner,
    #[serde(rename(deserialize = "┌"))]
    EastCorner,
    #[serde(rename(deserialize = "┘"))]
    WestCorner,
    #[serde(rename(deserialize = "┬"))]
    SouthFork,
    #[serde(rename(deserialize = "┴"))]
    NorthFork,
    #[serde(rename(deserialize = "┤"))]
    WestFork,
    #[serde(rename(deserialize = "├"))]
    EastFork,
    #[serde(rename(deserialize = "┼"))]
    Crossroads,
}

impl From<Connector> for ConnectorShape {
    fn from(val: Connector) -> Self {
        use crate::tile::CompassDirection::*;
        use crate::tile::ConnectorShape::*;
        use crate::tile::PathOrientation::*;
        match val {
            Connector::VerticalPath => Path(Vertical),
            Connector::HorizontalPath => Path(Horizontal),
            Connector::SouthCorner => Corner(South),
            Connector::NorthCorner => Corner(North),
            Connector::EastCorner => Corner(East),
            Connector::WestCorner => Corner(West),
            Connector::SouthFork => Fork(South),
            Connector::NorthFork => Fork(North),
            Connector::WestFork => Fork(West),
            Connector::EastFork => Fork(East),
            Connector::Crossroads => Crossroads,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Treasure(Gem, Gem);

impl From<Treasure> for (Gem, Gem) {
    fn from(val: Treasure) -> Self {
        (val.0, val.1)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinate {
    #[serde(rename(deserialize = "row#", serialize = "row#"))]
    pub row: Index,
    #[serde(rename(deserialize = "column#", serialize = "column#"))]
    pub column: Index,
}

impl From<Coordinate> for (usize, usize) {
    fn from(val: Coordinate) -> Self {
        (val.column.0, val.row.0)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(val: (usize, usize)) -> Self {
        Coordinate {
            row: Index(val.1),
            column: Index(val.0),
        }
    }
}

pub fn cmp_coordinates(c1: &Coordinate, c2: &Coordinate) -> Ordering {
    if c1.row.0 < c2.row.0 {
        Ordering::Less
    } else if c1.row.0 > c2.row.0 {
        Ordering::Greater
    } else if c1.column.0 < c2.column.0 {
        Ordering::Less
    } else if c1.column.0 > c2.column.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Index(pub usize);

impl From<JsonBoard> for Board<7> {
    fn from(val: JsonBoard) -> Self {
        let mut zipped_board = val
            .treasures
            .0
            .into_iter()
            .flat_map(|t| t.0)
            .zip(val.connectors.0.into_iter().flat_map(|c| c.0));
        let grid = [[(); 7]; 7].map(|list| {
            list.map(|_| {
                let tile_info = zipped_board.next().unwrap();
                Tile {
                    connector: tile_info.1.into(),
                    gems: tile_info.0.into(),
                }
            })
        });

        Board::new(
            grid,
            Tile {
                connector: ConnectorShape::Crossroads,
                gems: (Gem::amethyst, Gem::garnet),
            },
        )
    }
}

/// Describes the current state of the board; the spare tile; the
/// players and in what order they take turns (left to right); and the last
/// sliding action performed (if any). The first item in "plmt" is the
/// current player.
///
/// # Constraints
/// - `plmt` is a non-empty array
/// - no two `JsonPlayer`s will have the same `JsonColor`
#[derive(Debug, Deserialize)]
pub struct JsonState {
    board: JsonBoard,
    spare: JsonTile,
    /// the first player in `plmt` is the currently active player  
    plmt: Vec<JsonPlayer>,
    last: JsonAction,
}

impl From<JsonState> for State {
    fn from(jstate: JsonState) -> Self {
        let player_info: Vec<PlayerInfo> = jstate.plmt.into_iter().map(|pi| pi.into()).collect();
        State {
            board: jstate.board.into(),
            player_info,
            active_player: 0,
            previous_slide: jstate.last.into(),
        }
    }
}

/// JSON representation for a single `Tile` in the `Board`
#[derive(Debug, Deserialize)]
pub struct JsonTile {
    tilekey: Connector,
    #[serde(rename(deserialize = "1-image"))]
    image1: Gem,
    #[serde(rename(deserialize = "2-image"))]
    image2: Gem,
}

/// Describes a player's current location, the
/// location of its home, and the color of its avatar.
#[derive(Debug, Deserialize)]
pub struct JsonPlayer {
    current: Coordinate,
    home: Coordinate,
    color: JsonColor,
}

impl From<JsonPlayer> for PlayerInfo {
    fn from(jp: JsonPlayer) -> Self {
        PlayerInfo::new(
            jp.home.into(),
            jp.current.into(),
            Gem::amethyst,
            jp.color.into(),
        )
    }
}

/// Type alias for representing color strings in the
/// Json testing input
type JsonColor = String;

/// Conversion from `JsonColor`s into `common::Color`s
impl From<JsonColor> for Color {
    fn from(jc: JsonColor) -> Self {
        match jc.as_str() {
            "purple" => Color::Hex(128, 0, 128),
            "orange" => Color::Hex(255, 165, 0),
            "pink" => Color::Hex(255, 192, 203),
            "red" => Color::Hex(255, 0, 0),
            "green" => Color::Hex(0, 255, 0),
            "blue" => Color::Hex(0, 0, 255),
            "yellow" => Color::Hex(255, 255, 0),
            "white" => Color::Hex(255, 255, 255),
            "black" => Color::Hex(128, 0, 128),
            // need to do a regex match for color codes
            _ => todo!(),
        }
    }
}

/// Specifies the last sliding action that an actor
/// performed; `None` indicates that no sliding action has been performed yet.
#[derive(Debug, Deserialize)]
pub struct JsonAction(Option<(Index, JsonDirection)>);

impl From<JsonAction> for Option<Slide<BOARD_SIZE>> {
    fn from(ja: JsonAction) -> Self {
        let jslide = ja.0?;
        Some(Slide {
            index: jslide.0 .0,
            direction: jslide.1.into(),
        })
    }
}

/// Describes the direction in which a player may slide the tiles of a row
/// or column. For example, "LEFT" means that the spare tile is inserted into the
/// right side, such that the pieces move to the left, and the
/// left-most tile of the row drops out.
#[derive(Debug, Deserialize)]
pub enum JsonDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl From<JsonDirection> for CompassDirection {
    fn from(jd: JsonDirection) -> Self {
        use CompassDirection::*;
        use JsonDirection::*;
        match jd {
            UP => North,
            DOWN => South,
            LEFT => West,
            RIGHT => East,
        }
    }
}

/// Describes the possible counter-clockwise rotations around
/// the center of a tile.
#[derive(Debug, Deserialize)]
pub struct JsonDegree(pub usize);

impl From<JsonDegree> for usize {
    fn from(d: JsonDegree) -> Self {
        d.0 / 90
    }
}
