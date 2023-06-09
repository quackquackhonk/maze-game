use std::{
    io::{Read, Write},
    net::TcpStream,
    result::Result::Ok,
};

use anyhow::anyhow;
use players::player::PlayerApi;
use serde::Deserialize;
use serde_json::de::IoRead;

use crate::json::{JsonFunctionCall, JsonMName, JsonResult};

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
    pub fn new(player: Box<dyn PlayerApi>, r#in: In, out: Out) -> Self {
        Self {
            player,
            out,
            r#in: serde_json::Deserializer::from_reader(r#in),
        }
    }

    /// Listens for `JsonFunctionCall`s on `self.r#in` until `self.r#in` is closed.
    ///
    /// When the RefereeProxy gets a `JsonFunctionCall`, it calls the corresponding method on
    /// `self.player`, and writes the result of the that call to `self.out`
    pub fn receive_commands(&mut self) -> anyhow::Result<()> {
        while let Ok(mut command) = JsonFunctionCall::deserialize(&mut self.r#in) {
            match command.0 {
                JsonMName::Setup => {
                    if command.1.len() != 2 {
                        return Err(anyhow!("Not enough arguments for `setup`!"));
                    }
                    let goal = command.get_goal()?;
                    let state = command.get_option_state()?;
                    self.player.setup(state, goal)?;
                    self.out
                        .write_all(serde_json::to_string(&JsonResult::Void)?.as_bytes())?;
                }
                JsonMName::TakeTurn => {
                    if command.1.len() != 1 {
                        return Err(anyhow!("Not enough arguments for `take_turn`!"));
                    }
                    let state = command.get_state()?;
                    let choice = self.player.take_turn(state)?;
                    self.out.write_all(
                        serde_json::to_string(&JsonResult::Choice(choice.into()))?.as_bytes(),
                    )?;
                }
                JsonMName::Win => {
                    if command.1.len() != 1 {
                        return Err(anyhow!("Not enough arguments for `win`!"));
                    }
                    let did_win = command.get_won()?;
                    self.player.won(did_win)?;
                    self.out
                        .write_all(serde_json::to_string(&JsonResult::Void)?.as_bytes())?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::{
        color::ColorName,
        json::Name,
        state::{PlayerInfo, State},
    };
    use players::{player::LocalPlayer, strategy::NaiveStrategy};

    use crate::json::JsonFunctionCall;

    use super::RefereeProxy;

    #[test]
    fn test_listen() {
        let player = Box::new(LocalPlayer::new(
            Name::from_static("bob"),
            NaiveStrategy::Riemann,
        ));
        let state = State {
            player_info: vec![PlayerInfo {
                current: (0, 1),
                home: (1, 1),
                color: ColorName::Red.into(),
            }]
            .into(),
            ..Default::default()
        };
        let setup_cmd = JsonFunctionCall::setup(Some(state.clone()), (3, 1));
        let take_turn = JsonFunctionCall::take_turn(state);
        let home_setup_cmd = JsonFunctionCall::setup(None, (1, 1));
        let win_cmd = JsonFunctionCall::win(true);

        let mut commands = String::new();
        commands.push_str(&serde_json::to_string(&setup_cmd).unwrap());
        commands.push_str(&serde_json::to_string(&take_turn).unwrap());
        commands.push_str(&serde_json::to_string(&home_setup_cmd).unwrap());
        commands.push_str(&serde_json::to_string(&take_turn).unwrap());
        commands.push_str(&serde_json::to_string(&win_cmd).unwrap());

        let referee_output = String::from(r#""void""#)
            + r#"[0,"LEFT",0,{"row#":1,"column#":3}]"#
            + r#""void""#
            + r#"[0,"LEFT",0,{"row#":1,"column#":1}]"#
            + r#""void""#;
        let mut ref_proxy = RefereeProxy::new(player, commands.as_bytes(), vec![]);
        assert!(ref_proxy.receive_commands().is_ok());
        let ref_out = String::from_utf8(ref_proxy.out);
        assert!(ref_out.is_ok());
        assert_eq!(ref_out.unwrap(), referee_output);
    }

    #[test]
    fn test_listen_none() {
        let player = Box::new(LocalPlayer::new(
            Name::from_static("bob"),
            NaiveStrategy::Riemann,
        ));
        let mut ref_proxy = RefereeProxy::new(player, "".as_bytes(), vec![]);
        assert!(ref_proxy.receive_commands().is_ok());
        assert!(ref_proxy.out.is_empty());
        assert!(ref_proxy.r#in.end().is_ok());
    }

    #[test]
    fn test_listen_fail() {
        let player = Box::new(LocalPlayer::new(
            Name::from_static("bob"),
            NaiveStrategy::Riemann,
        ));

        // invalid win call
        let mut ref_proxy = RefereeProxy::new(player, "[\"win\", []]".as_bytes(), vec![]);
        assert!(ref_proxy.receive_commands().is_err());

        let player = Box::new(LocalPlayer::new(
            Name::from_static("bob"),
            NaiveStrategy::Riemann,
        ));
        // invalid take-turn call
        let mut ref_proxy = RefereeProxy::new(player, "[\"take-turn\", []]".as_bytes(), vec![]);
        assert!(ref_proxy.receive_commands().is_err());

        let player = Box::new(LocalPlayer::new(
            Name::from_static("bob"),
            NaiveStrategy::Riemann,
        ));
        // invalid setup call
        let mut ref_proxy = RefereeProxy::new(
            player,
            r#"["setup", [{"row#": 1, "column#": 0}]]"#.as_bytes(),
            vec![],
        );
        assert!(ref_proxy.receive_commands().is_err());

        let player = Box::new(LocalPlayer::new(
            Name::from_static("bob"),
            NaiveStrategy::Riemann,
        ));
        // invalid setup call again!
        let mut ref_proxy = RefereeProxy::new(
            player,
            r#"["setup", [{"row#": 1, "column#": 0}, false]]"#.as_bytes(),
            vec![],
        );
        assert!(ref_proxy.receive_commands().is_err());
    }
}
