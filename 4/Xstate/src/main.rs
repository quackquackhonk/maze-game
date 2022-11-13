#![allow(non_snake_case)]

use std::io::{stdin, stdout, Read, Write};

use anyhow::anyhow;
use common::board::Slide;
use common::grid::Position;
use common::json::{cmp_coordinates, Coordinate, JsonDegree, JsonDirection, JsonState};
use common::tile::CompassDirection;
use common::{FullPlayerInfo, State};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidJson {
    State(JsonState),
    Number(usize),
    Direction(JsonDirection),
    // Degree(JsonDegree),
}

fn read_json_and_write_json(reader: impl Read, writer: &mut impl Write) -> anyhow::Result<()> {
    let mut test_input = get_json_iter_from_reader(reader)?;

    let mut state: State<FullPlayerInfo> = match test_input
        .next()
        .ok_or_else(|| anyhow!("No valid State JSON found"))?
    {
        ValidJson::State(state) => state.into(),
        _ => Err(anyhow!("State was not the first JSON object sent"))?,
    };

    let slide: Slide = {
        let index: usize = match test_input
            .next()
            .ok_or_else(|| anyhow!("No valid Index JSON found"))?
        {
            ValidJson::Number(index) => index,
            _ => Err(anyhow!("Index was not the second JSON object sent"))?,
        };

        let dir: CompassDirection = match test_input
            .next()
            .ok_or_else(|| anyhow!("No valid Direction JSON found"))?
        {
            ValidJson::Direction(dir) => dir.into(),
            _ => Err(anyhow!("Direction was not the third JSON object sent"))?,
        };
        state
            .board
            .new_slide(index, dir)
            .ok_or_else(|| anyhow!("Slide is invalid"))?
    };

    let num_rotations: usize = match test_input
        .next()
        .ok_or_else(|| anyhow!("No valid Degree JSON found"))?
    {
        ValidJson::Number(deg) => JsonDegree(deg).try_into()?,
        x => Err(anyhow!(
            "Degree was not the fourth JSON object sent, got {:?}",
            x
        ))?,
    };

    // Perform the move requested by the player
    state.rotate_spare(num_rotations);
    state.slide_and_insert(slide).map_err(|err| anyhow!(err))?;

    // Gets vector of reachable positions
    let mut reachable_pos = state
        .reachable_by_player()
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
        .into_iter::<ValidJson>()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter())
}

fn main() -> anyhow::Result<()> {
    read_json_and_write_json(stdin().lock(), &mut stdout().lock())
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
                        .map_err(|e| {
                            println!("{}", e);
                            e
                        })
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
