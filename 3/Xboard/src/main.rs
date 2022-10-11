#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::cmp::Ordering;
use std::io;

use common::gem::Gem;
use common::tile::{ConnectorShape, Tile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Board {
    connectors: Matrix<Connector>,
    treasures: Matrix<Treasure>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Matrix<T>(Vec<Row<T>>);

#[derive(Debug, Deserialize, Serialize)]
struct Row<T>(Vec<T>);

#[derive(Debug, Deserialize, Serialize)]
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
    Board(Board),
    Coordinate(Coordinate),
}

impl From<Board> for common::board::Board<7> {
    fn from(val: Board) -> Self {
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
        common::board::Board::new(
            grid,
            Tile {
                connector: ConnectorShape::Crossroads,
                gems: (Gem::amethyst, Gem::garnet),
            },
        )
    }
}

fn main() {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin().lock());
    let mut test_input = deserializer.into_iter::<crate::ValidJson>().flatten();
    if let Some(ValidJson::Board(board)) = test_input.next() {
        if let Some(ValidJson::Coordinate(coord)) = test_input.next() {
            let board: common::board::Board<7> = board.into();
            let from_pos: (usize, usize) = coord.into();
            let mut reachable_pos = board
                .reachable(from_pos)
                .unwrap()
                .into_iter()
                .map(|pos| pos.into())
                .collect::<Vec<Coordinate>>();
            reachable_pos.sort_by(|c1, c2| {
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
            });
            println!("{}", serde_json::to_string(&reachable_pos).unwrap());
        }
    }
}
