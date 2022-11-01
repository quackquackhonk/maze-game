use common::{
    json::{Coordinate, JsonAction, JsonBoard, JsonColor, JsonTile, Name},
    PlayerInfo, State,
};
use players::strategy::NaiveStrategy;
use serde::{Deserialize, Serialize};

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

impl From<JsonRefereeState> for State {
    fn from(jrs: JsonRefereeState) -> Self {
        //        r#ref.run_game(players);
        State {
            board: (jrs.board, jrs.spare).into(),
            player_info: jrs.plmt.into_iter().map(|a| a.into()).collect(),
            previous_slide: jrs.last.into(),
        }
    }
}

impl From<State> for JsonRefereeState {
    fn from(st: State) -> Self {
        let (board, spare) = st.board.into();
        JsonRefereeState {
            board,
            spare,
            plmt: st.player_info.into_iter().map(|pi| pi.into()).collect(),
            last: st.previous_slide.into(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRefereePlayer {
    current: Coordinate,
    home: Coordinate,
    goto: Coordinate,
    color: JsonColor,
}

impl From<JsonRefereePlayer> for PlayerInfo {
    fn from(jrp: JsonRefereePlayer) -> Self {
        Self {
            home: jrp.home.into(),
            position: jrp.current.into(),
            goal: jrp.goto.into(),
            color: jrp.color.try_into().expect("meh"),
        }
    }
}

impl From<PlayerInfo> for JsonRefereePlayer {
    fn from(pi: PlayerInfo) -> Self {
        JsonRefereePlayer {
            current: pi.position.into(),
            home: pi.home.into(),
            goto: pi.goal.into(),
            color: pi.color.into(),
        }
    }
}
