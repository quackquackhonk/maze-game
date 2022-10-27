//! Contains JSON definitions for data in the `player` module

use common::board::Slide;
use common::json::{Coordinate, Index, JsonDegree, JsonDirection, JsonPlayer, JsonState};
use serde::{Deserialize, Serialize};

use crate::strategy::{NaiveStrategy, PlayerAction, PlayerBoardState, PlayerMove, PubPlayerInfo};

/// Describes either a `Reimann` or a `Euclid` strategy
#[derive(Debug, Deserialize)]
pub enum JsonStrategyDesignation {
    Reimann,
    Euclid,
}

impl From<JsonStrategyDesignation> for NaiveStrategy {
    fn from(jsd: JsonStrategyDesignation) -> Self {
        match jsd {
            JsonStrategyDesignation::Reimann => NaiveStrategy::Reimann,
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
    #[serde(rename(deserialize = "PASS"))]
    Pass,
    Move(Index, JsonDirection, JsonDegree, Coordinate),
}

impl From<JsonChoice> for PlayerAction {
    fn from(jc: JsonChoice) -> Self {
        match jc {
            JsonChoice::Pass => None,
            JsonChoice::Move(ind, dir, deg, coord) => Some(PlayerMove {
                slide: Slide::new(ind.0, dir.into()),
                rotations: deg.try_into().unwrap(),
                destination: coord.into(),
            }),
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

impl TryFrom<JsonPlayer> for PubPlayerInfo {
    type Error = String;
    fn try_from(jp: JsonPlayer) -> Result<Self, Self::Error> {
        Ok(Self {
            current: jp.current.into(),
            home: jp.home.into(),
            color: jp.color.try_into()?,
        })
    }
}

impl From<JsonState> for PlayerBoardState {
    fn from(js: JsonState) -> Self {
        Self {
            board: (js.board, js.spare).into(),
            players: js
                .plmt
                .into_iter()
                .map(|player| player.try_into())
                .collect::<Result<Vec<PubPlayerInfo>, String>>()
                .unwrap(),
            last: js.last.into(),
        }
    }
}
