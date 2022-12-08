use std::{cmp::Ordering, collections::HashSet, hash::Hash};

use aliri_braid::braid;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use unordered_pair::UnorderedPair;

use crate::{
    board::{Board, Slide},
    gem::Gem,
    grid::Position,
    tile::{CompassDirection, ConnectorShape, Tile},
    Color, ColorName, PlayerInfo, PubPlayerInfo, State,
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
pub enum JsonError {
    #[error("This board has not enough gems or connectors")]
    NotEnoughElements,
    #[error("This board has gem pairs that are not unique")]
    NonUniqueGems,
    #[error("Players in this state do not have unique colors")]
    NonUniqueColors,
    #[error("More than one player occupies a tile for a home")]
    NonUniqueHomes,
    #[error("Not enough homes for amount of players in the state")]
    NotEnoughHomes,
    #[error("Player(s) {0:?} have homes on a moveable tile")]
    HomeMoveableTile(Vec<Color>),
    #[error("Player(s) {0:?} have goals on a moveable tile")]
    PlayerGoalMoveableTile(Vec<Color>),
    #[error("Alternate Goals {0:?} are on moveable tiles")]
    GoalMoveableTile(Vec<Position>),
    #[error("The player has been assigned a goal that is in the list of remaining goals")]
    DuplicateAssignedGoals,
    #[error("{0:?} is/are out of bounds on the given board")]
    PositionOutOfBounds(Vec<Position>),
    #[error("{0:?} is not a valid slide for this board")]
    InvalidSlide(Slide),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
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

impl<T: Clone> Clone for Row<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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
    #[serde(rename = "row#")]
    pub row: Index,
    #[serde(rename = "column#")]
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

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}

impl TryFrom<(JsonBoard, JsonTile)> for Board {
    type Error = JsonError;

    fn try_from((jboard, jtile): (JsonBoard, JsonTile)) -> Result<Self, Self::Error> {
        let num_rows = jboard.treasures.0.len();
        let mut gems = jboard
            .treasures
            .0
            .into_iter()
            .flat_map(|t| t.0)
            .map(UnorderedPair::from)
            .collect::<Vec<_>>();
        let num_cols = gems.len() / num_rows;

        gems.push((jtile.image1, jtile.image2).into());

        if !has_unique_elements(&gems) {
            return Err(JsonError::NonUniqueGems);
        }
        gems.pop();

        let mut zipped_board = gems
            .into_iter()
            .zip(jboard.connectors.0.into_iter().flat_map(|c| c.0));

        let grid: Box<[Box<_>]> = (0..num_rows)
            .map(|_| 0..num_cols)
            .map(|list| {
                list.map(|_| {
                    let tile_info = zipped_board.next().ok_or(JsonError::NotEnoughElements)?;
                    Ok(Tile {
                        connector: tile_info.1.into(),
                        gems: tile_info.0,
                    })
                })
                .collect::<Result<_, JsonError>>()
            })
            .collect::<Result<_, JsonError>>()?;

        Ok(Board::new(grid, jtile.into()))
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

/// Describes the current state of the board; the spare tile; the
/// players and in what order they take turns (left to right); and the last
/// sliding action performed (if any). The first item in "plmt" is the
/// current player.
///
/// # Constraints
/// - `plmt` is a non-empty array
/// - no two `JsonPlayer`s will have the same `JsonColor`
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonState {
    pub board: JsonBoard,
    #[allow(dead_code)]
    pub spare: JsonTile,
    /// the first player in `plmt` is the currently active player  
    pub plmt: Vec<JsonPlayer>,
    pub last: JsonAction,
}

impl<PI: PlayerInfo> TryFrom<JsonState> for State<PI>
where
    PI: TryFrom<JsonPlayer, Error = JsonError>,
{
    type Error = JsonError;

    fn try_from(jstate: JsonState) -> Result<Self, Self::Error> {
        let board: Board = (jstate.board, jstate.spare).try_into()?;

        let player_info: Vec<PI> = jstate
            .plmt
            .into_iter()
            .map(|pi| pi.try_into())
            .collect::<Result<_, JsonError>>()?;

        let colors = player_info.iter().map(|pi| pi.color());
        if !has_unique_elements(colors) {
            return Err(JsonError::NonUniqueColors);
        }

        let homes = player_info.iter().map(|pi| pi.home()).collect::<Vec<_>>();
        if !has_unique_elements(&homes) {
            return Err(JsonError::NonUniqueHomes);
        }

        let possible_homes = board.possible_homes().collect::<Vec<_>>();
        let invalid_homes = player_info
            .iter()
            .filter(|pi| !possible_homes.contains(&pi.home()))
            .map(|pi| pi.color())
            .collect::<Vec<_>>();

        if !invalid_homes.is_empty() {
            return Err(JsonError::HomeMoveableTile(invalid_homes));
        }

        homes
            .iter()
            .fold(Ok(()), |acc: Result<(), JsonError>, home| {
                board
                    .in_bounds(home)
                    .then_some(())
                    .ok_or_else(|| JsonError::PositionOutOfBounds(vec![*home]))?;
                acc
            })?;

        let previous_slide = jstate.last.into();
        if let Some(slide) = previous_slide {
            if !board.valid_slide(slide) {
                return Err(JsonError::InvalidSlide(slide));
            }
        }

        Ok(Self {
            board,
            player_info: player_info.into(),
            previous_slide,
        })
    }
}

impl<PI: PlayerInfo> From<State<PI>> for JsonState
where
    JsonPlayer: From<PI>,
{
    fn from(state: State<PI>) -> Self {
        let (board, spare) = state.board.into();
        JsonState {
            board,
            spare,
            plmt: state.player_info.into_iter().map(|i| i.into()).collect(),
            last: state.previous_slide.into(),
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
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonPlayer {
    pub current: Coordinate,
    pub home: Coordinate,
    pub color: JsonColor,
}

impl TryFrom<JsonPlayer> for PubPlayerInfo {
    type Error = JsonError;

    fn try_from(jp: JsonPlayer) -> Result<Self, Self::Error> {
        Ok(Self {
            current: jp.current.into(),
            home: jp.home.into(),
            color: jp.color.try_into()?,
        })
    }
}

impl From<PubPlayerInfo> for JsonPlayer {
    fn from(ppi: PubPlayerInfo) -> Self {
        JsonPlayer {
            current: ppi.current.into(),
            home: ppi.home.into(),
            color: ppi.color.into(),
        }
    }
}

/// Type alias for representing color strings in the
/// Json testing input
pub type JsonColor = String;

/// Conversion from `JsonColor`s into `common::Color`s
impl TryFrom<JsonColor> for Color {
    type Error = JsonError;

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
            _ => Err(anyhow!("Invalid Color code {}", value))?,
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
            _ => Err(anyhow!("Invalid JsonDegree {}", d.0))?,
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
