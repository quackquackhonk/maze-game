use clap::ValueEnum;
use common::tile::CompassDirection;
use common::PubPlayerInfo;
use common::State;
use common::{board::Slide, grid::squared_euclidian_distance, grid::Position};
use itertools::Itertools;
use std::cmp::Ordering;

/// This trait represents getting a valid move from a given board state
pub trait Strategy {
    /// This returns a valid move given the game state
    fn get_move(
        &self,
        state: State<PubPlayerInfo>,
        start: Position,
        goal_tile: Position,
    ) -> PlayerAction;
}

/// This type represents a possible player action  
/// `None` -> A pass  
/// `Some(PlayerMove)` -> A move  
pub type PlayerAction = Option<PlayerMove>;

/// This type represents all the data needed to execute a move
///
/// # Warning
/// This type does not self-validate because it has no knowledge of the board it will be played on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerMove {
    pub slide: Slide,
    pub rotations: usize,
    pub destination: Position,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
/// Implements a strategy that after failing to find a move directly to the goal tile, checks
/// every other board position as a location to move. The order in which it checks every location
/// depends on the `NaiveStrategy` type.
pub enum NaiveStrategy {
    /// This variant sorts the posssible alternative goals in order of smallest to largest
    /// euclidian distance. It breaks any ties by picking the first one in row-column order.
    Euclid,
    /// This variant sorts the posssible alternative goals in order of row-column order.
    Riemann,
}

fn row_col_order(p1: &Position, p2: &Position) -> Ordering {
    if p1.1 < p2.1 {
        Ordering::Less
    } else if p1.1 > p2.1 {
        Ordering::Greater
    } else if p1.0 < p2.0 {
        Ordering::Less
    } else if p1.0 > p2.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl NaiveStrategy {
    /// This function creates a list of possible goals and orders them according to the strategy
    /// and returns a player action with the move if it found one or a pass if it couldn't
    fn find_move_to_reach_alt_goal(
        &self,
        state: &State<PubPlayerInfo>,
        start: Position,
        goal_tile: Position,
    ) -> PlayerAction {
        self.get_alt_goals(goal_tile, state)
            .into_iter()
            .find_map(|goal| self.find_move_to_reach(state, start, goal))
    }

    /// Returns a `Vec<Position>` containing alternative goals to try and reach
    /// sorted by how desireable they are according to their algorithm.
    /// - `NaiveStrategy::Euclid` sorts alt goals by ascending `euclidian_distance` to the
    /// `goal_tile`
    /// - `NaiveStrategy::Reimann` sorts alt goals in row-column order.
    fn get_alt_goals(
        &self,
        goal_tile: Position,
        board_state: &State<PubPlayerInfo>,
    ) -> Vec<Position> {
        //! alternative_goal_order is a Comparator<Position> function.
        #[allow(clippy::type_complexity)]
        let alternative_goal_order: Box<dyn Fn(&Position, &Position) -> Ordering> = match self {
            Self::Euclid => Box::new(|p1: &Position, p2: &Position| -> Ordering {
                let euclid1 = squared_euclidian_distance(p1, &goal_tile);
                let euclid2 = squared_euclidian_distance(p2, &goal_tile);
                match euclid1.cmp(&euclid2) {
                    Ordering::Equal => row_col_order(p1, p2),
                    rest => rest,
                }
            }),
            Self::Riemann => Box::new(row_col_order),
        };

        let mut possible_goals: Vec<Position> = (0..board_state.board.num_rows())
            .cartesian_product(0..board_state.board.num_cols())
            .collect();
        possible_goals.sort_by(alternative_goal_order);
        possible_goals
    }

    fn find_move_to_reach_helper<const N: usize>(
        &self,
        state: &State<PubPlayerInfo>,
        lines: impl Iterator<Item = usize>,
        directions: [CompassDirection; N],
        start: Position,
        destination: Position,
    ) -> PlayerAction {
        for line in lines {
            for direction in directions {
                for rotations in 0..4 {
                    if let Some(lslide) = state.previous_slide {
                        if lslide.index == line && lslide.direction.opposite() == direction {
                            continue;
                        }
                    }
                    let slide = state.board.new_slide(line, direction).unwrap();
                    if state.reachable_after_move(slide, rotations, destination, start) {
                        return Some(PlayerMove {
                            slide,
                            rotations,
                            destination,
                        });
                    }
                }
            }
        }
        None
    }

    fn find_move_to_reach(
        &self,
        state: &State<PubPlayerInfo>,
        start: Position,
        destination: Position,
    ) -> PlayerAction {
        self.find_move_to_reach_helper(
            state,
            state.board.slideable_rows(),
            [CompassDirection::West, CompassDirection::East],
            start,
            destination,
        )
        .or_else(|| {
            self.find_move_to_reach_helper(
                state,
                state.board.slideable_cols(),
                [CompassDirection::North, CompassDirection::South],
                start,
                destination,
            )
        })
    }
}

impl Strategy for NaiveStrategy {
    fn get_move(
        &self,
        state: State<PubPlayerInfo>,
        start: Position,
        goal_tile: Position,
    ) -> PlayerAction {
        self.find_move_to_reach(&state, start, goal_tile)
            .or_else(|| self.find_move_to_reach_alt_goal(&state, start, goal_tile))
    }
}

#[cfg(test)]
mod strategy_tests {
    use self::itertools::Itertools;

    use super::*;
    use common::gem::Gem;
    use common::grid::Grid;
    use common::tile::{ConnectorShape, PathOrientation, Tile};
    use common::ColorName;
    use itertools;
    use CompassDirection::*;

    #[test]
    fn test_get_move_euclid() {
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (1, 1),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let euclid = NaiveStrategy::Euclid;
        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼

        let slide = state.board.new_slide(2, East).unwrap();
        // what will Euclid do to go from (1, 1) -> (1, 3)
        let euclid_move = euclid.get_move(state, (1, 1), (1, 3));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides row 2 east, inserts crossroads, goes to (1, 3)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (1, 3),
            }
        );

        // what will Euclid do to go from (0, 0) to (2, 3)?
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (0, 0),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let slide = state.board.new_slide(0, East).unwrap();
        let euclid_move = euclid.get_move(state, (0, 0), (2, 3));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides the top row east, moves to (2, 2)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (2, 2),
            }
        );

        // what will Euclid do to go from (6, 4) to (2, 0)?
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (6, 4),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let slide = state.board.new_slide(4, East).unwrap();
        let euclid_move = euclid.get_move(state, (6, 4), (2, 0));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides row 4 east to wrap around to (0, 4) then move to (1, 2)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (1, 2),
            }
        );

        // there are no moves that will pass starting from (0, 0) on this board
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (1, 1),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let slide = state.board.new_slide(4, East).unwrap();
        let euclid_move = euclid.get_move(state, (6, 4), (2, 0));
        assert!(euclid_move.is_some());
        let euclid_move = euclid_move.unwrap();
        // slides row 4 east to wrap around to (0, 4) then move to (1, 2)
        assert_eq!(
            euclid_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (1, 2),
            }
        );
    }

    #[test]
    fn test_get_move_pass() {
        let euclid = NaiveStrategy::Euclid;
        let riemann = NaiveStrategy::Riemann;
        let mut state: State<PubPlayerInfo> = State {
            player_info: vec![PubPlayerInfo {
                current: (0, 2),
                home: (3, 3),
                color: ColorName::Red.into(),
            }]
            .into(),
            ..Default::default()
        };
        let mut idx = 0;
        let horizontal = ConnectorShape::Path(PathOrientation::Horizontal);
        let vertical = ConnectorShape::Path(PathOrientation::Vertical);
        state.board.grid = Grid::from([[(); 7]; 7].map(|list| {
            list.map(|_| {
                let tile = Tile {
                    connector: vertical,
                    gems: Gem::pair_from_num(idx),
                };
                idx += 1;
                tile
            })
        }));
        state.board.extra = Tile {
            connector: vertical,
            gems: (Gem::Zircon, Gem::Zoisite).into(),
        };
        state.previous_slide = state.board.new_slide(2, East);
        state.board.grid[(0, 1)].connector = horizontal;
        state.board.grid[(0, 3)].connector = horizontal;
        state.board.grid[(1, 1)].connector = horizontal;
        state.board.grid[(1, 3)].connector = horizontal;
        // Board is:
        //   0123456
        // 0 │││││││
        // 1 ──│││││
        // 2 │││││││
        // 3 ──│││││
        // 4 │││││││
        // 5 │││││││
        // 6 │││││││
        //
        // extra = │
        //
        // last slide: row 2 ->
        // both euclid and riemann will pass trying to
        // go from (0, 2) -> (3, 1)
        assert_eq!(euclid.get_move(state.clone(), (0, 2), (3, 1)), None);
        assert_eq!(riemann.get_move(state, (0, 2), (3, 1)), None);
    }

    #[test]
    fn test_get_move_reimann() {
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (1, 1),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let reimann = NaiveStrategy::Riemann;
        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼

        let slide = state.board.new_slide(2, East).unwrap();
        // what will Reimann do to go from (1, 1) -> (1, 3)
        let reimann_move = reimann.get_move(state, (1, 1), (1, 3));
        assert!(reimann_move.is_some());
        let reimann_move = reimann_move.unwrap();
        // slides row 2 east, inserts crossroads, goes to (1, 3)
        assert_eq!(
            reimann_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (1, 3),
            }
        );

        // what will Reimann do to go from (0, 0) to (2, 3)?
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (0, 0),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let slide = state.board.new_slide(0, East).unwrap();
        let reimann_move = reimann.get_move(state, (0, 0), (2, 3));
        assert!(reimann_move.is_some());
        let reimann_move = reimann_move.unwrap();
        // slides the top row east, moves to (0, 0)
        assert_eq!(
            reimann_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (0, 0),
            }
        );

        // what will Reimann do to go from (6, 4) to (2, 0)?
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (6, 4),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        let slide = state.board.new_slide(4, East).unwrap();
        let reimann_move = reimann.get_move(state.clone(), (6, 4), (2, 0));
        assert!(reimann_move.is_some());
        let reimann_move = reimann_move.unwrap();
        // slides row 4 east to wrap around to (0, 4) then move to (0, 2)
        assert_eq!(
            reimann_move,
            PlayerMove {
                slide,
                rotations: 0,
                destination: (0, 2),
            }
        );

        let mut any_passes = (0..state.board.num_rows())
            .cartesian_product(0..state.board.num_cols())
            .map(|dest| reimann.get_move(state.clone(), (3, 3), dest))
            .filter(|m| m.is_none());
        assert!(any_passes.next().is_none());
    }

    #[test]
    fn test_find_move_to_reach_alt_goal() {
        let state: State<PubPlayerInfo> = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (0, 2),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };

        let euclid = NaiveStrategy::Euclid;
        let reimann = NaiveStrategy::Riemann;
        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼

        // if Euclid is on (0, 2) and its goal is (0, 0), it will slide the leftmost column North
        // and then move to (0, 1)
        let euc_move = euclid.find_move_to_reach_alt_goal(&state, (0, 2), (0, 0));
        assert_eq!(
            euc_move,
            Some(PlayerMove {
                slide: state.board.new_slide(0, North).unwrap(),
                rotations: 0,
                destination: (1, 1)
            })
        );
        // With the same conditions, reimann is going to make the same move
        let rei_move = reimann.find_move_to_reach_alt_goal(&state, (0, 2), (0, 0));
        assert_eq!(
            rei_move,
            Some(PlayerMove {
                slide: state.board.new_slide(0, North).unwrap(),
                rotations: 0,
                destination: (1, 1)
            })
        );
        // what does Euclid do if on (3, 3) and its goal is (3, 2)?
        // Euclid will Slide the 2nd row West, and then move up to (3, 2) to avoid staying in place
        let euc_move = euclid.find_move_to_reach_alt_goal(&state, (3, 3), (2, 3));
        assert_eq!(
            euc_move,
            Some(PlayerMove {
                slide: state.board.new_slide(2, West).unwrap(),
                rotations: 0,
                destination: (3, 2)
            })
        );
        // Reimann will make the same slide but will move all the way up to (3, 0)
        let rei_move = reimann.find_move_to_reach_alt_goal(&state, (3, 3), (2, 3));
        assert_eq!(
            rei_move,
            Some(PlayerMove {
                slide: state.board.new_slide(2, West).unwrap(),
                rotations: 0,
                destination: (3, 0)
            })
        );

        // What if you start on (6, 6) and your goal is (0, 5)
        // Euclid will slide the bottom row east and move to (1,5)
        let euc_move = euclid.find_move_to_reach_alt_goal(&state, (6, 6), (0, 5));
        assert_eq!(
            euc_move,
            Some(PlayerMove {
                slide: state.board.new_slide(6, East).unwrap(),
                rotations: 0,
                destination: (1, 5)
            })
        );
        // Reimann will slide the last column down and move to (6, 1)
        let rei_move = reimann.find_move_to_reach_alt_goal(&state, (6, 6), (0, 5));
        assert_eq!(
            rei_move,
            Some(PlayerMove {
                slide: state.board.new_slide(6, South).unwrap(),
                rotations: 0,
                destination: (6, 1)
            })
        );
    }

    #[test]
    fn test_euclidian_distance() {
        let p_0_0 = (0, 0);
        let p_0_3 = (0, 3);
        let p_3_3 = (3, 3);
        let p_6_3 = (6, 3);
        let p_6_6 = (6, 6);
        assert_eq!(squared_euclidian_distance(&p_0_3, &p_0_3), 0);
        assert_eq!(squared_euclidian_distance(&p_0_3, &p_3_3), 9);
        assert_eq!(squared_euclidian_distance(&p_0_3, &p_6_3), 36);
        assert_eq!(squared_euclidian_distance(&p_0_0, &p_6_6), 72);
    }

    #[test]
    fn test_get_alt_goals_reimann() {
        let state: State<PubPlayerInfo> = State::default();
        let reimann_alt_goals = NaiveStrategy::Riemann.get_alt_goals((1, 1), &state);
        let max_cells = state.board.num_rows() * state.board.num_cols();
        assert_eq!(reimann_alt_goals.len(), max_cells);
        assert_eq!(reimann_alt_goals[0], (0, 0));
        assert_eq!(reimann_alt_goals[1], (1, 0));
        assert_eq!(reimann_alt_goals[2], (2, 0));
        assert_eq!(reimann_alt_goals[max_cells - 2], (5, 6));
        assert_eq!(reimann_alt_goals[max_cells - 1], (6, 6));
    }

    #[test]
    fn test_get_alt_goals_euclid() {
        let state = State::<PubPlayerInfo>::default();
        let euclid_alt_goals = NaiveStrategy::Euclid.get_alt_goals((1, 1), &state);
        let max_cells = state.board.num_rows() * state.board.num_cols();
        assert_eq!(euclid_alt_goals.len(), max_cells);
        assert_eq!(euclid_alt_goals[0], (1, 1));
        assert_eq!(euclid_alt_goals[1], (1, 0));
        assert_eq!(euclid_alt_goals[2], (0, 1));
        assert_eq!(euclid_alt_goals[3], (2, 1));
        assert_eq!(euclid_alt_goals[4], (1, 2));
        assert_eq!(euclid_alt_goals[max_cells - 2], (5, 6));
        assert_eq!(euclid_alt_goals[max_cells - 1], (6, 6));
    }

    #[test]
    fn test_find_move_to_reach() {
        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (4, 1),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        // Default Board<7> is:
        //   0123456
        // 0 ─│└┌┐┘┴
        // 1 ├┬┤┼─│└
        // 2 ┌┐┘┴├┬┤
        // 3 ┼─│└┌┐┘
        // 4 ┴├┬┤┼─│
        // 5 └┌┐┘┴├┬
        // 6 ┤┼─│└┌┐
        //
        // extra = ┼
        let euclid = NaiveStrategy::Euclid;
        let reimann = NaiveStrategy::Riemann;
        let start = state.player_info[0].current;
        let destination = (0, 1);
        assert_eq!(
            euclid.find_move_to_reach(&state, start, destination),
            Some(PlayerMove {
                slide: state.board.new_slide(0, West).unwrap(),
                rotations: 0,
                destination: (0, 1),
            })
        );
        assert_eq!(
            reimann.find_move_to_reach(&state, start, destination),
            Some(PlayerMove {
                slide: state.board.new_slide(0, West).unwrap(),
                rotations: 0,
                destination: (0, 1),
            })
        );

        // no move will take you from (4, 1) -> (2, 3)
        let destination = (2, 3);
        assert_eq!(euclid.find_move_to_reach(&state, start, destination), None);
        assert_eq!(reimann.find_move_to_reach(&state, start, destination), None);

        let state = State {
            player_info: vec![
                PubPlayerInfo {
                    current: (6, 0),
                    home: (1, 1),
                    color: ColorName::Red.into(),
                },
                PubPlayerInfo {
                    current: (2, 2),
                    home: (3, 1),
                    color: ColorName::Purple.into(),
                },
            ]
            .into(),
            ..Default::default()
        };
        // you can go from (6, 0) -> (1, 1) by wrapping around the board
        let start = state.player_info[0].current;
        let destination = (1, 1);
        assert_eq!(
            euclid.find_move_to_reach(&state, start, destination),
            Some(PlayerMove {
                slide: state.board.new_slide(0, East).unwrap(),
                rotations: 0,
                destination: (1, 1)
            })
        );
        assert_eq!(
            reimann.find_move_to_reach(&state, start, destination),
            Some(PlayerMove {
                slide: state.board.new_slide(0, East).unwrap(),
                rotations: 0,
                destination: (1, 1)
            })
        )
    }
}
