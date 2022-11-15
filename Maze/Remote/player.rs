use anyhow::anyhow;
use common::{board::Board, grid::Position, json::Name, PubPlayerInfo, State};
use players::{
    player::{PlayerApi, PlayerApiError, PlayerApiResult},
    strategy::PlayerAction,
};
use serde::Deserialize;
use serde_json::de::IoRead;
use std::{
    cell::RefCell,
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
};

use crate::json::{JsonFunctionCall, JsonResult};

/// Acts as a proxy for players across a network
pub struct PlayerProxy<In: Read + Send, Out: Write + Send> {
    name: Name,
    r#in: RefCell<serde_json::Deserializer<IoRead<In>>>,
    out: RefCell<Out>,
}

impl PlayerProxy<TcpStream, TcpStream> {
    pub fn from_tcp(name: Name, stream: TcpStream) -> Self {
        // TODO: what should this timeout actually be?
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .expect("The timeout is not zero");
        let out = RefCell::new(stream.try_clone().unwrap());
        let r#in = RefCell::new(serde_json::Deserializer::from_reader(stream));
        Self { name, out, r#in }
    }

    pub fn try_from_tcp(stream: TcpStream) -> io::Result<Self> {
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        let out = RefCell::new(stream.try_clone()?);
        let mut deser = serde_json::Deserializer::from_reader(stream);
        let name = Name::deserialize(&mut deser)?;
        let r#in = RefCell::new(deser);
        Ok(Self { name, out, r#in })
    }
}

impl<In: Read + Send, Out: Write + Send> PlayerProxy<In, Out> {
    pub fn new(name: Name, r#in: In, out: Out) -> Self {
        Self {
            name,
            out: RefCell::new(out),
            r#in: RefCell::new(serde_json::Deserializer::from_reader(r#in)),
        }
    }

    /// Reads a single `JsonResult` from `self.stream`
    ///
    /// # Errors
    /// This will error if reading from the stream or deserializing the `JsonResult` fails
    fn read_result(&self) -> PlayerApiResult<JsonResult> {
        Ok(JsonResult::deserialize(&mut *self.r#in.borrow_mut())?)
    }

    /// Writes a `JsonFunctionCall` to `self.stream`
    ///
    /// # Errors
    /// This will error if writing to `self.stream` or serializing `func` fails
    fn send_function_call(&self, func: &JsonFunctionCall) -> PlayerApiResult<()> {
        let msg = serde_json::to_string(func)?;
        self.out.borrow_mut().write_all(msg.as_bytes())?;
        Ok(())
    }
}

impl<In: Read + Send, Out: Write + Send> PlayerApi for PlayerProxy<In, Out> {
    fn name(&self) -> PlayerApiResult<Name> {
        Ok(self.name.clone())
    }

    fn propose_board0(&self, _cols: u32, _rows: u32) -> PlayerApiResult<Board> {
        // the spec doesn't say anything about calling propose_board0 on `PlayerProxy`s
        todo!()
    }

    fn setup(
        &mut self,
        state: Option<State<PubPlayerInfo>>,
        goal: Position,
    ) -> PlayerApiResult<()> {
        // create function call message
        self.send_function_call(&JsonFunctionCall::setup(state, goal))?;
        match self.read_result()? {
            JsonResult::Void => Ok(()),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than \"void\", when calling `setup`!"
            ))),
        }
    }

    fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
        self.send_function_call(&JsonFunctionCall::take_turn(state.clone()))?;
        match self.read_result()? {
            JsonResult::Choice(ch) => Ok(ch.into_action(&state.board)?),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than a JsonChoice when calling `take_turn`!"
            ))),
        }
    }

    fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
        self.send_function_call(&JsonFunctionCall::win(did_win))?;
        match self.read_result()? {
            JsonResult::Void => Ok(()),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than \"void\" when calling `won`!"
            ))),
        }
    }
}
