//! Contains JSON definitions for data in the `player` module

use common::board::Slide;
use common::json::{Coordinate, Index, JsonDegree, JsonDirection};
use common::BOARD_SIZE;
use serde::Deserialize;

use crate::strategy::{NaiveStrategy, PlayerAction, PlayerMove};

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
#[derive(Debug, Deserialize)]
pub enum JsonChoice {
    #[serde(rename(deserialize = "PASS"))]
    Pass,
    Move(Index, JsonDirection, JsonDegree, Coordinate),
}

impl From<JsonChoice> for PlayerAction<BOARD_SIZE, BOARD_SIZE> {
    fn from(jc: JsonChoice) -> Self {
        match jc {
            JsonChoice::Pass => None,
            JsonChoice::Move(ind, dir, deg, coord) => Some(PlayerMove {
                slide: Slide::<BOARD_SIZE, BOARD_SIZE>::new(ind.0, dir.into()).unwrap(),
                rotations: deg.into(),
                destination: coord.into(),
            }),
        }
    }
}
