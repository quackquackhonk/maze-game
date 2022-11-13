use anyhow::anyhow;
use common::{board::Board, PubPlayerInfo, State};
use serde::Deserialize;

use crate::{
    player::{PlayerApi, PlayerApiResult},
    strategy::PlayerMove,
};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BadFM {
    SetUp,
    TakeTurn,
    Win,
}

pub struct BadPlayer {
    bad_fm: BadFM,
    player: Box<dyn PlayerApi>,
}

impl BadPlayer {
    pub fn new(player: Box<dyn PlayerApi>, bad_fm: BadFM) -> Self {
        Self { bad_fm, player }
    }
}

impl PlayerApi for BadPlayer {
    fn name(&self) -> PlayerApiResult<common::json::Name> {
        self.player.name()
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board> {
        self.player.propose_board0(cols, rows)
    }

    fn setup(
        &mut self,
        state: Option<State<PubPlayerInfo>>,
        goal: common::grid::Position,
    ) -> PlayerApiResult<()> {
        if let BadFM::SetUp = self.bad_fm {
            let _ = 1_i32
                .checked_div(0)
                .ok_or_else(|| anyhow!("tried to divide by 0"))?;
        }
        self.player.setup(state, goal)
    }

    fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<Option<PlayerMove>> {
        if let BadFM::TakeTurn = self.bad_fm {
            let _ = 1_i32
                .checked_div(0)
                .ok_or_else(|| anyhow!("tried to divide by 0"))?;
        }
        self.player.take_turn(state)
    }

    fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
        if let BadFM::Win = self.bad_fm {
            let _ = 1_i32
                .checked_div(0)
                .ok_or_else(|| anyhow!("tried to divide by 0"))?;
        }
        self.player.won(did_win)
    }
}
