use std::{
    collections::HashSet,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// This type describes the connection type of a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorShape {
    /// Path Can Only Be Horizontal Or Vertical  
    /// ─ - Horizontal  
    /// │ - Vertical
    Path(PathOrientation),
    /// Direction is dictated by what CompassDirection
    /// it turns right to.  
    /// ┐ - South  
    /// └ - North  
    /// ┌ - East  
    /// ┘ - West  
    Corner(CompassDirection),
    /// Direction is dictated by the middle path  
    /// ┬ - South  
    /// ┴ - North  
    /// ├ - East  
    /// ┤ - West  
    Fork(CompassDirection),
    /// Crossroads is the same in every direction  
    /// ┼
    Crossroads,
}

type BoardError = String;

type BoardResult<T> = Result<T, BoardError>;

/// Describes the gems a tile can have
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gem {
    Amethyst,
    Garnet,
}

/// Represents a single tile on a board
#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    connector: ConnectorShape,
    gems: (Gem, Gem),
}

/// This enum describes the two orientations for [`ConnectorShape::Path`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOrientation {
    Horizontal,
    Vertical,
}

/// This enum describes the four orientations for [`ConnectorShape::Corner`] and [`ConnectorShape::Fork`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompassDirection {
    North,
    South,
    East,
    West,
}

/// Type alias for Positions on the Board
type Position = (usize, usize);

#[derive(Debug)]
struct Grid<T, const N: usize, const M: usize>([[T; N]; M]);

impl<T, const N: usize, const M: usize> From<[[T; N]; M]> for Grid<T, N, M> {
    fn from(from: [[T; N]; M]) -> Self {
        Grid(from)
    }
}

impl<T, const N: usize, const M: usize> Deref for Grid<T, N, M> {
    type Target = [[T; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize, const M: usize> DerefMut for Grid<T, N, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize, const M: usize> Index<usize> for Grid<T, N, M> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<T, const N: usize, const M: usize> IndexMut<usize> for Grid<T, N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const N: usize, const M: usize> Index<(usize, usize)> for Grid<T, N, M> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}
impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for Grid<T, N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

impl<T, const N: usize, const M: usize> Grid<T, N, M> {
    fn rotate_left(&mut self, index: usize) {
        self[index].rotate_left(1)
    }
    fn rotate_right(&mut self, index: usize) {
        self[index].rotate_right(1)
    }
    fn rotate_up(&mut self, col_num: usize) {
        for row_index in 1..self.len() {
            let (top_rows, bottom_rows) = self.split_at_mut(row_index);
            std::mem::swap(
                &mut top_rows[top_rows.len() - 1][col_num],
                &mut bottom_rows[0][col_num],
            );
        }
    }
    fn rotate_down(&mut self, col_num: usize) {
        for row_index in (0..(self.len() - 1)).rev() {
            let (top_rows, bottom_rows) = self.split_at_mut(row_index + 1);
            std::mem::swap(
                &mut top_rows[top_rows.len() - 1][col_num],
                &mut bottom_rows[0][col_num],
            );
        }
    }
}

/// Describes one board for the game of Maze`.`com
#[derive(Debug)]
pub struct Board<const BOARD_SIZE: usize> {
    grid: Grid<Option<Tile>, BOARD_SIZE, BOARD_SIZE>,
    extra: Tile,
}

/// Describes a slide motion
pub struct Slide<const BOARD_SIZE: usize> {
    /// The index of the row or column to be slid
    /// Counts from 0 from left to right and top to bottom
    index: usize,
    /// The direction the row or column is sliding to
    direction: CompassDirection,
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
                let row_num = self.grid.len() - 1;
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

impl<const BOARD_SIZE: usize> Default for Board<BOARD_SIZE> {
    /// Default Board<3> is:  
    /// ─│└  
    /// ┌┐┘  
    /// ┴├┬  
    /// extra = ┼
    fn default() -> Self {
        use CompassDirection::*;
        use ConnectorShape::*;
        use Gem::*;
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
            grid: Grid(grid),
            extra: Tile {
                connector: Crossroads,
                gems: (Amethyst, Garnet),
            },
        }
    }
}

impl Tile {
    /// Rotates the tile according to the symmetries of the underlying ConnectorShape
    pub fn rotate(&mut self) {
        self.connector = self.connector.rotate();
    }

    /// Checks if `self` can connect to `other` in the given [`CompassDirection`].
    fn connected(&self, other: &Self, direction: CompassDirection) -> bool {
        self.connector.connected(other.connector, direction)
    }
}

impl ConnectorShape {
    /// Rotates the ConnectorShape according to the symmetries of the ConnectorShape
    #[must_use]
    pub fn rotate(self: Self) -> Self {
        use ConnectorShape::*;
        use PathOrientation::*;
        match self {
            Path(Horizontal) => Path(Vertical),
            Path(Vertical) => Path(Horizontal),
            Corner(dir) => Corner(dir.rotate_clockwise()),
            Fork(dir) => Fork(dir.rotate_clockwise()),
            Crossroads => Crossroads,
        }
    }

    /// Can we go in this `direction` from this [`ConnectorShape`], `self`?
    fn connected_to(&self, direction: CompassDirection) -> bool {
        use CompassDirection::*;
        use ConnectorShape::*;
        use PathOrientation::*;
        matches!(
            (self, direction),
            (Path(Vertical), North | South)
                | (Path(Horizontal), East | West)
                | (Corner(North), North | East)
                | (Corner(South), South | West)
                | (Corner(East), East | South)
                | (Corner(West), West | North)
                | (Fork(North), North | East | West)
                | (Fork(South), South | East | West)
                | (Fork(East), East | North | South)
                | (Fork(West), West | North | South)
                | (Crossroads, _)
        )
    }

    /// Checks if `self` can connect to `other` in the given [`CompassDirection`].
    pub fn connected(&self, other: Self, direction: CompassDirection) -> bool {
        self.connected_to(direction)
            && other.connected_to(direction.rotate_clockwise().rotate_clockwise())
    }
}

impl CompassDirection {
    /// Returns a rotated direction 90 degrees clockwise.
    /// ```
    /// # use Common::CompassDirection;
    /// assert_eq!(CompassDirection::North.rotate_clockwise(), CompassDirection::East);
    /// ```
    #[must_use]
    pub fn rotate_clockwise(self: Self) -> Self {
        use CompassDirection::*;
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

#[cfg(test)]
mod Tests {
    use super::*;
    use CompassDirection::*;
    use ConnectorShape::*;
    use PathOrientation::*;

    #[test]
    pub fn compass_direction_rotate() {
        assert_eq!(North.rotate_clockwise(), East);
        assert_eq!(South.rotate_clockwise(), West);
        assert_eq!(East.rotate_clockwise(), South);
        assert_eq!(West.rotate_clockwise(), North);
    }

    #[test]
    pub fn connector_rotate() {
        assert_eq!(Crossroads.rotate(), Crossroads);
        assert_eq!(Crossroads.rotate().rotate(), Crossroads);

        assert_eq!(Path(Vertical).rotate(), Path(Horizontal));
        assert_eq!(Path(Vertical).rotate().rotate(), Path(Vertical));
        assert_eq!(Path(Horizontal).rotate(), Path(Vertical));
        assert_eq!(Path(Horizontal).rotate().rotate(), Path(Horizontal));

        assert_eq!(Corner(North).rotate(), Corner(East));
        assert_eq!(Corner(North).rotate().rotate(), Corner(South));
        assert_eq!(Corner(North).rotate().rotate().rotate(), Corner(West));
        assert_eq!(
            Corner(North).rotate().rotate().rotate().rotate(),
            Corner(North)
        );
    }

    #[test]
    pub fn tile_rotate() {
        use Gem::*;
        let mut tile = Tile {
            connector: Fork(North),
            gems: (Amethyst, Garnet),
        };

        tile.rotate();
        assert_eq!(tile.connector, Fork(East));
        tile.rotate();
        assert_eq!(tile.connector, Fork(South));
        tile.rotate();
        assert_eq!(tile.connector, Fork(West));
        tile.rotate();
        assert_eq!(tile.connector, Fork(North));
    }

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
    pub fn test_connected_to() {
        assert!(Crossroads.connected_to(North));
        assert!(Crossroads.connected_to(South));
        assert!(Crossroads.connected_to(East));
        assert!(Crossroads.connected_to(West));

        assert!(Path(Vertical).connected_to(North));
        assert!(Path(Vertical).connected_to(South));
        assert!(!Path(Vertical).connected_to(East));
        assert!(Path(Horizontal).connected_to(East));
        assert!(Path(Horizontal).connected_to(West));
        assert!(!Path(Horizontal).connected_to(North));

        assert!(Fork(North).connected_to(North));
        assert!(Fork(North).connected_to(East));
        assert!(Fork(North).connected_to(West));
        assert!(!Fork(North).connected_to(South));
        assert!(!Fork(South).connected_to(North));
        assert!(Fork(South).connected_to(East));
        assert!(Fork(South).connected_to(West));
        assert!(Fork(South).connected_to(South));
        assert!(Fork(East).connected_to(East));
        assert!(Fork(East).connected_to(North));
        assert!(Fork(East).connected_to(South));
        assert!(!Fork(East).connected_to(West));
        assert!(!Fork(West).connected_to(East));
        assert!(Fork(West).connected_to(North));
        assert!(Fork(West).connected_to(South));
        assert!(Fork(West).connected_to(West));

        assert!(Corner(North).connected_to(North));
        assert!(Corner(North).connected_to(East));
        assert!(!Corner(North).connected_to(South));
        assert!(!Corner(North).connected_to(West));
        assert!(!Corner(East).connected_to(North));
        assert!(Corner(East).connected_to(East));
        assert!(Corner(East).connected_to(South));
        assert!(!Corner(East).connected_to(West));
        assert!(!Corner(South).connected_to(North));
        assert!(!Corner(South).connected_to(East));
        assert!(Corner(South).connected_to(South));
        assert!(Corner(South).connected_to(West));
        assert!(Corner(West).connected_to(North));
        assert!(!Corner(West).connected_to(East));
        assert!(!Corner(West).connected_to(South));
        assert!(Corner(West).connected_to(West));
    }

    #[test]
    pub fn test_connected() {
        use Gem::*;
        let gems = (Amethyst, Garnet);
        assert!(Crossroads.connected(Crossroads, North));
        assert!(Crossroads.connected(Crossroads, South));
        assert!(Crossroads.connected(Crossroads, East));
        assert!(Crossroads.connected(Crossroads, West));

        assert!(!Path(Vertical).connected(Path(Horizontal), North));
        assert!(Path(Vertical).connected(Path(Vertical), North));
        assert!(!Path(Vertical).connected(Path(Vertical), East));
        assert!(Path(Horizontal).connected(Path(Horizontal), East));
        assert!(!Path(Horizontal).connected(Path(Horizontal), North));

        assert!(Fork(North).connected(Fork(South), North));
        assert!(!Fork(North).connected(Fork(South), South));
        assert!(Fork(North).connected(Path(Horizontal), East));
        assert!(Fork(North).connected(Corner(East), West));

        assert!(!Corner(East).connected(Crossroads, North));
        assert!(Corner(East).connected(Corner(North), South));
        assert!(Corner(East).connected(Path(Horizontal), East));
        assert!(!Corner(East).connected(Fork(East), West));

        // some tests for the Tile wrapper for connected
        assert!(Tile {
            connector: Crossroads,
            gems
        }
        .connected(
            &Tile {
                connector: Crossroads,
                gems
            },
            North
        ));
        assert!(Tile {
            connector: Path(Horizontal),
            gems
        }
        .connected(
            &Tile {
                connector: Path(Horizontal),
                gems
            },
            West
        ));
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
