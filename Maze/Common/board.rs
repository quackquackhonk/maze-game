use crate::grid::*;
use crate::tile::*;
use std::collections::HashSet;
use std::ops::Index;

type BoardError = String;

type BoardResult<T> = Result<T, BoardError>;

/// Describes one board for the game of Maze`.`com
#[derive(Debug)]
pub struct Board<const BOARD_SIZE: usize> {
    grid: Grid<Option<Tile>, BOARD_SIZE, BOARD_SIZE>,
    extra: Tile,
}

impl<const BOARD_SIZE: usize> Board<BOARD_SIZE> {
    /// Slides the given Slide struct command leaving a `None` in the place of the dislodged tile
    ///
    /// Returns the current extra tile to be inserted in [`Board::insert`]
    pub fn slide(&mut self, Slide { index, direction }: Slide<BOARD_SIZE>) -> BoardResult<Tile> {
        use CompassDirection::*;
        if self.grid.iter().flatten().any(std::option::Option::is_none) {
            return Err("Board cannot be slid twice before inserting a Tile!".to_string());
        };
        match direction {
            North => {
                let col_num = index * 2;
                let tmp = self.grid[(col_num, 0)].take();
                self.grid.rotate_up(col_num);
                Ok(std::mem::replace(&mut self.extra, tmp.unwrap()))
            }
            South => {
                let col_num = index * 2;
                let row_num = BOARD_SIZE - 1;
                let tmp = self.grid[(col_num, row_num)].take();
                self.grid.rotate_down(col_num);
                Ok(std::mem::replace(&mut self.extra, tmp.unwrap()))
            }
            East => {
                let row_num = index * 2;
                self.grid.rotate_right(row_num);
                Ok(std::mem::replace(
                    &mut self.extra,
                    self.grid[(0, row_num)].take().unwrap(),
                ))
            }
            West => {
                let row_num = index * 2;
                self.grid.rotate_left(row_num);
                Ok(std::mem::replace(
                    &mut self.extra,
                    self.grid[(BOARD_SIZE - 1, row_num)].take().unwrap(),
                ))
            }
        }
    }

    /// Inserts the given Tile into the open slot on the Board
    ///
    /// Does nothing if there is no open slot, i.e. the board has not been slided yet
    /// using [`Board::slide`]
    pub fn insert(&mut self, tile: Tile) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let idx = (x, y);
                if self.grid[idx].is_none() {
                    self.grid[idx] = Some(tile);
                    return;
                }
            }
        }
    }

    /// Can you go from `from` to `to` in the given `dir`?
    fn connected_positions(&self, from: Position, to: Position, dir: CompassDirection) -> bool {
        self.grid[from]
            .as_ref()
            .unwrap()
            .connected(self.grid[to].as_ref().unwrap(), dir)
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
    /// Returns an error if `start.0` > BOARD_SIZE or `start.1` > BOARD_SIZE or either `start.0` or
    /// `start.1` are negative.
    pub fn reachable(&self, start: Position) -> BoardResult<Vec<Position>> {
        if start.0 >= BOARD_SIZE || start.1 >= BOARD_SIZE {
            return Err("Out-of-bounds Position".to_string());
        }

        // push start onto worklist
        // reachable = []
        // visited = <>
        let mut worklist = Vec::new();
        worklist.push(start);
        let mut reachable = Vec::new();
        let mut visited = HashSet::new();
        while let Some(curr) = worklist.pop() {
            let neighbors = self.reachable_neighbors(curr);
            let not_visited_neighbors = neighbors.into_iter().filter(|x| !visited.contains(x));
            not_visited_neighbors.for_each(|n| {
                reachable.push(n);
                worklist.push(n)
            });
            //    push x onto visited
            visited.insert(curr);
        }

        Ok(reachable)
    }
}

impl<const BOARD_SIZE: usize> Index<Position> for Board<BOARD_SIZE> {
    type Output = Option<Tile>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index]
    }
}

impl<const BOARD_SIZE: usize> Default for Board<BOARD_SIZE> {
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
        let mut grid = [[(); BOARD_SIZE]; BOARD_SIZE].map(|list| list.map(|_| None));
        for (idx, cell) in grid.iter_mut().flatten().enumerate() {
            *cell = Some(Tile {
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
                gems: (Amethyst, Garnet),
            });
        }
        Self {
            grid: Grid::from(grid),
            extra: Tile {
                connector: Crossroads,
                gems: (Amethyst, Garnet),
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
    /// Fails if the index for the row/col is out of bounds
    pub fn new(index: usize, direction: CompassDirection) -> Result<Slide<BOARD_SIZE>, String> {
        if index > BOARD_SIZE / 2 {
            Err(format!("Index must be between 0 and {}", BOARD_SIZE / 2))
        } else {
            Ok(Slide { index, direction })
        }
    }
}

#[cfg(test)]
mod BoardTests {
    use crate::gem::Gem;

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
    pub fn test_slide() {
        let mut b: Board<7> = Board::default();
        assert!(b.grid.iter().flatten().all(std::option::Option::is_some));
        assert!(b.slide(Slide::new(0, South).unwrap()).is_ok());
        assert!(b.grid.iter().flatten().any(std::option::Option::is_none));
        assert_eq!(
            b.grid
                .iter()
                .flatten()
                .fold(0, |sum, opt| { sum + if opt.is_none() { 1 } else { 0 } }),
            1
        );
        assert!(b.grid[0][0].is_none());

        let mut b: Board<7> = Board::default();
        assert!(b.grid.iter().flatten().all(std::option::Option::is_some));
        assert!(b.slide(Slide::new(0, North).unwrap()).is_ok());
        assert!(b.grid.iter().flatten().any(std::option::Option::is_none));
        assert_eq!(
            b.grid
                .iter()
                .flatten()
                .fold(0, |sum, opt| { sum + if opt.is_none() { 1 } else { 0 } }),
            1
        );
        assert!(b.grid[b.grid.len() - 1][0].is_none());

        let mut b: Board<7> = Board::default();
        assert!(b.grid.iter().flatten().all(std::option::Option::is_some));
        assert!(b.slide(Slide::new(0, East).unwrap()).is_ok());
        assert!(b.grid.iter().flatten().any(std::option::Option::is_none));
        assert_eq!(
            b.grid
                .iter()
                .flatten()
                .fold(0, |sum, opt| { sum + if opt.is_none() { 1 } else { 0 } }),
            1
        );
        assert!(b.grid[0][0].is_none());

        let mut b: Board<7> = Board::default();
        assert!(b.grid.iter().flatten().all(std::option::Option::is_some));
        assert!(b.slide(Slide::new(3, West).unwrap()).is_ok());
        assert!(b.grid.iter().flatten().any(std::option::Option::is_none));
        assert_eq!(
            b.grid
                .iter()
                .flatten()
                .fold(0, |sum, opt| { sum + if opt.is_none() { 1 } else { 0 } }),
            1
        );
        assert!(b.grid[b.grid.len() - 1][b.grid.len() - 1].is_none());
    }
    #[test]
    pub fn test_insert() {
        // Initial Board state
        // ─│└
        // ┌┐┘
        // ┴├┬
        // extra = ┼
        let mut b: Board<3> = Board::default();
        dbg!(&b.grid);
        assert_eq!(b.extra.connector, Crossroads);
        assert!(b.grid[0][0].is_some());
        let to_insert = b.slide(Slide::new(0, South).unwrap()).unwrap();
        // Board after slide
        //  │└
        // ─┐┘
        // ┌├┬
        // extra = ┴, to_insert = ┼
        assert_eq!(to_insert.connector, Crossroads);
        dbg!(&b.grid);
        assert_eq!(b.extra.connector, Fork(North));
        assert!(b.grid[0][0].is_none());
        b.insert(to_insert);
        // Board after slide + insert
        // ┼│└
        // ─┐┘
        // ┌├┬
        // extra = ┴
        assert_eq!(
            b.grid[0][0],
            Some(Tile {
                connector: Crossroads,
                gems: (Gem::Amethyst, Gem::Garnet)
            })
        );

        let to_insert = b.slide(Slide::new(0, East).unwrap()).unwrap();
        // Board after slide
        //  ┼│
        // ─┐┘
        // ┌├┬
        // extra = └, to_insert = ┴
        assert_eq!(to_insert.connector, Fork(North));
        assert_eq!(b.extra.connector, Corner(North));
        assert!(b.grid[0][0].is_none());
        b.insert(to_insert);
        // Board after insert
        // ┴┼│
        // ─┐┘
        // ┌├┬
        // extra = └
        assert_eq!(
            b.grid[0][0],
            Some(Tile {
                connector: Fork(North),
                gems: (Gem::Amethyst, Gem::Garnet)
            })
        );

        // Board after slide + insert
        // ┴┼│
        // ─┐┘
        // ┌├┬
        // extra = └
        let to_insert = b.slide(Slide::new(1, West).unwrap()).unwrap();
        // Board after slide
        // ┴┼│
        // ─┐┘
        // ├┬
        // extra = ┌, to_insert = └
        assert_eq!(to_insert.connector, Corner(North));
        assert_eq!(b.extra.connector, Corner(East));
        assert!(b.grid[2][2].is_none());
        b.insert(to_insert);
        // Board after slide + insert
        // ┴┼│
        // ─┐┘
        // ├┬└
        // extra = ┌
        assert_eq!(
            b.grid[2][2],
            Some(Tile {
                connector: Corner(North),
                gems: (Gem::Amethyst, Gem::Garnet)
            })
        );

        assert!(b.slide(Slide::new(0, South).unwrap()).is_ok());
        assert!(b.slide(Slide::new(0, South).unwrap()).is_err());
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
        // Default Board\<3> is:
        // ─│└
        // ┌┐┘
        // ┴├┬
        // extra = ┼
        let b = Board::<3>::default();
        assert!(b.reachable((10, 10)).is_err());
        let from_0_0 = b.reachable((0, 0));
        assert!(from_0_0.is_ok());
        assert!(from_0_0.unwrap().is_empty());
        let from_2_2 = b.reachable((2, 2));
        assert!(from_2_2.is_ok());
        assert!(!from_2_2.unwrap().is_empty());
    }
}
