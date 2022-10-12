#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::cmp::Ordering;
use std::io;

use common::board::Board;
use common::gem::Gem;
use common::tile::{ConnectorShape, Tile};
use serde::{Deserialize, Serialize};

pub type Position = (usize, usize);

#[derive(Debug, Deserialize)]
struct JsonBoard {
    connectors: Matrix<Connector>,
    treasures: Matrix<Treasure>,
}

#[derive(Debug, Deserialize)]
struct Matrix<T>(Vec<Row<T>>);

#[derive(Debug, Deserialize)]
struct Row<T>(Vec<T>);

#[derive(Debug, Deserialize)]
enum Connector {
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
        use common::tile::CompassDirection::*;
        use common::tile::ConnectorShape::*;
        use common::tile::PathOrientation::*;
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
struct Treasure(Gem, Gem);

impl From<Treasure> for (Gem, Gem) {
    fn from(val: Treasure) -> Self {
        (val.0, val.1)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Coordinate {
    #[serde(rename(deserialize = "row#", serialize = "row#"))]
    row: Index,
    #[serde(rename(deserialize = "column#", serialize = "column#"))]
    column: Index,
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

#[derive(Debug, Deserialize, Serialize)]
struct Index(usize);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ValidJson {
    Board(JsonBoard),
    Coordinate(Coordinate),
}

impl From<JsonBoard> for Board<7> {
    fn from(val: JsonBoard) -> Self {
        let zipped_board = val
            .treasures
            .0
            .into_iter()
            .flat_map(|t| t.0)
            .zip(val.connectors.0.into_iter().flat_map(|c| c.0));
        let mut grid = [[(); 7]; 7].map(|list| list.map(|_| None));

        for (cell, tile_info) in grid.iter_mut().flatten().zip(zipped_board) {
            *cell = Some(Tile {
                connector: tile_info.1.into(),
                gems: tile_info.0.into(),
            });
        }
        Board::new(
            grid,
            Tile {
                connector: ConnectorShape::Crossroads,
                gems: (Gem::amethyst, Gem::garnet),
            },
        )
    }
}

fn cmp_coordinates(c1: &Coordinate, c2: &Coordinate) -> Ordering {
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

fn main() -> Result<(), String> {
    // Turn the STDIN Stream into A ValidJson Stream
    let deserializer = serde_json::Deserializer::from_reader(io::stdin().lock());
    let mut test_input = deserializer.into_iter::<crate::ValidJson>().flatten();

    let board: Board<7> = match test_input.next().ok_or("No valid Board JSON found")? {
        ValidJson::Board(board) => board.into(),
        _ => Err("Board was not the first JSON object sent")?,
    };

    // Position is the tuple (usize, usize)
    let from_pos: Position = match test_input.next().ok_or("No valid Coordinate JSON found")? {
        ValidJson::Coordinate(coord) => coord.into(),
        _ => Err("Coordinate was not the second JSON object sent")?,
    };

    let mut reachable_pos = board
        .reachable(from_pos)
        .unwrap()
        .into_iter()
        .map(|pos| pos.into())
        .collect::<Vec<Coordinate>>();
    reachable_pos.sort_by(cmp_coordinates);

    println!("{}", serde_json::to_string(&reachable_pos).unwrap());

    Ok(())
}
