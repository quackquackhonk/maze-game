use common::grid::Position;
use common::json::{Coordinate, Index, JsonDegree, JsonDirection, JsonState};
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonArguments {
    State(JsonState),
    Coordinate(Coordinate),
    Boolean(bool),
}

impl From<Option<State<PubPlayerInfo>>> for JsonArguments {
    fn from(st: Option<State<PubPlayerInfo>>) -> Self {
        match st {
            Some(state) => JsonArguments::State(state.into()),
            None => JsonArguments::Boolean(false),
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
            MaybeResult::Void(str) if str == *"void" => Ok(JsonResult::Void),
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
            JsonResult::Choice(choice) => JsonChoice::serialize(choice, serializer),
        }
    }
}

#[test]
fn test_parse_json_result() {
    let mut deserializer = serde_json::Deserializer::from_str("\"void\"").into_iter();
    assert!(matches!(
        deserializer.next().unwrap().unwrap(),
        JsonResult::Void
    ));

    let mut deserializer =
        serde_json::Deserializer::from_str("[1, \"LEFT\", 90, { \"row#\": 0, \"column#\": 0 }]")
            .into_iter();
    let r#move = deserializer.next().unwrap().unwrap();
    dbg!(&r#move);
    assert!(matches!(
        r#move,
        JsonResult::Choice(JsonChoice::Move(
            Index(1),
            JsonDirection::LEFT,
            JsonDegree(90),
            Coordinate {
                row: Index(0),
                column: Index(0)
            }
        ))
    ));

    assert_eq!(
        "\"void\"",
        &serde_json::to_string(&JsonResult::Void).unwrap()
    );
    assert_eq!(
        "[1,\"LEFT\",90,{\"row#\":0,\"column#\":0}]",
        &serde_json::to_string(&JsonResult::Choice(JsonChoice::Move(
            Index(1),
            JsonDirection::LEFT,
            JsonDegree(90),
            Coordinate {
                row: Index(0),
                column: Index(0)
            }
        )))
        .unwrap()
    );
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
