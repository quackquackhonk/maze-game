use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
    mem::swap,
};

use common::{json::Name, State};
use players::player::{LocalPlayer, Player};
use referee::{
    json::{JsonRefereeState, PS},
    referee::Referee,
};
use serde::{Deserialize, Serialize};

/// Enumerated Valid JSON input for `xchoice`
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ValidJson {
    PlaserSpec(Vec<PS>),
    RefereeState(JsonRefereeState),
}

/// Turn the `impl Read` into A `ValidJson` Stream
fn get_json_iter_from_reader(reader: impl Read) -> Result<impl Iterator<Item = ValidJson>, String> {
    let deserializer = serde_json::Deserializer::from_reader(reader);
    Ok(deserializer
        .into_iter::<ValidJson>()
        .map(|x| x.map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter())
}

/// Writes the `impl Serialize` to the `impl Write`
fn write_json_out_to_writer(output: impl Serialize, writer: &mut impl Write) -> Result<(), String> {
    writer
        .write(
            serde_json::to_string(&output)
                .map_err(|e| e.to_string())?
                .as_bytes(),
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn read_and_write_json(reader: impl Read, writer: &mut impl Write) -> Result<(), String> {
    let mut input = get_json_iter_from_reader(reader)?;

    let mut players: Vec<Box<dyn Player>> = match input.next().ok_or("asdasdas")? {
        ValidJson::PlaserSpec(pss) => pss
            .into_iter()
            .map(|pss| -> Box<dyn Player> {
                let (name, strategy) = pss.into();
                Box::new(LocalPlayer::new(name, strategy))
            })
            .collect(),
        _ => Err("")?,
    };

    let mut state: State = match input.next().ok_or("ehhhhhhh")? {
        ValidJson::RefereeState(a) => a.into(),
        _ => Err("")?,
    };

    let r#ref = Referee::new(0);
    let mut reached_goal = HashSet::default();

    let gamewinner = r#ref.run_from_state(
        &mut state,
        &mut players,
        &mut reached_goal,
        &mut Vec::default(),
    );
    let (winners, _) = Referee::calculate_winners(gamewinner, players, &state, reached_goal);
    let mut winner_names: Vec<Name> = winners.into_iter().map(|w| w.name()).collect();
    winner_names.sort();

    write_json_out_to_writer(winner_names, writer)?;

    Ok(())
}

fn main() -> Result<(), String> {
    read_and_write_json(stdin().lock(), &mut stdout().lock())
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
                        read_and_write_json(
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
