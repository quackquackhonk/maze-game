use std::{
    cell::RefCell,
    collections::HashSet,
    io::{Read, Write},
    rc::Rc,
};

use common::{json::Name, FullPlayerInfo, State};
use players::player::{LocalPlayer, PlayerApi};
use referee::{
    json::{JsonRefereeState, PS},
    observer::Observer,
    referee::{Player, Referee},
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

    let players: Vec<Box<dyn PlayerApi>> = match input.next().ok_or("asdasdas")? {
        ValidJson::PlayerSpec(pss) => pss
            .into_iter()
            .map(|pss| -> Box<dyn PlayerApi> {
                let (name, strategy) = pss.into();
                Box::new(LocalPlayer::new(name, strategy))
            })
            .collect(),
        _ => Err("")?,
    };

    let state: State<FullPlayerInfo> = match input.next().ok_or("ehhhhhhh")? {
        ValidJson::RefereeState(a) => a.into(),
        _ => Err("")?,
    };

    let mut state: State<Player> = State {
        board: state.board,
        player_info: state
            .player_info
            .into_iter()
            .zip(players)
            .map(|(info, api)| Player {
                api: Rc::new(RefCell::new(api)),
                info,
            })
            .collect(),
        previous_slide: state.previous_slide,
    };

    let r#ref = Referee::new(0);
    let reached_goal = HashSet::default();

    let kicked = Vec::default();

    let game_result = r#ref.run_from_state(&mut state, &mut observers, reached_goal, kicked);
    let mut winner_names: Vec<Name> = game_result
        .winners
        .into_iter()
        .flat_map(|w| w.name())
        .collect();
    winner_names.sort();

    write_json_out_to_writer(winner_names, writer)?;

    Ok(())
}
