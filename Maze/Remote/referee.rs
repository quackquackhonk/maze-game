use std::{
    io::{Read, Write},
    net::TcpStream,
    result::Result::Ok,
};

use anyhow::anyhow;
use players::player::PlayerApi;
use serde::Deserialize;
use serde_json::de::IoRead;

use crate::json::{JsonArguments, JsonFunctionCall, JsonMName, JsonResult};

pub struct RefereeProxy<In: Read, Out: Write> {
    player: Box<dyn PlayerApi>,
    r#in: serde_json::Deserializer<IoRead<In>>,
    out: Out,
}

impl RefereeProxy<TcpStream, TcpStream> {
    pub fn from_tcp(player: Box<dyn PlayerApi>, stream: TcpStream) -> Self {
        Self {
            player,
            r#in: serde_json::Deserializer::from_reader(stream.try_clone().unwrap()),
            out: stream,
        }
    }
}

impl<In: Read, Out: Write> RefereeProxy<In, Out> {
    pub fn listen(mut self) -> anyhow::Result<()> {
        // TODO: Send name when connecting to the server + connecting to the server
        while let Ok(mut command) = JsonFunctionCall::deserialize(&mut self.r#in) {
            match command.0 {
                JsonMName::Setup => {
                    let goal = match command.1.pop() {
                        Some(JsonArguments::Coordinate(coords)) => coords.into(),
                        _ => Err(anyhow!("Last argument was not a goal"))?,
                    };
                    let state = match command.1.pop() {
                        Some(JsonArguments::State(state)) => state.map(|s| s.into()),
                        _ => Err(anyhow!("First argument was not an Option<State>"))?,
                    };
                    self.player.setup(state, goal)?;
                    self.out
                        .write_all(serde_json::to_string(&JsonResult::Void)?.as_bytes())?;
                }
                JsonMName::TakeTurn => {
                    let state = match command.1.pop() {
                        Some(JsonArguments::State(Some(state))) => state.into(),
                        _ => Err(anyhow!("Did not recieve a state"))?,
                    };
                    let choice = self.player.take_turn(state)?;
                    self.out.write_all(
                        serde_json::to_string(&JsonResult::Choice(choice.into()))?.as_bytes(),
                    )?;
                }
                JsonMName::Win => {
                    let did_win = match command.1.pop() {
                        Some(JsonArguments::Boolean(bool)) => bool,
                        _ => Err(anyhow!("Did not recieve win condition"))?,
                    };
                    self.player.won(did_win)?;
                    self.out
                        .write_all(serde_json::to_string(&JsonResult::Void)?.as_bytes())?;
                }
            }
        }
        Ok(())
    }
}
