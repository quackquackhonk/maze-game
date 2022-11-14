use common::grid::Position;
use common::json::{Coordinate, JsonState};
use common::{PubPlayerInfo, State};
use players::json::JsonChoice;
use serde::{de, Deserialize, Serialize};

/// Contains all valid method names a Referee can send to a Player
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum JsonMName {
    Setup,
    TakeTurn,
    Win,
}

#[derive(Debug)]
pub enum JsonFState {
    False,
    State(JsonState),
}

impl<'de> Deserialize<'de> for JsonFState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MaybeState {
            Bool(bool),
            State(JsonState),
        }

        let value = MaybeState::deserialize(deserializer)?;
        match value {
            MaybeState::Bool(false) => Ok(JsonFState::False),
            MaybeState::State(state) => Ok(JsonFState::State(state)),
            MaybeState::Bool(true) => Err(de::Error::unknown_variant("true", &[])),
        }
    }
}

impl Serialize for JsonFState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JsonFState::False => Ok(bool::serialize(&false, serializer)?),
            JsonFState::State(state) => Ok(JsonState::serialize(&state, serializer)?),
        }
    }
}

#[test]
fn test_json_fstate() {
    let mut deserializer = serde_json::Deserializer::from_str("false").into_iter();
    let value = deserializer.next().unwrap().unwrap();
    assert!(matches!(value, JsonFState::False));

    assert_eq!("false", &serde_json::to_string(&JsonFState::False).unwrap());
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

#[derive(Debug)]
pub enum JsonResult {
    Void,
    Choice(JsonChoice),
}

impl<'de> Deserialize<'de> for JsonResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MaybeResult {
            Void(String),
            Choice(JsonChoice),
        }

        match MaybeResult::deserialize(deserializer)? {
            MaybeResult::Void(str) if str == String::from("void") => Ok(JsonResult::Void),
            MaybeResult::Choice(choice) => Ok(JsonResult::Choice(choice)),
            MaybeResult::Void(variant) => {
                Err(de::Error::unknown_variant(&variant, &["void", "choice"]))
            }
        }
    }
}

impl Serialize for JsonResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JsonResult::Void => String::serialize(&String::from("void"), serializer),
            JsonResult::Choice(choice) => JsonChoice::serialize(&choice, serializer),
        }
    }
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
