use crate::referee::GameResult;
use common::{
    board::Board,
    color::Color,
    grid::Position,
    json::{
        has_unique_elements, Coordinate, JsonAction, JsonBoard, JsonColor, JsonError, JsonTile,
        Name,
    },
    state::{FullPlayerInfo, PrivatePlayerInfo, PublicPlayerInfo, State},
};
use players::{bad_player::BadFM, player::PlayerApi, strategy::NaiveStrategy};
use serde::{Deserialize, Serialize};

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
    goals: Option<Vec<Coordinate>>,
}

fn valid_positions(
    player_info: Vec<(Color, Position)>,
    valid: impl AsRef<Vec<Position>>,
    board: &Board,
    unique_error: impl FnOnce(Vec<Color>) -> JsonError,
) -> Result<(), JsonError> {
    let valid = valid.as_ref();
    let invalid = player_info
        .iter()
        .filter(|(_, position)| !valid.contains(position))
        .map(|(color, _)| color)
        .collect::<Vec<_>>();
    if !invalid.is_empty() {
        return Err(unique_error(invalid.into_iter().cloned().collect()));
    }

    player_info
        .iter()
        .fold(Ok(()), |acc: Result<(), JsonError>, (_, position)| {
            board
                .in_bounds(position)
                .then_some(())
                .ok_or_else(|| JsonError::PositionOutOfBounds(vec![*position]))?;
            acc
        })?;
    Ok(())
}

impl<PI: PrivatePlayerInfo> TryFrom<JsonRefereeState> for (State<PI>, Vec<Position>)
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

        let out_of_bounds = player_info
            .iter()
            .map(|pi| pi.position())
            .filter(|pos| !board.in_bounds(pos))
            .collect::<Vec<_>>();
        if !out_of_bounds.is_empty() {
            return Err(JsonError::PositionOutOfBounds(out_of_bounds));
        }

        let colors = player_info.iter().map(|pi| pi.color());
        if !has_unique_elements(colors) {
            return Err(JsonError::NonUniqueColors);
        }

        let homes = player_info.iter().map(|pi| pi.home()).collect::<Vec<_>>();
        if !has_unique_elements(&homes) {
            return Err(JsonError::NonUniqueHomes);
        }

        if board.possible_homes().count() < player_info.len() {
            // not enough homes for players
            return Err(JsonError::NotEnoughHomes);
        }

        let homes_and_colors = player_info
            .iter()
            .map(|pi| (pi.color(), pi.home()))
            .collect();

        valid_positions(
            homes_and_colors,
            board.possible_homes().collect::<Vec<_>>(),
            &board,
            JsonError::HomeMoveableTile,
        )?;

        let rem_goals: Vec<Position> = jstate
            .goals
            .unwrap_or_default()
            .into_iter()
            .map(|c| c.into())
            .collect();

        let possible_goals = board.possible_goals().collect::<Vec<_>>();
        let invalid_alt_goals = rem_goals
            .iter()
            .filter(|goal| !possible_goals.contains(goal))
            .collect::<Vec<_>>();
        if !invalid_alt_goals.is_empty() {
            return Err(JsonError::GoalMoveableTile(
                invalid_alt_goals.into_iter().cloned().collect(),
            ));
        }

        let goals_and_colors = player_info
            .iter()
            .map(|pi| (pi.color(), pi.goal()))
            .collect();

        valid_positions(
            goals_and_colors,
            possible_goals,
            &board,
            JsonError::PlayerGoalMoveableTile,
        )?;

        let previous_slide = jstate.last.into();
        if let Some(slide) = previous_slide {
            if !board.valid_slide(slide) {
                return Err(JsonError::InvalidSlide(slide));
            }
        }

        Ok((
            State {
                board,
                player_info: player_info.into(),
                previous_slide,
            },
            rem_goals,
        ))
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
            goals: None,
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
            gr.winners.into_iter().map(|p| p.name()).collect(),
            gr.kicked.into_iter().map(|p| p.name()).collect(),
        )
    }
}
