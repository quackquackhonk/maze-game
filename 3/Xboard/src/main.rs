#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::cmp::Ordering;
use std::io::{stdin, stdout, Read, Write};

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

/// Read bytes (the JSON) from the reader and Write the results (the result JSON) to the writer
fn read_json_and_write_json(reader: impl Read, writer: &mut impl Write) -> Result<(), String> {
    let mut test_input = get_json_iter_from_reader(reader)?;

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
        .reachable(from_pos)?
        .into_iter()
        .map(Position::into)
        .collect::<Vec<Coordinate>>();
    reachable_pos.sort_by(cmp_coordinates);

    writer
        .write(
            serde_json::to_string(&reachable_pos)
                .map_err(|e| e.to_string())?
                .as_bytes(),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Turn the STDIN Stream into A ValidJson Stream
fn get_json_iter_from_reader(reader: impl Read) -> Result<impl Iterator<Item = ValidJson>, String> {
    let deserializer = serde_json::Deserializer::from_reader(reader);
    Ok(deserializer
        .into_iter::<crate::ValidJson>()
        .map(|x| x.map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter())
}

fn main() -> Result<(), String> {
    read_json_and_write_json(stdin().lock(), &mut stdout().lock())?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_handle_client_from_file() {
        let tests_path = Path::new("./../Tests/");
        let files = tests_path.read_dir().unwrap().collect::<Vec<_>>();
        let file_count = files.len() / 2;
        let mut results: Vec<(Option<String>, Option<String>)> = vec![(None, None); file_count];
        for dir_entry in files.into_iter().flatten() {
            let path = dir_entry.path();
            let mut split_path = path.file_name().unwrap().to_str().unwrap().split('-');
            if let Ok(num) = split_path.next().unwrap().parse::<usize>() {
                match split_path.next() {
                    Some("in.json") => {
                        let mut buf = Vec::new();
                        read_json_and_write_json(
                            &mut BufReader::new(File::open(&path).unwrap()),
                            &mut buf,
                        )
                        .unwrap();

                        results[num].0 = Some(String::from_utf8(buf).unwrap());
                    }
                    Some("out.json") => {
                        results[num].1 = Some(std::fs::read_to_string(&path).unwrap())
                    }
                    _ => {}
                };
            }
        }

        for (input, output) in results {
            let input = input
                .iter()
                .map(|str| serde_json::from_str(str).unwrap())
                .collect::<Vec<serde_json::Value>>();
            let output = output
                .iter()
                .map(|str| serde_json::from_str(str).unwrap())
                .collect::<Vec<serde_json::Value>>();
            assert_eq!(input, output);
        }
    }
}
