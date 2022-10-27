use std::io::{stdin, stdout, Read, Write};

use common::grid::Position;
use common::json::{Coordinate, JsonAction, JsonState};
use common::{State, BOARD_SIZE};
use players::json::{JsonChoice, JsonStrategyDesignation};
use players::strategy::{NaiveStrategy, PlayerBoardState, Strategy};
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

    let strat: NaiveStrategy = match input.next().ok_or("No valid JSON Strategy found")? {
        ValidJson::StrategyDesig(strat) => strat.into(),
        _ => Err("StrategyDesignation was not the first json input found")?,
    };

    let state: PlayerBoardState = match input.next().ok_or("No valid State JSON found")? {
        ValidJson::State(state) => state.into(),
        _ => Err("State was not the second json input found")?,
    };

    let goal: Position = match input.next().ok_or("No valid State JSON found")? {
        ValidJson::Goal(state) => state.into(),
        _ => Err("State was not the second json input found")?,
    };

    let start = state.players[0].current;
    let choice = strat.get_move(state, start, goal);
    let action: JsonChoice = choice.into();

    write_json_out_to_writer(action, writer)?;

    Ok(())
}

fn main() {
    read_and_write_json(stdin().lock(), &mut stdout().lock());
}
