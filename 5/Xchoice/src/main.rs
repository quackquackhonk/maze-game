use std::io::{Read, Write};

use common::grid::Position;
use common::json::{Coordinate, JsonState, JsonStrategyDesignation};
use common::BOARD_SIZE;
use player::strategy::{NaiveStrategy, PlayerBoardState};
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

// fn execute_test<const COLS: usize, const ROWS: usize>(
//     strat: NaiveStrategy,
//     state: PlayerBoardState<COLS, ROWS>,
//     goal: Position,
// ) -> PlayerAction<COLS, ROWS> {
//     strat.get_move(board_state, start, goal_tile)
// }

fn read_and_write_json(reader: impl Read, writer: &mut impl Write) -> Result<(), String> {
    let input = get_json_iter_from_reader(reader)?;

    let strat: NaiveStrategy<BOARD_SIZE, BOARD_SIZE> =
        match input.next().ok_or("No valid JSON Strategy found")? {
            ValidJson::StrategyDesig(strat) => strat.into(),
            _ => Err("StrategyDesignation was not the first json input found")?,
        };

    let state: PlayerBoardState<BOARD_SIZE, BOARD_SIZE> =
        match input.next().ok_or("No valid State JSON found")? {
            ValidJson::State(state) => state.into(),
            _ => Err("State was not the second json input found")?,
        };

    let goal: Position = match input.next().ok_or("No valid State JSON found")? {
        ValidJson::Goal(state) => state.into(),
        _ => Err("State was not the second json input found")?,
    };

    Ok(())
}

fn main() {}
