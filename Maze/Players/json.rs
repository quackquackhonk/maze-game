//! Contains JSON definitions for data in the `player` module

use common::board::Board;
use common::json::{Coordinate, Index, JsonDegree, JsonDirection, JsonError};
use common::tile::CompassDirection;
use serde::ser::SerializeTuple;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::strategy::{NaiveStrategy, PlayerAction, PlayerMove};

/// Describes either a `Reimann` or a `Euclid` strategy
#[derive(Debug, Deserialize)]
pub enum JsonStrategyDesignation {
    Riemann,
    Euclid,
}

impl From<JsonStrategyDesignation> for NaiveStrategy {
    fn from(jsd: JsonStrategyDesignation) -> Self {
        match jsd {
            JsonStrategyDesignation::Riemann => NaiveStrategy::Riemann,
            JsonStrategyDesignation::Euclid => NaiveStrategy::Euclid,
        }
    }
}

/// Describes a choice a player can make for their action
/// A `Pass` is a player passing their turn
/// A `Move` contains the `Index` of the row/col being slid, the `JsonDirection` of the slide, a
/// number of `JsonDegree`s to rotate the spare tile counter-clockwise, and the destination
/// `Coordinate` that the player is moving to.
#[derive(Debug)]
pub enum JsonChoice {
    Pass,
    Move(Index, JsonDirection, JsonDegree, Coordinate),
}
impl<'de> Deserialize<'de> for JsonChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MaybeChoice {
            Pass(String),
            Move(Index, JsonDirection, JsonDegree, Coordinate),
        }

        let value = MaybeChoice::deserialize(deserializer)?;
        match value {
            MaybeChoice::Pass(str) if *"PASS" == str => Ok(JsonChoice::Pass),
            MaybeChoice::Move(index, direction, degree, coordinate) => {
                Ok(JsonChoice::Move(index, direction, degree, coordinate))
            }
            MaybeChoice::Pass(value) => Err(de::Error::unknown_variant(&value, &["PASS", "Move"])),
        }
    }
}

impl Serialize for JsonChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JsonChoice::Pass => Ok(String::serialize(&String::from("PASS"), serializer)?),
            JsonChoice::Move(index, direction, degree, coordinate) => {
                let mut tup = serializer.serialize_tuple(4)?;
                tup.serialize_element(index)?;
                tup.serialize_element(direction)?;
                tup.serialize_element(degree)?;
                tup.serialize_element(coordinate)?;
                tup.end()
            }
        }
    }
}

#[test]
fn test_json_choice() {
    let mut deserializer = serde_json::Deserializer::from_str("\"PASS\"").into_iter();
    assert!(matches!(
        deserializer.next().unwrap().unwrap(),
        JsonChoice::Pass
    ));

    let mut deserializer =
        serde_json::Deserializer::from_str("[1, \"LEFT\", 90, { \"row#\": 0, \"column#\": 0 }]")
            .into_iter();
    let r#move = deserializer.next().unwrap().unwrap();
    dbg!(&r#move);
    assert!(matches!(
        r#move,
        JsonChoice::Move(
            Index(1),
            JsonDirection::LEFT,
            JsonDegree(90),
            Coordinate {
                row: Index(0),
                column: Index(0)
            }
        )
    ));

    assert_eq!(
        "\"PASS\"",
        &serde_json::to_string(&JsonChoice::Pass).unwrap()
    );
    assert_eq!(
        "[1,\"LEFT\",90,{\"row#\":0,\"column#\":0}]",
        &serde_json::to_string(&JsonChoice::Move(
            Index(1),
            JsonDirection::LEFT,
            JsonDegree(90),
            Coordinate {
                row: Index(0),
                column: Index(0)
            }
        ))
        .unwrap()
    );
}

impl JsonChoice {
    pub fn into_action(self, board: &Board) -> Result<PlayerAction, JsonError> {
        match self {
            JsonChoice::Pass => Ok(None),
            JsonChoice::Move(index, direction, rotations, destination) => Ok(Some(PlayerMove {
                slide: board
                    .new_slide(index.0, direction.into())
                    .ok_or(JsonError {
                        msg: format!(
                            "Slide row/col {} with direction {:?} is not a slidable row/col",
                            index.0,
                            CompassDirection::from(direction),
                        ),
                    })?,
                rotations: rotations.try_into()?,
                destination: destination.into(),
            })),
        }
    }
}

impl From<PlayerAction> for JsonChoice {
    fn from(pa: PlayerAction) -> Self {
        match pa {
            None => JsonChoice::Pass,
            Some(PlayerMove {
                slide,
                rotations,
                destination,
            }) => JsonChoice::Move(
                Index(slide.index),
                slide.direction.into(),
                JsonDegree(rotations * 90),
                destination.into(),
            ),
        }
    }
}
