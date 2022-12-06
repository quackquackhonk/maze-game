use common::{
    json::{Coordinate, JsonAction, JsonBoard, JsonColor, JsonError, JsonTile, Name},
    FullPlayerInfo, PlayerInfo, State,
};
use players::{bad_player::BadFM, player::PlayerApi, strategy::NaiveStrategy};
use serde::{Deserialize, Serialize};

use crate::referee::GameResult;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PS(Name, JsonStrategy);

impl From<PS> for (Name, NaiveStrategy) {
    fn from(ps: PS) -> Self {
        (ps.0, ps.1.into())
    }
}

#[test]
fn ps_parse_test() {
    assert_eq!(
        serde_json::from_str::<PS>("[\"bob\", \"Riemann\"]").unwrap(),
        PS(Name::from_static("bob"), JsonStrategy::Riemann)
    );
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BadPS(Name, JsonStrategy, BadFM);

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BadPS2(Name, JsonStrategy, BadFM, u64);

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayerSpec {
    PS(PS),
    BadPS(BadPS),
    BadPS2(BadPS2),
}

impl From<BadPS> for (Name, NaiveStrategy, BadFM) {
    fn from(bad_ps: BadPS) -> Self {
        (bad_ps.0, bad_ps.1.into(), bad_ps.2)
    }
}

impl From<BadPS2> for (Name, NaiveStrategy, BadFM, u64) {
    fn from(bad_ps2: BadPS2) -> Self {
        (bad_ps2.0, bad_ps2.1.into(), bad_ps2.2, bad_ps2.3)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum JsonStrategy {
    Riemann,
    Euclid,
}

impl From<JsonStrategy> for NaiveStrategy {
    fn from(jss: JsonStrategy) -> Self {
        match jss {
            JsonStrategy::Riemann => NaiveStrategy::Riemann,
            JsonStrategy::Euclid => NaiveStrategy::Euclid,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRefereeState {
    board: JsonBoard,
    spare: JsonTile,
    plmt: Vec<JsonRefereePlayer>,
    last: JsonAction,
}

impl TryFrom<JsonRefereeState> for State<FullPlayerInfo> {
    type Error = JsonError;

    fn try_from(jrs: JsonRefereeState) -> Result<Self, Self::Error> {
        Ok(State {
            board: (jrs.board, jrs.spare).into(),
            player_info: jrs
                .plmt
                .into_iter()
                .map(|a| a.try_into())
                .collect::<Result<_, _>>()?,
            previous_slide: jrs.last.into(),
        })
    }
}

impl From<State<FullPlayerInfo>> for JsonRefereeState {
    fn from(st: State<FullPlayerInfo>) -> Self {
        let (board, spare) = st.board.into();
        JsonRefereeState {
            board,
            spare,
            plmt: st.player_info.into_iter().map(|pi| pi.into()).collect(),
            last: st.previous_slide.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRefereePlayer {
    current: Coordinate,
    home: Coordinate,
    goto: Coordinate,
    color: JsonColor,
}

impl TryFrom<JsonRefereePlayer> for FullPlayerInfo {
    type Error = JsonError;

    fn try_from(jrp: JsonRefereePlayer) -> Result<Self, Self::Error> {
        Ok(Self::new(
            jrp.home.into(),
            jrp.current.into(),
            jrp.goto.into(),
            jrp.color.try_into()?,
        ))
    }
}

impl From<FullPlayerInfo> for JsonRefereePlayer {
    fn from(pi: FullPlayerInfo) -> Self {
        JsonRefereePlayer {
            current: pi.position().into(),
            home: pi.home().into(),
            goto: pi.goal.into(),
            color: pi.color().into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JsonGameResult(Vec<Name>, Vec<Name>);

impl From<GameResult> for JsonGameResult {
    fn from(gr: GameResult) -> Self {
        JsonGameResult(
            gr.winners.into_iter().flat_map(|p| p.name()).collect(),
            gr.kicked.into_iter().flat_map(|p| p.name()).collect(),
        )
    }
}
