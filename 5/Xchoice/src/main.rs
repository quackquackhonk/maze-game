use std::io::{stdin, stdout, Read, Write};

use anyhow::anyhow;
use common::{
    grid::Position,
    json::{Coordinate, JsonState},
    state::{PubPlayerInfo, State},
};
use players::{
    json::{JsonChoice, JsonStrategyDesignation},
    strategy::{NaiveStrategy, Strategy},
};
use serde::{Deserialize, Serialize};

/// Enumerated Valid JSON input for `xchoice`
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ValidJson {
    StrategyDesig(JsonStrategyDesignation),
    State(JsonState),
    Goal(Coordinate),
}

/// Turn the `impl Read` into A `ValidJson` Stream
fn get_json_iter_from_reader(reader: impl Read) -> anyhow::Result<impl Iterator<Item = ValidJson>> {
    let deserializer = serde_json::Deserializer::from_reader(reader);
    Ok(deserializer
        .into_iter::<ValidJson>()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter())
}

/// Writes the `impl Serialize` to the `impl Write`
fn write_json_out_to_writer(output: impl Serialize, writer: &mut impl Write) -> anyhow::Result<()> {
    Ok(writer.write_all(serde_json::to_string(&output)?.as_bytes())?)
}

fn read_and_write_json(reader: impl Read, writer: &mut impl Write) -> anyhow::Result<()> {
    let mut input = get_json_iter_from_reader(reader)?;

    let strat: NaiveStrategy = match input
        .next()
        .ok_or_else(|| anyhow!("No valid JSON Strategy found"))?
    {
        ValidJson::StrategyDesig(strat) => strat.into(),
        _ => Err(anyhow!(
            "StrategyDesignation was not the first json input found"
        ))?,
    };

    let state: State<PubPlayerInfo> = match input
        .next()
        .ok_or_else(|| anyhow!("No valid State JSON found"))?
    {
        ValidJson::State(state) => state.try_into()?,
        _ => Err(anyhow!("State was not the second json input found"))?,
    };

    let goal: Position = match input
        .next()
        .ok_or_else(|| anyhow!("No valid State JSON found"))?
    {
        ValidJson::Goal(state) => state.into(),
        _ => Err(anyhow!("State was not the second json input found"))?,
    };

    let start = state.player_info[0].current;
    let choice = strat.get_move(state, start, goal);
    let action: JsonChoice = choice.into();

    write_json_out_to_writer(action, writer)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
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
