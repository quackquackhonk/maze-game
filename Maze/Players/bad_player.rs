use std::{thread, time::Duration};

use anyhow::anyhow;
use common::{board::Board, grid::Position, json::Name, PubPlayerInfo, State};
use serde::Deserialize;

use crate::{
    player::{PlayerApi, PlayerApiResult},
    strategy::PlayerAction,
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
    fn name(&self) -> PlayerApiResult<Name> {
        self.player.name()
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board> {
        self.player.propose_board0(cols, rows)
    }

    fn setup(
        &mut self,
        state: Option<State<PubPlayerInfo>>,
        goal: Position,
    ) -> PlayerApiResult<()> {
        if let BadFM::SetUp = self.bad_fm {
            let _ = 1_i32
                .checked_div(0)
                .ok_or_else(|| anyhow!("tried to divide by 0"))?;
        }
        self.player.setup(state, goal)
    }

    fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
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

struct BadPlayerLoop {
    badfm: BadFM,
    times: u64,
    times_called: u64,
    api: Box<dyn PlayerApi>,
}

impl BadPlayerLoop {
    fn new(api: Box<dyn PlayerApi>, badfm: BadFM, times: u64) -> Self {
        Self {
            badfm,
            times,
            api,
            times_called: 0,
        }
    }
}

impl BadPlayerLoop {
    /// If times_called
    fn inc_or_loop(&self) {
        if self.times_called + 1 == self.times {
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

impl PlayerApi for BadPlayerLoop {
    fn name(&self) -> PlayerApiResult<Name> {
        self.api.name()
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board> {
        self.api.propose_board0(cols, rows)
    }

    fn setup(
        &mut self,
        state: Option<State<PubPlayerInfo>>,
        goal: Position,
    ) -> PlayerApiResult<()> {
        if let BadFM::SetUp = self.badfm {
            self.inc_or_loop();
        }
        self.api.setup(state, goal)
    }

    fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
        if let BadFM::TakeTurn = self.badfm {
            self.inc_or_loop();
        }
        self.api.take_turn(state)
    }

    fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
        if let BadFM::Win = self.badfm {
            self.inc_or_loop();
        }
        self.api.won(did_win)
    }
}
