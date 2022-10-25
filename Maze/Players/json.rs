//! Contains JSON definitions for data in the `player` module

use common::board::Slide;
use common::json::{Coordinate, Index, JsonDegree, JsonDirection, JsonPlayer, JsonState};
use common::BOARD_SIZE;
use serde::Deserialize;

use crate::strategy::{NaiveStrategy, PlayerAction, PlayerBoardState, PlayerMove, PubPlayerInfo};

/// Describes either a `Reimann` or a `Euclid` strategy
#[derive(Debug, Deserialize)]
pub enum JsonStrategyDesignation {
    Reimann,
    Euclid,
}

impl From<JsonStrategyDesignation> for NaiveStrategy<BOARD_SIZE, BOARD_SIZE> {
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
#[serde(untagged)]
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

impl From<JsonState> for PlayerBoardState<BOARD_SIZE, BOARD_SIZE> {
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
