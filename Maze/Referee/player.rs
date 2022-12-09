use std::{
    fmt::Debug,
    sync::{
        mpsc::{self, RecvTimeoutError},
        Arc,
    },
    thread,
    time::Duration,
};

use common::{
    board::Board,
    color::Color,
    grid::Position,
    json::Name,
    state::{FullPlayerInfo, PlayerInfo, PrivatePlayerInfo, PublicPlayerInfo, State},
};
use parking_lot::Mutex;
use players::{
    player::{PlayerApi, PlayerApiError, PlayerApiResult},
    strategy::PlayerAction,
};
use thiserror::Error;

#[derive(Clone)]
pub struct Player {
    pub api: Arc<Mutex<Box<dyn PlayerApi>>>,
    pub info: FullPlayerInfo,
    name: Name,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("name", &self.name)
            .field("info", &self.info)
            .finish()
    }
}

impl Player {
    pub fn new(api: Box<dyn PlayerApi>, info: FullPlayerInfo) -> Self {
        Player {
            name: api.name(),
            api: Arc::new(Mutex::new(api)),
            info,
        }
    }
}

impl PublicPlayerInfo for Player {
    fn position(&self) -> Position {
        self.info.position()
    }

    fn set_position(&mut self, dest: Position) {
        self.info.set_position(dest);
    }

    fn home(&self) -> Position {
        self.info.home()
    }

    fn reached_home(&self) -> bool {
        self.info.reached_home()
    }

    fn color(&self) -> Color {
        self.info.color()
    }
}

impl PrivatePlayerInfo for Player {
    fn reached_goal(&self) -> bool {
        self.info.reached_goal()
    }

    fn set_goal(&mut self, goal: Position) {
        self.info.set_goal(goal)
    }

    fn goal(&self) -> Position {
        self.info.goal()
    }

    fn get_goals_reached(&self) -> u64 {
        self.info.get_goals_reached()
    }

    fn inc_goals_reached(&mut self) {
        self.info.inc_goals_reached()
    }
}

const TIMEOUT: Duration = Duration::from_secs(4);

impl PlayerApi for Player {
    fn name(&self) -> Name {
        self.name.clone()
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board> {
        let api = self.api.clone();
        run_with_timeout(move || api.lock().propose_board0(cols, rows), TIMEOUT)?
    }

    fn setup(&mut self, state: Option<State<PlayerInfo>>, goal: Position) -> PlayerApiResult<()> {
        let api = self.api.clone();
        run_with_timeout(move || api.lock().setup(state, goal), TIMEOUT)?
    }

    fn take_turn(&self, state: State<PlayerInfo>) -> PlayerApiResult<PlayerAction> {
        let api = self.api.clone();
        run_with_timeout(move || api.lock().take_turn(state), TIMEOUT)?
    }

    fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
        let api = self.api.clone();
        run_with_timeout(move || api.lock().won(did_win), TIMEOUT)?
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.info.color() == other.info.color()
    }
}

impl PartialEq<Color> for Player {
    fn eq(&self, other: &Color) -> bool {
        &self.info.color() == other
    }
}

impl PartialEq<Player> for Color {
    fn eq(&self, other: &Player) -> bool {
        self == &other.color()
    }
}

impl PartialOrd<Player> for Player {
    fn partial_cmp(&self, other: &Player) -> Option<std::cmp::Ordering> {
        self.name().partial_cmp(&other.name())
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Player {}
#[derive(Debug, Error)]
#[error("Timed Out!")]
struct TimeoutError;

impl From<TimeoutError> for PlayerApiError {
    fn from(_: TimeoutError) -> Self {
        PlayerApiError::Timeout
    }
}

/// Runs `f` in a separate thread, and has the child thread send the result of `f` through a
/// channel. The main thread waits on the channel, and if no response is returned `timeout` passes,
/// returns an `Err`.
fn run_with_timeout<F, T>(f: F, timeout: Duration) -> Result<T, TimeoutError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let _ = thread::spawn(move || {
        let result = f();
        tx.send(result)
    });

    match rx.recv_timeout(timeout) {
        Ok(result) => Ok(result),
        Err(RecvTimeoutError::Timeout) => Err(TimeoutError),
        Err(RecvTimeoutError::Disconnected) => unreachable!(),
    }
}
