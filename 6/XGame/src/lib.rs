use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
    mem::swap,
};

use common::{json::Name, State};
use players::player::{LocalPlayer, Player};
use referee::{
    json::{JsonRefereeState, PS},
    observer::Observer,
    referee::Referee,
};
use serde::{Deserialize, Serialize};
/// Enumerated Valid JSON input for `xchoice`
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ValidJson {
    PlayerSpec(Vec<PS>),
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

pub fn read_and_write_json(
    reader: impl Read,
    writer: &mut impl Write,
    mut observers: Vec<Box<dyn Observer>>,
) -> Result<(), String> {
    let mut input = get_json_iter_from_reader(reader)?;

    let mut players: Vec<Box<dyn Player>> = match input.next().ok_or("asdasdas")? {
        ValidJson::PlayerSpec(pss) => pss
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

    r#ref.broadcast_initial_state(&state, &mut players);
    let mut kicked = Vec::default();

    let gamewinner = r#ref.run_from_state(
        &mut state,
        &mut players,
        &mut observers,
        &mut reached_goal,
        &mut kicked,
    );
    let (winners, _losers) = Referee::calculate_winners(gamewinner, players, &state, reached_goal);
    let mut winner_names: Vec<Name> = winners.into_iter().map(|w| w.name()).collect();
    winner_names.sort();

    write_json_out_to_writer(winner_names, writer)?;

    Ok(())
}
