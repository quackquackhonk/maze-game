use common::{
    board::Board,
    grid::Position,
    json::{
        has_unique_elements, Coordinate, JsonAction, JsonBoard, JsonColor, JsonError, JsonTile,
        Name,
    },
    FullPlayerInfo, PlayerInfo, PrivatePlayerInfo, State,
};
use players::{bad_player::BadFM, player::PlayerApi, strategy::NaiveStrategy};
use serde::{Deserialize, Serialize};

use crate::referee::GameResult;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct PS(Name, JsonStrategy);

impl From<PS> for (Name, NaiveStrategy) {
    fn from(ps: PS) -> Self {
        (ps.0, ps.1.into())
    }
}

#[test]
fn ps_parse_test() {
    assert_eq!(
        serde_json::from_str::<PS>("[\"bob\", \"Riemann\"]").unwrap(),
        PS(Name::from_static("bob"), JsonStrategy::Riemann)
    );
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BadPS(Name, JsonStrategy, BadFM);

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct BadPS2(Name, JsonStrategy, BadFM, u64);

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayerSpec {
    PS(PS),
    BadPS(BadPS),
    BadPS2(BadPS2),
}

impl From<BadPS> for (Name, NaiveStrategy, BadFM) {
    fn from(bad_ps: BadPS) -> Self {
        (bad_ps.0, bad_ps.1.into(), bad_ps.2)
    }
}

impl From<BadPS2> for (Name, NaiveStrategy, BadFM, u64) {
    fn from(bad_ps2: BadPS2) -> Self {
        (bad_ps2.0, bad_ps2.1.into(), bad_ps2.2, bad_ps2.3)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum JsonStrategy {
    Riemann,
    Euclid,
}

impl From<JsonStrategy> for NaiveStrategy {
    fn from(jss: JsonStrategy) -> Self {
        match jss {
            JsonStrategy::Riemann => NaiveStrategy::Riemann,
            JsonStrategy::Euclid => NaiveStrategy::Euclid,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRefereeState {
    board: JsonBoard,
    spare: JsonTile,
    plmt: Vec<JsonRefereePlayer>,
    last: JsonAction,
}

fn valid_positions(
    positions: Vec<Position>,
    valid: Vec<Position>,
    board: &Board,
    unique_error: JsonError,
) -> Result<(), JsonError> {
    if positions.iter().any(|home| !valid.contains(home)) {
        return Err(unique_error);
    }

    positions
        .iter()
        .fold(Ok(()), |acc: Result<(), JsonError>, home| {
            board
                .in_bounds(home)
                .then_some(())
                .ok_or(JsonError::PositionOutOfBounds(*home))?;
            acc
        })?;
    Ok(())
}

impl<PI: PrivatePlayerInfo> TryFrom<JsonRefereeState> for State<PI>
where
    PI: TryFrom<JsonRefereePlayer, Error = JsonError>,
{
    type Error = JsonError;

    fn try_from(jstate: JsonRefereeState) -> Result<Self, Self::Error> {
        let board: Board = (jstate.board, jstate.spare).try_into()?;

        let player_info: Vec<PI> = jstate
            .plmt
            .into_iter()
            .map(|pi| pi.try_into())
            .collect::<Result<_, JsonError>>()?;

        let colors = player_info.iter().map(|pi| pi.color());
        if !has_unique_elements(colors) {
            return Err(JsonError::NonUniqueColors);
        }

        let homes = player_info.iter().map(|pi| pi.home()).collect::<Vec<_>>();
        if !has_unique_elements(&homes) {
            return Err(JsonError::NonUniqueHomes);
        }

        valid_positions(
            homes,
            board.possible_homes().collect(),
            &board,
            JsonError::GoalMoveableTile,
        )?;

        let goals = player_info.iter().map(|pi| pi.goal()).collect::<Vec<_>>();

        valid_positions(
            goals,
            board.possible_goals().collect(),
            &board,
            JsonError::GoalMoveableTile,
        )?;

        let previous_slide = jstate.last.into();
        if let Some(slide) = previous_slide {
            if !board.valid_slide(slide) {
                return Err(JsonError::InvalidSlide(slide));
            }
        }

        Ok(Self {
            board,
            player_info: player_info.into(),
            previous_slide,
        })
    }
}

impl From<State<FullPlayerInfo>> for JsonRefereeState {
    fn from(st: State<FullPlayerInfo>) -> Self {
        let (board, spare) = st.board.into();
        JsonRefereeState {
            board,
            spare,
            plmt: st.player_info.into_iter().map(|pi| pi.into()).collect(),
            last: st.previous_slide.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRefereePlayer {
    current: Coordinate,
    home: Coordinate,
    goto: Coordinate,
    color: JsonColor,
}

impl TryFrom<JsonRefereePlayer> for FullPlayerInfo {
    type Error = JsonError;

    fn try_from(jrp: JsonRefereePlayer) -> Result<Self, Self::Error> {
        Ok(Self::new(
            jrp.home.into(),
            jrp.current.into(),
            jrp.goto.into(),
            jrp.color.try_into()?,
        ))
    }
}

impl From<FullPlayerInfo> for JsonRefereePlayer {
    fn from(pi: FullPlayerInfo) -> Self {
        JsonRefereePlayer {
            current: pi.position().into(),
            home: pi.home().into(),
            goto: pi.goal.into(),
            color: pi.color().into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JsonGameResult(Vec<Name>, Vec<Name>);

impl From<GameResult> for JsonGameResult {
    fn from(gr: GameResult) -> Self {
        JsonGameResult(
            gr.winners.into_iter().flat_map(|p| p.name()).collect(),
            gr.kicked.into_iter().flat_map(|p| p.name()).collect(),
        )
    }
}
