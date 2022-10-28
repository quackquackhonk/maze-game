use crate::grid::{Grid, Position};
use crate::tile::{CompassDirection, ConnectorShape, PathOrientation, Tile};
use std::collections::HashSet;
use std::ops::Index;

type BoardError = String;

pub type BoardResult<T> = Result<T, BoardError>;

/// Describes one board for the game of Maze`.`com
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub grid: Grid<Tile>,
    pub extra: Tile,
}

impl Board {
    pub fn new(grid: impl Into<Grid<Tile>>, extra: Tile) -> Self {
        Board {
            grid: grid.into(),
            extra,
        }
    }

    /// Slides the given Slide struct command and inserts the spare tile in the location of the
    /// hole in the board. The dislodged tile becomes the new `spare_tile`.
    pub fn slide_and_insert(&mut self, Slide { index, direction }: Slide) -> BoardResult<()> {
        use CompassDirection::*;
        match direction {
            North => {
                if index > self.grid.len() {
                    return Err(format!("Row index: {} out of bounds", index));
                }
                let col_num = index;
                let row_num = self.grid.len() - 1;
                self.grid.rotate_up(col_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(col_num, row_num)]);
                Ok(())
            }
            South => {
                if index > self.grid.len() {
                    return Err(format!("Row index: {} out of bounds", index));
                }
                let col_num = index;
                self.grid.rotate_down(col_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(col_num, 0)]);
                Ok(())
            }
            East => {
                if index > self.grid[0].len() {
                    return Err(format!("Col index: {} out of bounds", index));
                }
                let row_num = index;
                self.grid.rotate_right(row_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(0, row_num)]);
                Ok(())
            }
            West => {
                if index > self.grid[0].len() {
                    return Err(format!("Col index: {} out of bounds", index));
                }
                let row_num = index;
                let col_num = self.grid[0].len() - 1;
                self.grid.rotate_left(row_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(col_num, row_num)]);
                Ok(())
            }
        }
    }

    /// Can you go from `from` to `to` in the given `dir`?
    fn connected_positions(&self, from: Position, to: Position, dir: CompassDirection) -> bool {
        self.grid[from].connected(&self.grid[to], dir)
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
            return Err("Out-of-bounds Position".to_string());
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
        self.extra.rotate();
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
    /// Default Board<7> is:
    /// ─│└┌┐┘┴
    /// ├┬┤┼─│└
    /// ┌┐┘┴├┬┤
    /// ┼─│└┌┐┘
    /// ┴├┬┤┼─│
    /// └┌┐┘┴├┬
    /// ┤┼─│└┌┐
    /// extra = ┼
    ///
    /// Default Board<3> is:  
    /// ─│└  
    /// ┌┐┘  
    /// ┴├┬  
    /// extra = ┼
    fn default_board() -> Self {
        use crate::gem::Gem::*;
        use CompassDirection::*;
        use ConnectorShape::*;
        use PathOrientation::*;
        let mut idx = -1;
        let grid = [[(); COLS]; ROWS].map(|list| {
            list.map(|_| {
                idx += 1;
                Tile {
                    connector: match idx % 11 {
                        0 => Path(Horizontal),
                        1 => Path(Vertical),
                        2 => Corner(North),
                        3 => Corner(East),
                        4 => Corner(South),
                        5 => Corner(West),
                        6 => Fork(North),
                        7 => Fork(East),
                        8 => Fork(South),
                        9 => Fork(West),
                        10 => Crossroads,
                        _ => unreachable!("usize % 11 is never > 10"),
                    },
                    gems: (amethyst, garnet).into(),
                }
            })
        });
        Self {
            grid: Grid::from(grid),
            extra: Tile {
                connector: Crossroads,
                gems: (amethyst, garnet).into(),
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

impl Slide {
    /// Attempts to create a slide command
    ///
    /// ```
    /// # use common::board::Slide;
    /// # use common::tile::CompassDirection;
    /// Slide::new(3, CompassDirection::North);
    /// ```
    pub fn new(index: usize, direction: CompassDirection) -> Slide {
        Self { index, direction }
    }

    #[must_use]
    pub fn is_valid_slide(&self, rows: usize, cols: usize) -> bool {
        match self.direction {
            CompassDirection::North | CompassDirection::South
                if self.index % 2 == 0 && self.index < rows =>
            {
                true
            }
            CompassDirection::East | CompassDirection::West
                if self.index % 2 == 0 && self.index < cols =>
            {
                true
            }
            _ => false,
        }
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
mod BoardTests {
    use super::*;
    use CompassDirection::*;
    use ConnectorShape::*;

    #[test]
    pub fn test_slide_new() {
        assert!(Slide::new(0, North).is_valid_slide(1, 1));
        assert!(!Slide::new(2, North).is_valid_slide(1, 1));

        assert!(Slide::new(0, South).is_valid_slide(7, 7));
        assert!(Slide::new(2, East).is_valid_slide(7, 7));
        assert!(!Slide::new(5, West).is_valid_slide(7, 7));
    }

    #[test]
    fn test_slide_move_position() {
        let north_slide = Slide::new(0, North);
        assert_eq!(north_slide.move_position((1, 1), 7, 7), (1, 1));
        assert_eq!(north_slide.move_position((0, 0), 7, 7), (0, 6));
        assert_eq!(north_slide.move_position((0, 3), 7, 7), (0, 2));
        let south_slide = Slide::new(4, South);
        assert_eq!(south_slide.move_position((5, 1), 7, 7), (5, 1));
        assert_eq!(south_slide.move_position((4, 0), 7, 7), (4, 1));
        assert_eq!(south_slide.move_position((4, 6), 7, 7), (4, 0));
        let east_slide = Slide::new(2, East);
        assert_eq!(east_slide.move_position((5, 1), 7, 7), (5, 1));
        assert_eq!(east_slide.move_position((1, 2), 7, 7), (2, 2));
        assert_eq!(east_slide.move_position((6, 2), 7, 7), (0, 2));
        let west_slide = Slide::new(6, West);
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
        assert_eq!(b.extra.connector, Crossroads);

        b.slide_and_insert(Slide::new(0, South));
        // Board after slide + insert
        // ┼│└
        // ─┐┘
        // ┌├┬
        // extra = ┴
        assert_eq!(b.grid[(0, 0)].connector, Crossroads);
        dbg!(&b.grid);
        assert_eq!(b.extra.connector, Fork(North));

        b.slide_and_insert(Slide::new(0, East));
        // Board after insert
        // ┴┼│
        // ─┐┘
        // ┌├┬
        // extra = └
        assert_eq!(b.grid[(0, 0)].connector, Fork(North));
        assert_eq!(b.extra.connector, Corner(North));

        b.slide_and_insert(Slide::new(2, West));
        dbg!(&b);
        // Board after slide + insert
        // ┴┼│
        // ─┐┘
        // ├┬└
        // extra = ┌
        assert_eq!(b.grid[(2, 2)].connector, Corner(North));
        assert_eq!(b.extra.connector, Corner(East));
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
