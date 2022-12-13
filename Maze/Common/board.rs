use itertools::Itertools;
use thiserror::Error;

use crate::gem::Gem;
use crate::grid::{Grid, Position};
use crate::tile::{CompassDirection, ConnectorShape, Tile};
use std::collections::HashSet;
use std::ops::Index;

#[derive(Debug, Error)]
pub enum OutOfBounds {
    #[error("{0} is out of bounds!")]
    Index(usize),
    #[error("{0:?} is out of bounds!")]
    Position(Position),
}

pub type BoardResult<T> = Result<T, OutOfBounds>;

/// Describes one board for the game of Maze`.`com
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub grid: Grid<Tile>,
    pub spare: Tile,
}

impl Board {
    pub fn new(grid: impl Into<Grid<Tile>>, spare: Tile) -> Self {
        Board {
            grid: grid.into(),
            spare,
        }
    }

    #[inline]
    pub fn num_rows(&self) -> usize {
        self.grid.len()
    }

    pub fn slideable_rows(&self) -> impl Iterator<Item = usize> {
        (0..self.num_rows()).step_by(2)
    }

    #[inline]
    pub fn num_cols(&self) -> usize {
        self.grid[0].len()
    }

    pub fn slideable_cols(&self) -> impl Iterator<Item = usize> {
        (0..self.num_cols()).step_by(2)
    }

    #[must_use]
    pub fn in_bounds(&self, pos: &Position) -> bool {
        (0..self.num_cols()).contains(&pos.0) && (0..self.num_rows()).contains(&pos.1)
    }

    pub fn possible_homes(&self) -> impl Iterator<Item = Position> {
        let slideable_cols = self.slideable_cols().collect::<Vec<_>>();
        let slideable_rows = self.slideable_cols().collect::<Vec<_>>();
        (0..self.num_cols())
            .cartesian_product(0..self.num_rows())
            .filter(move |(col, row)| {
                !slideable_cols.contains(col) && !slideable_rows.contains(row)
            })
    }

    pub fn possible_goals(&self) -> impl Iterator<Item = Position> {
        self.possible_homes()
    }

    /// Slides the given Slide struct command and inserts the spare tile in the location of the
    /// hole in the board. The dislodged tile becomes the new `spare_tile`.
    pub fn slide_and_insert(&mut self, Slide { index, direction }: Slide) -> BoardResult<()> {
        use CompassDirection::*;
        match direction {
            North => {
                if index > self.num_cols() {
                    return Err(OutOfBounds::Index(index));
                }
                let col_num = index;
                let row_num = self.grid.len() - 1;
                self.grid.rotate_up(col_num);
                std::mem::swap(&mut self.spare, &mut self.grid[(col_num, row_num)]);
                Ok(())
            }
            South => {
                if index > self.num_cols() {
                    return Err(OutOfBounds::Index(index));
                }
                let col_num = index;
                self.grid.rotate_down(col_num);
                std::mem::swap(&mut self.spare, &mut self.grid[(col_num, 0)]);
                Ok(())
            }
            East => {
                if index > self.num_rows() {
                    return Err(OutOfBounds::Index(index));
                }
                let row_num = index;
                self.grid.rotate_right(row_num);
                std::mem::swap(&mut self.spare, &mut self.grid[(0, row_num)]);
                Ok(())
            }
            West => {
                if index > self.num_rows() {
                    return Err(OutOfBounds::Index(index));
                }
                let row_num = index;
                let col_num = self.grid[0].len() - 1;
                self.grid.rotate_left(row_num);
                std::mem::swap(&mut self.spare, &mut self.grid[(col_num, row_num)]);
                Ok(())
            }
        }
    }

    /// Can you go from `from` to `to` in the given `dir`?
    fn connected_positions(&self, from: Position, to: Position, dir: CompassDirection) -> bool {
        Tile::connected(&self.grid[from], &self.grid[to], dir)
    }

    /// Returns a Vector of Positions representing all cells directly reachable from `start`
    fn reachable_neighbors(&self, start: Position) -> Vec<Position> {
        use CompassDirection::*;
        let mut neighbors = Vec::new();
        // north neighbor
        if start.1 > 0 && self.connected_positions(start, (start.0, start.1 - 1), North) {
            neighbors.push((start.0, start.1 - 1));
        }
        // east neighbor
        if start.0 < self.grid[0].len() - 1
            && self.connected_positions(start, (start.0 + 1, start.1), East)
        {
            neighbors.push((start.0 + 1, start.1));
        }
        // south neighbor
        if start.1 < self.grid.len() - 1
            && self.connected_positions(start, (start.0, start.1 + 1), South)
        {
            neighbors.push((start.0, start.1 + 1));
        }
        // west neighbor
        if start.0 > 0 && self.connected_positions(start, (start.0 - 1, start.1), West) {
            neighbors.push((start.0 - 1, start.1));
        }
        neighbors
    }

    /// Returns a Vector of Positions representing all cells on the Board reachable from `start`
    ///
    /// # Errors
    /// Returns an error if `start.0` > `BOARD_SIZE` or `start.1` > `BOARD_SIZE` or either `start.0` or
    /// `start.1` are negative.
    pub fn reachable(&self, start: Position) -> BoardResult<Vec<Position>> {
        if start.0 >= self.grid[0].len() || start.1 >= self.grid.len() {
            return Err(OutOfBounds::Position(start));
        }

        // push start onto worklist
        // reachable = []
        // visited = <>
        let mut worklist = vec![start];
        let mut reachable = HashSet::from([start]);
        let mut visited = HashSet::new();
        while let Some(curr) = worklist.pop() {
            let neighbors = self.reachable_neighbors(curr);
            let not_visited_neighbors = neighbors.into_iter().filter(|x| !visited.contains(x));
            not_visited_neighbors.for_each(|n| {
                reachable.insert(n);
                worklist.push(n);
            });
            //    push x onto visited
            visited.insert(curr);
        }

        Ok(reachable.into_iter().collect())
    }

    pub fn rotate_spare(&mut self) {
        self.spare.rotate();
    }
}

impl Index<Position> for Board {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index]
    }
}

pub trait DefaultBoard<const COLS: usize, const ROWS: usize> {
    fn default_board() -> Self;
}

impl<const COLS: usize, const ROWS: usize> DefaultBoard<COLS, ROWS> for Board {
    /// Default Board<7,7> is:
    /// ─│└┌┐┘┴
    /// ├┬┤┼─│└
    /// ┌┐┘┴├┬┤
    /// ┼─│└┌┐┘
    /// ┴├┬┤┼─│
    /// └┌┐┘┴├┬
    /// ┤┼─│└┌┐
    /// extra = ┼
    ///
    /// Default Board<3,3> is:  
    /// ─│└  
    /// ┌┐┘  
    /// ┴├┬  
    /// extra = ┼
    fn default_board() -> Self {
        use ConnectorShape::*;
        let mut idx = 0;
        let grid = [[(); COLS]; ROWS].map(|list| {
            list.map(|_| {
                let tile = Tile::from_num(idx);
                idx += 1;
                tile
            })
        });
        Self {
            grid: Grid::from(grid),
            spare: Tile {
                connector: Crossroads,
                gems: (Gem::from_num(idx * 2), Gem::from_num(idx * 2 + 1)).into(),
            },
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        DefaultBoard::<7, 7>::default_board()
    }
}

/// Describes a slide motion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slide {
    /// The index of the row or column to be slid
    /// Counts from 0 from left to right and top to bottom
    pub index: usize,
    /// The direction the row or column is sliding to
    pub direction: CompassDirection,
}

// For slide stuff
impl Board {
    /// Attempts to create a slide command
    ///
    /// ```
    /// use common::board::Slide;
    /// use common::tile::CompassDirection;
    /// use common::board::Board;
    /// use common::board::DefaultBoard;
    ///
    /// let board: Board = DefaultBoard::<7,7>::default_board();
    /// assert!(board.new_slide(3, CompassDirection::North).is_none());
    /// assert!(board.new_slide(4, CompassDirection::East).is_some());
    /// ```
    pub fn new_slide(&self, index: usize, direction: CompassDirection) -> Option<Slide> {
        let slide = Slide::new_unchecked(index, direction);
        self.valid_slide(slide).then_some(slide)
    }

    pub fn valid_slide(&self, Slide { index, direction }: Slide) -> bool {
        match direction {
            CompassDirection::North | CompassDirection::South
                if self.slideable_cols().contains(&index) =>
            {
                true
            }
            CompassDirection::East | CompassDirection::West
                if self.slideable_rows().contains(&index) =>
            {
                true
            }
            _ => false,
        }
    }
}

impl Slide {
    pub fn new_unchecked(index: usize, direction: CompassDirection) -> Slide {
        Self { index, direction }
    }

    #[must_use]
    pub fn move_position(&self, pos: Position, cols: usize, rows: usize) -> Position {
        use CompassDirection::*;
        match self {
            Slide {
                index,
                direction: North,
            } if *index == pos.0 => {
                if pos.1 == 0 {
                    (pos.0, rows - 1)
                } else {
                    (pos.0, pos.1 - 1)
                }
            }
            Slide {
                index,
                direction: South,
            } if *index == pos.0 => {
                if pos.1 == rows - 1 {
                    (pos.0, 0)
                } else {
                    (pos.0, pos.1 + 1)
                }
            }
            Slide {
                index,
                direction: East,
            } if *index == pos.1 => {
                if pos.0 == cols - 1 {
                    (0, pos.1)
                } else {
                    (pos.0 + 1, pos.1)
                }
            }
            Slide {
                index,
                direction: West,
            } if *index == pos.1 => {
                if pos.0 == 0 {
                    (cols - 1, pos.1)
                } else {
                    (pos.0 - 1, pos.1)
                }
            }
            _ => pos,
        }
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;
    use CompassDirection::*;
    use ConnectorShape::*;

    #[test]
    pub fn test_slide_new() {
        let one_by_one: Board = DefaultBoard::<1, 1>::default_board();
        assert!(one_by_one.new_slide(0, North).is_some());
        assert!(one_by_one.new_slide(1, North).is_none());

        let seven_by_seven: Board = DefaultBoard::<7, 7>::default_board();
        assert!(seven_by_seven.new_slide(0, South).is_some());
        assert!(seven_by_seven.new_slide(2, East).is_some());
        assert!(seven_by_seven.new_slide(5, West).is_none());
    }

    #[test]
    fn test_slide_move_position() {
        let b: Board = DefaultBoard::<7, 7>::default_board();
        let north_slide = b.new_slide(0, North).unwrap();
        assert_eq!(north_slide.move_position((1, 1), 7, 7), (1, 1));
        assert_eq!(north_slide.move_position((0, 0), 7, 7), (0, 6));
        assert_eq!(north_slide.move_position((0, 3), 7, 7), (0, 2));
        let south_slide = b.new_slide(4, South).unwrap();
        assert_eq!(south_slide.move_position((5, 1), 7, 7), (5, 1));
        assert_eq!(south_slide.move_position((4, 0), 7, 7), (4, 1));
        assert_eq!(south_slide.move_position((4, 6), 7, 7), (4, 0));
        let east_slide = b.new_slide(2, East).unwrap();
        assert_eq!(east_slide.move_position((5, 1), 7, 7), (5, 1));
        assert_eq!(east_slide.move_position((1, 2), 7, 7), (2, 2));
        assert_eq!(east_slide.move_position((6, 2), 7, 7), (0, 2));
        let west_slide = b.new_slide(6, West).unwrap();
        assert_eq!(west_slide.move_position((5, 1), 7, 7), (5, 1));
        assert_eq!(west_slide.move_position((0, 6), 7, 7), (6, 6));
        assert_eq!(west_slide.move_position((6, 6), 7, 7), (5, 6));
    }

    #[test]
    pub fn test_slide_and_insert() {
        // Initial Board state
        // ─│└
        // ┌┐┘
        // ┴├┬
        // extra = ┼
        let mut b: Board = DefaultBoard::<3, 3>::default_board();
        dbg!(&b.grid);
        assert_eq!(b.spare.connector, Crossroads);

        b.slide_and_insert(b.new_slide(0, South).unwrap()).unwrap();
        // Board after slide + insert
        // ┼│└
        // ─┐┘
        // ┌├┬
        // extra = ┴
        assert_eq!(b.grid[(0, 0)].connector, Crossroads);
        dbg!(&b.grid);
        assert_eq!(b.spare.connector, Fork(North));

        b.slide_and_insert(b.new_slide(0, East).unwrap()).unwrap();
        // Board after insert
        // ┴┼│
        // ─┐┘
        // ┌├┬
        // extra = └
        assert_eq!(b.grid[(0, 0)].connector, Fork(North));
        assert_eq!(b.spare.connector, Corner(North));

        b.slide_and_insert(b.new_slide(2, West).unwrap()).unwrap();
        dbg!(&b);
        // Board after slide + insert
        // ┴┼│
        // ─┐┘
        // ├┬└
        // extra = ┌
        assert_eq!(b.grid[(2, 2)].connector, Corner(North));
        assert_eq!(b.spare.connector, Corner(East));
    }

    #[test]
    pub fn test_reachable_from_position() {
        // Default Board<3> is:
        // ─│└
        // ┌┐┘
        // ┴├┬
        // extra = ┼
        let b: Board = DefaultBoard::<3, 3>::default_board();
        assert_eq!(b.reachable_neighbors((0, 0)), Vec::new());
        assert_eq!(b.reachable_neighbors((2, 2)), vec![(1, 2)]);
        assert_eq!(b.reachable_neighbors((0, 1)), vec![(1, 1), (0, 2)]);
        assert_eq!(b.reachable_neighbors((1, 2)), vec![(1, 1), (2, 2)]);
        assert_eq!(b.reachable_neighbors((0, 2)), vec![(0, 1)]);
    }

    #[test]
    pub fn test_reachable() {
        // Default Board<3> is:
        // ─│└
        // ┌┐┘
        // ┴├┬
        // extra = ┼
        let b: Board = DefaultBoard::<3, 3>::default_board();
        assert!(b.reachable((10, 10)).is_err());
        let from_0_0 = b.reachable((0, 0));
        assert!(from_0_0.is_ok());
        assert_eq!(from_0_0.unwrap().len(), 1);
        let from_2_2 = b.reachable((2, 2));
        assert!(from_2_2.is_ok());
        assert_eq!(from_2_2.unwrap().len(), 5);
    }
}
