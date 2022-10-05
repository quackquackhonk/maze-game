/// This type describes a single tile on a board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorShape {
    /// Path Can Only Be Horizontal Or Vertical
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
    Crossroads,
}

#[derive(Debug)]
pub enum Gem {
    Amethyst,
    Garnet,
}

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

/// This enum describes the four orientations for [`ConnectorShape::Corner`] [`ConnectorShape::Fork`]
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

impl Board {
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

impl ConnectorShape {
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
    /// use Common::CompassDirection;
    /// assert_eq!(CompassDirection::North.rotate_clockwise(), CompassDirection::East);
    /// ```
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
    pub fn test_compass_direction_rotate() {
        use CompassDirection::*;
        assert_eq!(North.rotate_clockwise(), East);
        assert_eq!(South.rotate_clockwise(), West);
        assert_eq!(East.rotate_clockwise(), South);
        assert_eq!(West.rotate_clockwise(), North);
    }

    #[test]
    pub fn tiletype_rotate() {
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
}
