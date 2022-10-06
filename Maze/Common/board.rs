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

/// Describes the gems a tile can have
#[derive(Debug)]
pub enum Gem {
    Amethyst,
    Garnet,
}

/// Represents a single tile on a board
#[derive(Debug)]
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

const BOARD_SIZE: usize = 7;

/// Describes one board for the game of Maze`.`com
#[derive(Debug)]
pub struct Board {
    grid: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
    extra: Tile,
}

/// Describes a slide motion
pub struct Slide {
    /// The index of the row or column to be slid
    /// Counts from 0 from left to right and top to bottom
    index: usize,
    /// The direction the row or column is sliding to
    direction: CompassDirection,
}

impl Slide {
    /// Attempts to create a slide command
    ///
    /// Fails if the index for the row/col is out of bounds
    pub fn new(index: usize, direction: CompassDirection) -> Result<Slide, String> {
        if index > BOARD_SIZE / 2 {
            Err(format!("Index must be between 0 and {}", BOARD_SIZE / 2))
        } else {
            Ok(Slide { index, direction })
        }
    }
}

impl Board {
    /// Slides the given Slide struct command leaving a `None` in the place of the dislodged tile
    ///
    /// Returns the current extra tile to be inserted in [`Board::insert`]
    pub fn slide(&mut self, Slide { index, direction }: Slide) -> Tile {
        use CompassDirection::*;
        match direction {
            North => {
                let col_num = index * 2;
                let tmp = self.grid[0][col_num].take();
                for row_index in (0..(self.grid.len() - 1)).rev() {
                    std::mem::swap(
                        &mut self.grid[row_index][col_num].as_ref(),
                        &mut self.grid[row_index + 1][col_num].as_ref(),
                    );
                }
                tmp.unwrap()
            }
            South => {
                let col_num = index * 2;
                let tmp = self.grid[self.grid.len()][col_num].take();
                for row_index in 1..self.grid.len() {
                    std::mem::swap(
                        &mut self.grid[row_index][col_num].as_ref(),
                        &mut self.grid[row_index - 1][col_num].as_ref(),
                    );
                }
                tmp.unwrap()
            }
            East => {
                let row = &mut self.grid[index * 2];
                row.rotate_right(1);
                std::mem::replace(&mut self.extra, row[0].take().unwrap())
            }
            West => {
                let row = &mut self.grid[index * 2];
                row.rotate_left(1);
                std::mem::replace(&mut self.extra, row[row.len() - 1].take().unwrap())
            }
        }
    }

    /// Inserts the given Tile into the open slot on the Board
    ///
    /// Does nothing if there is no open slot, i.e. the board has not been slided yet
    /// using [`Board::slide`]
    pub fn insert(&mut self, tile: Tile) {
        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                if let None = self.grid[r][c] {
                    self.grid[r][c] = Some(tile);
                    return;
                }
            }
        }
    }

    pub fn reachable(&self) {
        todo!()
    }
}

impl Default for Board {
    fn default() -> Self {
        use ConnectorShape::*;
        use Gem::*;
        let grid = [[(); BOARD_SIZE]; BOARD_SIZE].map(|list| {
            list.map(|_| {
                Some(Tile {
                    connector: Crossroads,
                    gems: (Amethyst, Garnet),
                })
            })
        });
        Self {
            grid,
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

    #[test]
    pub fn compass_direction_rotate() {
        use CompassDirection::*;
        assert_eq!(North.rotate_clockwise(), East);
        assert_eq!(South.rotate_clockwise(), West);
        assert_eq!(East.rotate_clockwise(), South);
        assert_eq!(West.rotate_clockwise(), North);
    }

    #[test]
    pub fn connector_rotate() {
        use CompassDirection::*;
        use ConnectorShape::*;
        use PathOrientation::*;
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
        use CompassDirection::*;
        use ConnectorShape::*;
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
}
