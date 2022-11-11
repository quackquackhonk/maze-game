#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use std::io::{stdin, stdout, Read, Write};

use anyhow::anyhow;
use common::board::Board;
use common::grid::Position;
use common::json::{cmp_coordinates, Coordinate, JsonBoard};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidJson {
    Board(JsonBoard),
    Coordinate(Coordinate),
}

/// Read bytes (the JSON) from the reader and Write the results (the result JSON) to the writer
fn read_json_and_write_json(reader: impl Read, writer: &mut impl Write) -> anyhow::Result<()> {
    let mut test_input = get_json_iter_from_reader(reader)?;

    let board: Board = match test_input
        .next()
        .ok_or(anyhow!("No valid Board JSON found"))?
    {
        ValidJson::Board(board) => board.into(),
        _ => Err(anyhow!("Board was not the first JSON object sent"))?,
    };

    // Position is the tuple (usize, usize)
    let from_pos: Position = match test_input
        .next()
        .ok_or(anyhow!("No valid Coordinate JSON found"))?
    {
        ValidJson::Coordinate(coord) => coord.into(),
        _ => Err(anyhow!("Coordinate was not the second JSON object sent"))?,
    };

    let mut reachable_pos = board
        .reachable(from_pos)?
        .into_iter()
        .map(Position::into)
        .collect::<Vec<Coordinate>>();
    reachable_pos.sort_by(cmp_coordinates);

    writer.write_all(serde_json::to_string(&reachable_pos)?.as_bytes())?;

    Ok(())
}

/// Turn the STDIN Stream into A ValidJson Stream
fn get_json_iter_from_reader(reader: impl Read) -> anyhow::Result<impl Iterator<Item = ValidJson>> {
    let deserializer = serde_json::Deserializer::from_reader(reader);
    Ok(deserializer
        .into_iter::<crate::ValidJson>()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter())
}

fn main() -> anyhow::Result<()> {
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
