//! Contains JSON definitions for data in the `player` module

use common::board::Slide;
use common::json::{Coordinate, Index, JsonDegree, JsonDirection, JsonState};
use common::PubPlayerInfo;
use serde::{Deserialize, Serialize};

use crate::strategy::{NaiveStrategy, PlayerAction, PlayerBoardState, PlayerMove};

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
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonChoice {
    #[serde(rename = "PASS")]
    Pass,
    Move(Index, JsonDirection, JsonDegree, Coordinate),
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

impl From<JsonState> for PlayerBoardState {
    fn from(js: JsonState) -> Self {
        Self {
            board: (js.board, js.spare).into(),
            players: js
                .plmt
                .into_iter()
                .map(|player| player.into())
                .collect::<Vec<PubPlayerInfo>>(),
            last: js.last.into(),
        }
    }
}
