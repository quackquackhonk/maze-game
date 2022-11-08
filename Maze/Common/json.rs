use std::cmp::Ordering;

use aliri_braid::braid;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use unordered_pair::UnorderedPair;

use crate::{
    board::{Board, Slide},
    gem::Gem,
    tile::{CompassDirection, ConnectorShape, Tile},
    Color, ColorName, FullPlayerInfo, PlayerInfo, PubPlayerInfo, State,
};

#[derive(Debug, Error)]
#[error("this name is invalid")]
pub struct InvalidName;

#[braid(serde, validator)]
pub struct Name;

impl aliri_braid::Validator for Name {
    type Error = InvalidName;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        let hexcode_re = regex::Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        (raw.len() <= 20 && hexcode_re.is_match(raw))
            .then_some(())
            .ok_or(InvalidName)
    }
}

#[derive(Debug, Error)]
#[error("this json is formed incorrectly: {msg}")]
pub struct JsonError {
    pub msg: String,
}

impl PartialEq<&str> for Name {
    fn eq(&self, &other: &&str) -> bool {
        self.0 == other
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonBoard {
    connectors: Matrix<Connector>,
    treasures: Matrix<Treasure>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Matrix<T>(Vec<Row<T>>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Row<T>(Vec<T>);

#[derive(Debug, Deserialize, Serialize)]
pub enum Connector {
    #[serde(rename = "│")]
    VerticalPath,
    #[serde(rename = "─")]
    HorizontalPath,
    #[serde(rename = "┐")]
    SouthCorner,
    #[serde(rename = "└")]
    NorthCorner,
    #[serde(rename = "┌")]
    EastCorner,
    #[serde(rename = "┘")]
    WestCorner,
    #[serde(rename = "┬")]
    SouthFork,
    #[serde(rename = "┴")]
    NorthFork,
    #[serde(rename = "┤")]
    WestFork,
    #[serde(rename = "├")]
    EastFork,
    #[serde(rename = "┼")]
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

impl From<ConnectorShape> for Connector {
    fn from(cs: ConnectorShape) -> Self {
        use crate::tile::CompassDirection::*;
        use crate::tile::ConnectorShape::*;
        use crate::tile::PathOrientation::*;
        match cs {
            Path(Vertical) => Connector::VerticalPath,
            Path(Horizontal) => Connector::HorizontalPath,
            Corner(South) => Connector::SouthCorner,
            Corner(North) => Connector::NorthCorner,
            Corner(East) => Connector::EastCorner,
            Corner(West) => Connector::WestCorner,
            Fork(South) => Connector::SouthFork,
            Fork(North) => Connector::NorthFork,
            Fork(West) => Connector::WestFork,
            Fork(East) => Connector::EastFork,
            Crossroads => Connector::Crossroads,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Treasure(Gem, Gem);

impl From<Treasure> for UnorderedPair<Gem> {
    fn from(val: Treasure) -> Self {
        (val.0, val.1).into()
    }
}

impl From<UnorderedPair<Gem>> for Treasure {
    fn from(val: UnorderedPair<Gem>) -> Self {
        Treasure(val.0, val.1)
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

#[must_use]
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

impl From<JsonBoard> for Board {
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
                gems: (Gem::Amethyst, Gem::Garnet).into(),
            },
        )
    }
}

impl From<Board> for (JsonBoard, JsonTile) {
    fn from(b: Board) -> Self {
        let rows = b.num_rows();
        let cols = b.num_cols();
        let mut connectors = vec![];
        let mut treasures = vec![];

        for row in 0..rows {
            let mut connector_row = Row(vec![]);
            let mut treasure_row = Row(vec![]);
            for col in 0..cols {
                connector_row.0.push(b[(col, row)].connector.into());
                treasure_row.0.push(b[(col, row)].gems.into());
            }
            connectors.push(connector_row);
            treasures.push(treasure_row);
        }

        (
            JsonBoard {
                connectors: Matrix(connectors),
                treasures: Matrix(treasures),
            },
            b.extra.into(),
        )
    }
}

impl From<(JsonBoard, JsonTile)> for Board {
    fn from((jboard, jtile): (JsonBoard, JsonTile)) -> Self {
        let mut zipped_board = jboard
            .treasures
            .0
            .into_iter()
            .flat_map(|t| t.0)
            .zip(jboard.connectors.0.into_iter().flat_map(|c| c.0));
        let grid = [[(); 7]; 7].map(|list| {
            list.map(|_| {
                let tile_info = zipped_board.next().unwrap();
                Tile {
                    connector: tile_info.1.into(),
                    gems: tile_info.0.into(),
                }
            })
        });

        Board::new(grid, jtile.into())
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
    pub board: JsonBoard,
    #[allow(dead_code)]
    pub spare: JsonTile,
    /// the first player in `plmt` is the currently active player  
    pub plmt: Vec<JsonPlayer>,
    pub last: JsonAction,
}

impl From<JsonState> for State<FullPlayerInfo> {
    fn from(jstate: JsonState) -> Self {
        let player_info: Vec<FullPlayerInfo> =
            jstate.plmt.into_iter().map(|pi| pi.into()).collect();
        State {
            board: (jstate.board, jstate.spare).into(),
            player_info: player_info.into(),
            previous_slide: jstate.last.into(),
        }
    }
}

impl From<JsonState> for State<PubPlayerInfo> {
    fn from(jstate: JsonState) -> Self {
        let player_info: Vec<PubPlayerInfo> = jstate.plmt.into_iter().map(|pi| pi.into()).collect();
        State {
            board: (jstate.board, jstate.spare).into(),
            player_info: player_info.into(),
            previous_slide: jstate.last.into(),
        }
    }
}

/// JSON representation for a single `Tile` in the `Board`
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct JsonTile {
    tilekey: Connector,
    #[serde(rename = "1-image")]
    image1: Gem,
    #[serde(rename = "2-image")]
    image2: Gem,
}

impl From<JsonTile> for Tile {
    fn from(jtile: JsonTile) -> Self {
        Tile {
            connector: jtile.tilekey.into(),
            gems: (jtile.image1, jtile.image2).into(),
        }
    }
}

impl From<Tile> for JsonTile {
    fn from(tile: Tile) -> Self {
        JsonTile {
            tilekey: tile.connector.into(),
            image1: tile.gems.0,
            image2: tile.gems.1,
        }
    }
}

/// Describes a player's current location, the
/// location of its home, and the color of its avatar.
#[derive(Debug, Deserialize)]
pub struct JsonPlayer {
    pub current: Coordinate,
    pub home: Coordinate,
    pub color: JsonColor,
}

impl From<JsonPlayer> for FullPlayerInfo {
    fn from(jp: JsonPlayer) -> Self {
        FullPlayerInfo::new(
            jp.home.into(),
            jp.current.into(),
            // FIXME: this should not always be (1, 1)
            (1, 1),
            jp.color
                .try_into()
                .expect("Json Color values are always valid"),
        )
    }
}

impl From<JsonPlayer> for PubPlayerInfo {
    fn from(jp: JsonPlayer) -> Self {
        Self {
            current: jp.current.into(),
            home: jp.home.into(),
            color: jp
                .color
                .try_into()
                .expect("Json Color values are always valid"),
        }
    }
}

/// Type alias for representing color strings in the
/// Json testing input
pub type JsonColor = String;

/// Conversion from `JsonColor`s into `common::Color`s
impl TryFrom<JsonColor> for Color {
    type Error = String;

    fn try_from(value: JsonColor) -> Result<Self, Self::Error> {
        use ColorName::*;
        let hexcode_re =
            regex::Regex::new(r"^[A-F|\d][A-F|\d][A-F|\d][A-F|\d][A-F|\d][A-F|\d]$").unwrap();
        match value.as_str() {
            "purple" => Ok(Purple.into()),
            "orange" => Ok(Orange.into()),
            "pink" => Ok(Pink.into()),
            "red" => Ok(Red.into()),
            "green" => Ok(Green.into()),
            "blue" => Ok(Blue.into()),
            "yellow" => Ok(Yellow.into()),
            "white" => Ok(White.into()),
            "black" => Ok(Black.into()),
            hexcode if hexcode_re.is_match(hexcode) => {
                // parse the hexcode
                let rgb = hex::decode(hexcode).expect("Hexcodes will be valid from regex match");
                Ok(Color {
                    name: hexcode.to_string(),
                    code: (rgb[0], rgb[1], rgb[2]),
                })
            }
            // need to do a regex match for color codes
            _ => Err(format!("Invalid Color code {}", value)),
        }
    }
}

impl From<Color> for JsonColor {
    fn from(color: Color) -> Self {
        color.name
    }
}

/// Specifies the last sliding action that an actor
/// performed; `None` indicates that no sliding action has been performed yet.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonAction(Option<(Index, JsonDirection)>);

impl From<JsonAction> for Option<Slide> {
    fn from(ja: JsonAction) -> Self {
        let jslide = ja.0?;
        Some(Slide {
            index: jslide.0 .0,
            direction: jslide.1.into(),
        })
    }
}

impl From<Option<Slide>> for JsonAction {
    fn from(s: Option<Slide>) -> Self {
        match s {
            None => JsonAction(None),
            Some(Slide { index, direction }) => JsonAction(Some((Index(index), direction.into()))),
        }
    }
}

/// Describes the direction in which a player may slide the tiles of a row
/// or column. For example, "LEFT" means that the spare tile is inserted into the
/// right side, such that the pieces move to the left, and the
/// left-most tile of the row drops out.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
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

impl From<CompassDirection> for JsonDirection {
    fn from(cd: CompassDirection) -> Self {
        use CompassDirection::*;
        use JsonDirection::*;
        match cd {
            North => UP,
            South => DOWN,
            East => RIGHT,
            West => LEFT,
        }
    }
}

/// Describes the possible counter-clockwise rotations around
/// the center of a tile.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonDegree(pub usize);

impl TryFrom<JsonDegree> for usize {
    type Error = JsonError;

    fn try_from(d: JsonDegree) -> Result<Self, Self::Error> {
        match d.0 {
            0 => Ok(0),
            90 => Ok(1),
            180 => Ok(2),
            270 => Ok(3),
            _ => Err(JsonError {
                msg: format!("Invalid JsonDegree {}", d.0),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_validator() {
        assert!(serde_json::from_str::<Name>("\"Bill\"").is_ok());
        assert!(serde_json::from_str::<Name>("\"\"").is_err());
        assert!(serde_json::from_str::<Name>("\"_\"").is_err());
        assert!(serde_json::from_str::<Name>("\"BartholomewRobertsonTheThird\"").is_err());
    }
}
