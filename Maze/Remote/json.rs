use common::grid::Position;
use common::json::{Coordinate, JsonState};
use common::{PubPlayerInfo, State};
use players::json::JsonChoice;
use serde::{Deserialize, Serialize};

/// Contains all valid method names a Referee can send to a Player
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum JsonMName {
    Setup,
    TakeTurn,
    Win,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonArguments {
    State(Option<JsonState>),
    Coordinate(Coordinate),
    Boolean(bool),
}

impl From<Option<State<PubPlayerInfo>>> for JsonArguments {
    fn from(st: Option<State<PubPlayerInfo>>) -> Self {
        match st {
            Some(state) => JsonArguments::State(Some(state.into())),
            None => JsonArguments::State(None),
        }
    }
}

impl From<Position> for JsonArguments {
    fn from(p: Position) -> Self {
        JsonArguments::Coordinate(p.into())
    }
}

impl From<bool> for JsonArguments {
    fn from(b: bool) -> Self {
        JsonArguments::Boolean(b)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonResult {
    #[serde(rename = "void")]
    Void,
    Choice(JsonChoice),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonFunctionCall(pub JsonMName, pub Vec<JsonArguments>);

impl JsonFunctionCall {
    pub fn setup(state: Option<State<PubPlayerInfo>>, goal: Position) -> Self {
        Self(JsonMName::Setup, vec![state.into(), goal.into()])
    }

    pub fn take_turn(state: State<PubPlayerInfo>) -> Self {
        Self(JsonMName::TakeTurn, vec![Some(state).into()])
    }

    pub fn win(did_win: bool) -> Self {
        Self(JsonMName::Win, vec![won.into()])
    }
}

