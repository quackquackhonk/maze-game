use unordered_pair::UnorderedPair;

use crate::gem::Gem;
/// Represents a single tile on a board
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub connector: ConnectorShape,
    pub gems: UnorderedPair<Gem>,
}

impl Tile {
    /// Rotates the tile according to the symmetries of the underlying `ConnectorShape`
    pub fn rotate(&mut self) {
        self.connector = self.connector.rotate();
    }

    /// Checks if `self` can connect to `other` in the given [`CompassDirection`].
    pub fn connected(&self, other: &Self, direction: CompassDirection) -> bool {
        self.connector.connected(other.connector, direction)
    }

    pub fn from_num(num: usize) -> Tile {
        Self {
            connector: ConnectorShape::from_num(num),
            gems: Gem::pair_from_num(num),
        }
    }
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

impl CompassDirection {
    /// Returns a rotated direction 90 degrees clockwise.
    /// ```
    /// # use common::tile::CompassDirection;
    /// assert_eq!(CompassDirection::North.rotate_clockwise(), CompassDirection::East);
    /// ```
    #[must_use]
    pub fn rotate_clockwise(self) -> Self {
        use CompassDirection::*;
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    /// Returns a rotated direction 90 degrees counter clockwise.
    /// ```
    /// # use common::tile::CompassDirection;
    /// assert_eq!(CompassDirection::North.rotate_counter_clockwise(), CompassDirection::West);
    /// ```
    #[must_use]
    pub fn rotate_counter_clockwise(self) -> Self {
        use CompassDirection::*;
        match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    /// Returns the opposite direction of the given direction
    /// ```
    ///# use common::tile::CompassDirection;
    /// assert_eq!(CompassDirection::North.opposite(), CompassDirection::South);
    /// assert_eq!(CompassDirection::South.opposite(), CompassDirection::North);
    /// assert_eq!(CompassDirection::East.opposite(), CompassDirection::West);
    /// assert_eq!(CompassDirection::West.opposite(), CompassDirection::East);
    /// ```
    #[must_use]
    pub fn opposite(self) -> Self {
        use CompassDirection::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

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

impl ConnectorShape {
    /// Rotates the `ConnectorShape` according to the symmetries of the `ConnectorShape`
    #[must_use]
    pub fn rotate(self) -> Self {
        use ConnectorShape::*;
        use PathOrientation::*;
        match self {
            Path(Horizontal) => Path(Vertical),
            Path(Vertical) => Path(Horizontal),
            Corner(dir) => Corner(dir.rotate_counter_clockwise()),
            Fork(dir) => Fork(dir.rotate_counter_clockwise()),
            Crossroads => Crossroads,
        }
    }

    /// Can we go in this `direction` from this [`ConnectorShape`], `self`?
    pub fn connected_to(self, direction: CompassDirection) -> bool {
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
    pub fn connected(self, other: Self, direction: CompassDirection) -> bool {
        self.connected_to(direction) && other.connected_to(direction.opposite())
    }

    pub fn from_num(num: usize) -> Self {
        use CompassDirection::*;
        use ConnectorShape::*;
        use PathOrientation::*;
        match num % 11 {
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
        }
    }
}

#[cfg(test)]
mod tile_tests {
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

        assert_eq!(Corner(North).rotate(), Corner(West));
        assert_eq!(Corner(North).rotate().rotate(), Corner(South));
        assert_eq!(Corner(North).rotate().rotate().rotate(), Corner(East));
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
            gems: (Amethyst, Garnet).into(),
        };

        tile.rotate();
        assert_eq!(tile.connector, Fork(West));
        tile.rotate();
        assert_eq!(tile.connector, Fork(South));
        tile.rotate();
        assert_eq!(tile.connector, Fork(East));
        tile.rotate();
        assert_eq!(tile.connector, Fork(North));
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
        let gems = (Gem::Amethyst, Gem::Garnet);
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
            gems: gems.into()
        }
        .connected(
            &Tile {
                connector: Crossroads,
                gems: gems.into()
            },
            North
        ));
        assert!(Tile {
            connector: Path(Horizontal),
            gems: gems.into()
        }
        .connected(
            &Tile {
                connector: Path(Horizontal),
                gems: gems.into()
            },
            West
        ));
    }
}
