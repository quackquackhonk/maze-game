use anyhow::anyhow;
use common::{
    board::Board,
    grid::Position,
    json::Name,
    state::{PlayerInfo, State},
};
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

const TIMEOUT: Duration = Duration::from_secs(4);

impl PlayerProxy<TcpStream, TcpStream> {
    pub fn try_from_tcp(name: Name, stream: TcpStream) -> io::Result<Self> {
        stream.set_read_timeout(Some(TIMEOUT))?;
        let out = RefCell::new(stream.try_clone()?);
        let deser = serde_json::Deserializer::from_reader(stream);
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
    fn name(&self) -> Name {
        self.name.clone()
    }

    fn propose_board0(&self, _cols: u32, _rows: u32) -> PlayerApiResult<Board> {
        // the spec doesn't say anything about calling propose_board0 on `PlayerProxy`s
        todo!()
    }

    fn setup(&mut self, state: Option<State<PlayerInfo>>, goal: Position) -> PlayerApiResult<()> {
        // create function call message
        self.send_function_call(&JsonFunctionCall::setup(state, goal))?;
        match self.read_result()? {
            JsonResult::Void => Ok(()),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than \"void\", when calling `setup`!"
            ))),
        }
    }

    fn take_turn(&self, state: State<PlayerInfo>) -> PlayerApiResult<PlayerAction> {
        self.send_function_call(&JsonFunctionCall::take_turn(state.clone()))?;
        match self.read_result()? {
            JsonResult::Choice(ch) => Ok(ch.try_into_action(&state.board)?),
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

#[cfg(test)]
mod tests {

    use std::ops::Deref;

    use common::{
        board::Slide,
        color::ColorName,
        json::{Coordinate, Index, JsonDegree, JsonDirection},
        tile::CompassDirection,
    };
    use players::{json::JsonChoice, strategy::PlayerMove};
    use serde_json::json;

    use super::*;

    impl<In: Read + Send, Out: Write + Send> PlayerProxy<In, Out> {
        fn get_output(&self) -> impl Deref<Target = Out> + '_ {
            self.out.borrow()
        }
    }

    #[test]
    fn test_name() {
        let player = PlayerProxy::new(Name::from_static("john"), "".as_bytes(), Vec::new());

        assert_eq!(player.name(), Name::from_static("john"));
    }

    #[test]
    fn test_setup() {
        let mut player = PlayerProxy::new(Name::from_static("joe"), "\"void\"".as_bytes(), vec![]);

        player.setup(None, (1, 2)).expect("Should not error");
        assert_eq!(
            serde_json::to_string(&JsonFunctionCall::setup(None, (1, 2)))
                .unwrap()
                .as_bytes(),
            &*player.get_output()
        );
        assert_eq!(
            serde_json::to_value(&JsonFunctionCall::setup(None, (1, 2))).unwrap(),
            json!(["setup", [false, { "row#":2, "column#": 1 }]])
        );

        let mut player = PlayerProxy::new(Name::from_static("joe"), "\"void\"".as_bytes(), vec![]);
        let state = State {
            player_info: vec![PlayerInfo {
                current: (0, 0),
                home: (1, 1),
                color: ColorName::Red.into(),
            }]
            .into(),
            ..Default::default()
        };
        player
            .setup(Some(state.clone()), (3, 4))
            .expect("Setting up should not error");

        assert_eq!(
            serde_json::to_string(&JsonFunctionCall::setup(Some(state.clone()), (3, 4)))
                .unwrap()
                .as_bytes(),
            &*player.get_output()
        );
        assert_eq!(
            serde_json::to_value(&JsonFunctionCall::setup(Some(state), (3, 4))).unwrap(),
            json!(["setup", [
                {
                    "board": {
                        "connectors": [
                            [ "─", "│", "└", "┌", "┐", "┘", "┴" ],
                            [ "├", "┬", "┤", "┼", "─", "│", "└" ],
                            [ "┌", "┐", "┘", "┴", "├", "┬", "┤" ],
                            [ "┼", "─", "│", "└", "┌", "┐", "┘" ],
                            [ "┴", "├", "┬", "┤", "┼", "─", "│" ],
                            [ "└", "┌", "┐", "┘", "┴", "├", "┬" ],
                            [ "┤", "┼", "─", "│", "└", "┌", "┐" ] 
                        ],
                        "treasures": [
                            [ [ "alexandrite-pear-shape", "alexandrite-pear-shape" ], [ "alexandrite-pear-shape", "alexandrite" ], [ "alexandrite-pear-shape", "almandine-garnet" ], [ "alexandrite-pear-shape", "amethyst" ], [ "alexandrite-pear-shape", "ametrine" ], [ "alexandrite-pear-shape", "ammolite" ], [ "alexandrite-pear-shape", "apatite" ] ],
                            [ [ "alexandrite-pear-shape", "aplite" ], [ "alexandrite-pear-shape", "apricot-square-radiant" ], [ "alexandrite-pear-shape", "aquamarine" ], [ "alexandrite-pear-shape", "australian-marquise" ], [ "alexandrite-pear-shape", "aventurine" ], [ "alexandrite-pear-shape", "azurite" ], [ "alexandrite-pear-shape", "beryl" ] ],
                            [ [ "alexandrite-pear-shape", "black-obsidian" ], [ "alexandrite-pear-shape", "black-onyx" ], [ "alexandrite-pear-shape", "black-spinel-cushion" ], [ "alexandrite-pear-shape", "blue-ceylon-sapphire" ], [ "alexandrite-pear-shape", "blue-cushion" ], [ "alexandrite-pear-shape", "blue-pear-shape" ], [ "alexandrite-pear-shape", "blue-spinel-heart" ] ],
                            [ [ "alexandrite-pear-shape", "bulls-eye" ], [ "alexandrite-pear-shape", "carnelian" ], [ "alexandrite-pear-shape", "chrome-diopside" ], [ "alexandrite-pear-shape", "chrysoberyl-cushion" ], [ "alexandrite-pear-shape", "chrysolite" ], [ "alexandrite-pear-shape", "citrine-checkerboard" ], [ "alexandrite-pear-shape", "citrine" ] ],
                            [ [ "alexandrite-pear-shape", "clinohumite" ], [ "alexandrite-pear-shape", "color-change-oval" ], [ "alexandrite-pear-shape", "cordierite" ], [ "alexandrite-pear-shape", "diamond" ], [ "alexandrite-pear-shape", "dumortierite" ], [ "alexandrite-pear-shape", "emerald" ], [ "alexandrite-pear-shape", "fancy-spinel-marquise" ] ],
                            [ [ "alexandrite-pear-shape", "garnet" ], [ "alexandrite-pear-shape", "golden-diamond-cut" ], [ "alexandrite-pear-shape", "goldstone" ], [ "alexandrite-pear-shape", "grandidierite" ], [ "alexandrite-pear-shape", "gray-agate" ], [ "alexandrite-pear-shape", "green-aventurine" ], [ "alexandrite-pear-shape", "green-beryl-antique" ] ],
                            [ [ "alexandrite-pear-shape", "green-beryl" ], [ "alexandrite-pear-shape", "green-princess-cut" ], [ "alexandrite-pear-shape", "grossular-garnet" ], [ "alexandrite-pear-shape", "hackmanite" ], [ "alexandrite-pear-shape", "heliotrope" ], [ "alexandrite-pear-shape", "hematite" ], [ "alexandrite-pear-shape", "iolite-emerald-cut" ] ]
                        ]
                    },
                    "spare": {
                        "tilekey": "┼",
                        "1-image": "yellow-heart",
                        "2-image": "yellow-jasper"
                    },
                    "plmt": [
                        {
                            "current": {"row#": 0, "column#": 0},
                            "home": {"row#": 1, "column#": 1},
                            "color": "red"
                        }
                    ],
                    "last": null
                },
                { "row#":4, "column#": 3 }]])
        );

        // test no response
        let mut player = PlayerProxy::new(Name::from_static("joe"), "".as_bytes(), vec![]);
        assert!(player.setup(None, (0, 0)).is_err());

        // test wrong response
        let mut player = PlayerProxy::new(Name::from_static("joe"), "wrong".as_bytes(), vec![]);
        assert!(player.setup(None, (0, 0)).is_err());
    }

    #[test]
    fn test_take_turn() {
        let choice = serde_json::to_string(&JsonChoice::Move(
            Index(0),
            JsonDirection::UP,
            JsonDegree(90),
            Coordinate {
                row: Index(2),
                column: Index(3),
            },
        ))
        .unwrap();
        let player = PlayerProxy::new(Name::from_static("joe"), choice.as_bytes(), vec![]);
        let state = State::default();

        let r#move = player.take_turn(state).unwrap();
        assert_eq!(
            r#move,
            Some(PlayerMove {
                slide: Slide {
                    index: 0,
                    direction: CompassDirection::North
                },
                rotations: 1,
                destination: (3, 2)
            })
        );

        let choice = serde_json::to_string(&JsonResult::Choice(JsonChoice::Pass)).unwrap();
        dbg!(&choice);
        let player = PlayerProxy::new(Name::from_static("joe"), choice.as_bytes(), vec![]);
        let state = State::default();

        let r#move = player.take_turn(state).unwrap();
        assert_eq!(r#move, None);

        // test no response
        let player = PlayerProxy::new(Name::from_static("joe"), "".as_bytes(), vec![]);
        assert!(player.take_turn(State::default()).is_err());

        // test wrong response
        let player = PlayerProxy::new(Name::from_static("joe"), "wrong".as_bytes(), vec![]);
        assert!(player.take_turn(State::default()).is_err());
    }

    #[test]
    fn test_win() {
        let mut player = PlayerProxy::new(Name::from_static("joe"), "\"void\"".as_bytes(), vec![]);

        player.won(false).expect("Sending win should not fail");
        assert_eq!(
            serde_json::to_string(&JsonFunctionCall::win(false))
                .unwrap()
                .as_bytes(),
            &*player.get_output()
        );
        assert_eq!(
            serde_json::to_value(&JsonFunctionCall::win(false)).unwrap(),
            json!(["win", [false]])
        );

        let mut player = PlayerProxy::new(Name::from_static("joe"), "\"void\"".as_bytes(), vec![]);
        player.won(true).expect("Sending win should not fail");
        assert_eq!(
            serde_json::to_string(&JsonFunctionCall::win(true))
                .unwrap()
                .as_bytes(),
            &*player.get_output()
        );
        assert_eq!(
            serde_json::to_value(&JsonFunctionCall::win(true)).unwrap(),
            json!(["win", [true]])
        );

        // test no response
        let mut player = PlayerProxy::new(Name::from_static("joe"), "".as_bytes(), vec![]);
        assert!(player.won(true).is_err());

        // test wrong response
        let mut player = PlayerProxy::new(Name::from_static("joe"), "wrong".as_bytes(), vec![]);
        assert!(player.won(true).is_err());
    }
}
