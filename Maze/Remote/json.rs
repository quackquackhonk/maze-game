use anyhow::anyhow;
use common::{
    grid::Position,
    json::{Coordinate, JsonState},
    state::{PlayerInfo, State},
};
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

impl From<Option<State<PlayerInfo>>> for JsonArguments {
    fn from(st: Option<State<PlayerInfo>>) -> Self {
        match st {
            Some(state) => JsonArguments::State(state.into()),
            None => JsonArguments::Boolean(false),
        }
    }
}

impl From<State<PlayerInfo>> for JsonArguments {
    fn from(st: State<PlayerInfo>) -> Self {
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
            Choice(JsonChoice),
            Void(String), // This must go second!!! otherwise "\"PASS\"" is serialized like void
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
    use common::json::{Index, JsonDegree, JsonDirection};
    let mut deserializer = serde_json::Deserializer::from_str(r#""void""#).into_iter();
    assert!(matches!(
        deserializer.next().unwrap().unwrap(),
        JsonResult::Void
    ));

    let mut deserializer =
        serde_json::Deserializer::from_str(r#"[1, "LEFT", 90, { "row#": 0, "column#": 0 }]"#)
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

    let mut deserializer = serde_json::Deserializer::from_str(r#""PASS""#).into_iter();
    let r#move = deserializer.next().unwrap().unwrap();
    dbg!(&r#move);
    assert!(matches!(r#move, JsonResult::Choice(JsonChoice::Pass)));

    assert_eq!(
        r#""void""#,
        &serde_json::to_string(&JsonResult::Void).unwrap()
    );
    assert_eq!(
        r#"[1,"LEFT",90,{"row#":0,"column#":0}]"#,
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
    pub fn get_state(&mut self) -> anyhow::Result<State<PlayerInfo>> {
        if let JsonArguments::State(s) =
            self.1.pop().ok_or_else(|| anyhow!("No more arguments!"))?
        {
            Ok(s.try_into()?)
        } else {
            Err(anyhow!("Last argument is not a State!"))
        }
    }
    pub fn get_option_state(&mut self) -> anyhow::Result<Option<State<PlayerInfo>>> {
        match self.1.pop().ok_or_else(|| anyhow!("No more arguments!"))? {
            JsonArguments::State(s) => Ok(Some(s.try_into()?)),
            JsonArguments::Boolean(b) if !b => Ok(None),
            _ => Err(anyhow!("Last argument is not a Option<State>")),
        }
    }
    pub fn get_goal(&mut self) -> anyhow::Result<Position> {
        if let JsonArguments::Coordinate(c) =
            self.1.pop().ok_or_else(|| anyhow!("No more arguments!"))?
        {
            Ok(c.into())
        } else {
            Err(anyhow!("Last argument is not a Coordinate!"))
        }
    }
    pub fn get_won(&mut self) -> anyhow::Result<bool> {
        if let JsonArguments::Boolean(b) =
            self.1.pop().ok_or_else(|| anyhow!("No more arguments!"))?
        {
            Ok(b)
        } else {
            Err(anyhow!("Last argument is not a boolean!"))
        }
    }

    pub fn setup(state: Option<State<PlayerInfo>>, goal: Position) -> Self {
        Self(JsonMName::Setup, vec![state.into(), goal.into()])
    }

    pub fn take_turn(state: State<PlayerInfo>) -> Self {
        Self(JsonMName::TakeTurn, vec![Some(state).into()])
    }

    pub fn win(did_win: bool) -> Self {
        Self(JsonMName::Win, vec![did_win.into()])
    }
}
