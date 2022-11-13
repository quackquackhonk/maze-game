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
pub enum JsonFState {
    #[serde(rename = "false")]
    False,
    State(JsonState),
}

impl From<JsonFState> for Option<State<PubPlayerInfo>> {
    fn from(jfs: JsonFState) -> Self {
        match jfs {
            JsonFState::False => None,
            JsonFState::State(st) => Some(st.into()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonArguments {
    FState(JsonFState),
    State(JsonState),
    Coordinate(Coordinate),
    Boolean(bool),
}

impl From<Option<State<PubPlayerInfo>>> for JsonArguments {
    fn from(st: Option<State<PubPlayerInfo>>) -> Self {
        match st {
            Some(state) => JsonArguments::FState(JsonFState::State(state.into())),
            None => JsonArguments::FState(JsonFState::False),
        }
    }
}

impl From<State<PubPlayerInfo>> for JsonArguments {
    fn from(st: State<PubPlayerInfo>) -> Self {
        JsonArguments::State(st.into())
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
        Self(JsonMName::Win, vec![did_win.into()])
    }
}
