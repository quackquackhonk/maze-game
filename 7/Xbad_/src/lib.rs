use std::{
    collections::HashSet,
    io::{Read, Write},
    sync::Arc,
};

use anyhow::{anyhow, bail};
use common::{json::Name, FullPlayerInfo, State};
use parking_lot::Mutex;
use players::{
    bad_player::BadPlayer,
    player::{LocalPlayer, PlayerApi},
};
use referee::{
    json::{JsonRefereeState, PlayerSpec},
    observer::Observer,
    referee::{Player, Referee},
};
use serde::{Deserialize, Serialize};

/// Enumerated Valid JSON input for `xchoice`
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ValidJson {
    PlayerSpec(Vec<PlayerSpec>),
    RefereeState(JsonRefereeState),
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
    writer.write_all(serde_json::to_string(&output)?.as_bytes())?;
    Ok(())
}

pub fn read_and_write_json(
    reader: impl Read,
    writer: &mut impl Write,
    mut observers: Vec<Box<dyn Observer>>,
) -> anyhow::Result<()> {
    let mut input = get_json_iter_from_reader(reader)?;

    let players: Vec<Box<dyn PlayerApi + Send>> = match input
        .next()
        .ok_or_else(|| anyhow!("Did not recieve JSON"))?
    {
        ValidJson::PlayerSpec(pss) => pss
            .into_iter()
            .map(|pss| -> Box<dyn PlayerApi + Send> {
                match pss {
                    PlayerSpec::PS(ps) => {
                        let (name, strategy) = ps.into();
                        Box::new(LocalPlayer::new(name, strategy))
                    }
                    PlayerSpec::BadPS(bad_ps) => {
                        let (name, strategy, bad_fm) = bad_ps.into();
                        Box::new(BadPlayer::new(
                            Box::new(LocalPlayer::new(name, strategy)),
                            bad_fm,
                        ))
                    }
                    _ => panic!("BadPS2s are not valid input for xbad"),
                }
            })
            .collect(),
        _ => bail!(""),
    };

    let state: State<FullPlayerInfo> = match input
        .next()
        .ok_or_else(|| anyhow!("Did not receive JSON"))?
    {
        ValidJson::RefereeState(a) => a.into(),
        _ => bail!(""),
    };

    let mut state: State<Player> = State {
        board: state.board,
        player_info: state
            .player_info
            .into_iter()
            .zip(players)
            .map(|(info, api)| Player {
                api: Arc::new(Mutex::new(api)),
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

    let mut kicked_names: Vec<Name> = game_result
        .kicked
        .into_iter()
        .flat_map(|k| k.name())
        .collect();
    kicked_names.sort();

    write_json_out_to_writer((winner_names, kicked_names), writer)?;

    Ok(())
}
