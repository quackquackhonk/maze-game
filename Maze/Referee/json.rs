use common::{
    json::{Coordinate, JsonAction, JsonBoard, JsonColor, JsonTile, Name},
    PlayerInfo, State,
};
use players::strategy::NaiveStrategy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PS(Name, JsonStrategy);

impl From<PS> for (Name, NaiveStrategy) {
    fn from(ps: PS) -> Self {
        (ps.0, ps.1.into())
    }
}

#[derive(Debug, Deserialize)]
pub enum JsonStrategy {
    Riemman,
    Euclid,
}

impl From<JsonStrategy> for NaiveStrategy {
    fn from(jss: JsonStrategy) -> Self {
        match jss {
            JsonStrategy::Riemman => NaiveStrategy::Riemann,
            JsonStrategy::Euclid => NaiveStrategy::Euclid,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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
