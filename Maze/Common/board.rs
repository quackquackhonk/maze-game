use crate::grid::{Grid, Position};
use crate::tile::{CompassDirection, ConnectorShape, PathOrientation, Tile};
use std::collections::HashSet;
use std::ops::Index;

type BoardError = String;

pub type BoardResult<T> = Result<T, BoardError>;

/// Describes one board for the game of Maze`.`com
#[derive(Debug)]
pub struct Board<const BOARD_SIZE: usize> {
    pub(crate) grid: Grid<Tile, BOARD_SIZE, BOARD_SIZE>,
    pub(crate) extra: Tile,
}

impl<const BOARD_SIZE: usize> Board<BOARD_SIZE> {
    pub fn new(grid: impl Into<Grid<Tile, BOARD_SIZE, BOARD_SIZE>>, extra: Tile) -> Self {
        Board {
            grid: grid.into(),
            extra,
        }
    }

    /// Slides the given Slide struct command and inserts the spare tile in the location of the
    /// hole in the board. The dislodged tile becomes the new spare_tile.
    pub fn slide_and_insert(&mut self, Slide { index, direction }: Slide<BOARD_SIZE>) {
        use CompassDirection::*;
        match direction {
            North => {
                let col_num = index;
                let row_num = BOARD_SIZE - 1;
                self.grid.rotate_up(col_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(col_num, row_num)])
            }
            South => {
                let col_num = index;
                self.grid.rotate_down(col_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(col_num, 0)])
            }
            East => {
                let row_num = index;
                self.grid.rotate_right(row_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(0, row_num)])
            }
            West => {
                let row_num = index;
                let col_num = BOARD_SIZE - 1;
                self.grid.rotate_left(row_num);
                std::mem::swap(&mut self.extra, &mut self.grid[(col_num, row_num)])
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
        if start.0 < BOARD_SIZE - 1 && self.connected_positions(start, (start.0 + 1, start.1), East)
        {
            neighbors.push((start.0 + 1, start.1));
        }
        // south neighbor
        if start.1 < BOARD_SIZE - 1
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
        if start.0 >= BOARD_SIZE || start.1 >= BOARD_SIZE {
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

impl<const BOARD_SIZE: usize> Index<Position> for Board<BOARD_SIZE> {
    type Output = Tile;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index]
    }
}

impl<const BOARD_SIZE: usize> Default for Board<BOARD_SIZE> {
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
    fn default() -> Self {
        use crate::gem::Gem::*;
        use CompassDirection::*;
        use ConnectorShape::*;
        use PathOrientation::*;
        let mut idx = -1;
        let grid = [[(); BOARD_SIZE]; BOARD_SIZE].map(|list| {
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
                    gems: (amethyst, garnet),
                }
            })
        });
        Self {
            grid: Grid::from(grid),
            extra: Tile {
                connector: Crossroads,
                gems: (amethyst, garnet),
            },
        }
    }
}

/// Describes a slide motion
#[derive(Debug, Clone, Copy)]
pub struct Slide<const BOARD_SIZE: usize> {
    /// The index of the row or column to be slid
    /// Counts from 0 from left to right and top to bottom
    pub(crate) index: usize,
    /// The direction the row or column is sliding to
    pub(crate) direction: CompassDirection,
}

impl<const BOARD_SIZE: usize> Slide<BOARD_SIZE> {
    /// Attempts to create a slide command
    ///
    /// # Errors
    ///
    /// Errors if the index for the row/col is not a valid index for a slideable row/col.  
    /// i.e. on a 7x7 board valid indices are 0, 1, 2, and 3
    /// ```should_panic
    /// # use common::board::Slide;
    /// # use common::tile::CompassDirection;
    /// Slide::<7>::new(4, CompassDirection::North).unwrap();
    /// ```
    pub fn new(index: usize, direction: CompassDirection) -> Result<Slide<BOARD_SIZE>, String> {
        if index > BOARD_SIZE / 2 {
            Err(format!("Index must be between 0 and {}", BOARD_SIZE / 2))
        } else {
            Ok(Slide {
                index: index * 2,
                direction,
            })
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
        assert!(Slide::<1>::new(0, North).is_ok());
        assert!(Slide::<1>::new(1, North).is_err());

        assert!(Slide::<7>::new(0, South).is_ok());
        assert!(Slide::<7>::new(2, East).is_ok());
        assert!(Slide::<7>::new(4, West).is_err());
    }

    #[test]
    pub fn test_slide_and_insert() {
        // Initial Board state
        // ─│└
        // ┌┐┘
        // ┴├┬
        // extra = ┼
        let mut b: Board<3> = Board::default();
        dbg!(&b.grid);
        assert_eq!(b.extra.connector, Crossroads);

        b.slide_and_insert(Slide::new(0, South).unwrap());
        // Board after slide + insert
        // ┼│└
        // ─┐┘
        // ┌├┬
        // extra = ┴
        assert_eq!(b.grid[(0, 0)].connector, Crossroads);
        dbg!(&b.grid);
        assert_eq!(b.extra.connector, Fork(North));

        b.slide_and_insert(Slide::new(0, East).unwrap());
        // Board after insert
        // ┴┼│
        // ─┐┘
        // ┌├┬
        // extra = └
        assert_eq!(b.grid[(0, 0)].connector, Fork(North));
        assert_eq!(b.extra.connector, Corner(North));

        b.slide_and_insert(Slide::new(1, West).unwrap());
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
        let b = Board::<3>::default();
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
        let b = Board::<3>::default();
        assert!(b.reachable((10, 10)).is_err());
        let from_0_0 = b.reachable((0, 0));
        assert!(from_0_0.is_ok());
        assert_eq!(from_0_0.unwrap().len(), 1);
        let from_2_2 = b.reachable((2, 2));
        assert!(from_2_2.is_ok());
        assert_eq!(from_2_2.unwrap().len(), 5);
    }
}
