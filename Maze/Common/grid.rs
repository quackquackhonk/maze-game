use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Type alias for Positions on the Board
/// A position is a tuple of (x, y)
/// where:
///     x is a column index
///     y is a row index
pub type Position = (usize, usize);

/// Represents a wrapper type for a 2D array with added functionality
#[derive(Debug)]
pub struct Grid<T, const COLS: usize, const ROWS: usize>([[T; COLS]; ROWS]);

impl<T, const COLS: usize, const ROWS: usize> Clone for Grid<T, COLS, ROWS>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, const COLS: usize, const ROWS: usize> Grid<T, COLS, ROWS> {
    /// Rotates the row at `index` left one time
    pub fn rotate_left(&mut self, index: usize) {
        self[index].rotate_left(1);
    }

    /// Rotates the row at `index` right one time
    pub fn rotate_right(&mut self, index: usize) {
        self[index].rotate_right(1);
    }

    /// Rotates the column at `index` up one time
    pub fn rotate_up(&mut self, col_num: usize) {
        for row_index in 1..self.len() {
            let (top_rows, bottom_rows) = self.split_at_mut(row_index);
            std::mem::swap(
                &mut top_rows[row_index - 1][col_num],
                &mut bottom_rows[0][col_num],
            );
        }
    }

    /// Rotates the column at `index` down one time
    pub fn rotate_down(&mut self, col_num: usize) {
        for row_index in (0..(self.len() - 1)).rev() {
            let (top_rows, bottom_rows) = self.split_at_mut(row_index + 1);
            std::mem::swap(
                &mut top_rows[row_index][col_num],
                &mut bottom_rows[0][col_num],
            );
        }
    }
}

/// [`From`] implementation for a `Grid`.
///
/// # Examples
/// ```
/// use common::grid::Grid;
/// let g = Grid::from([[(); 4]; 5]);
/// assert_eq!(g.len(), 5);
/// assert_eq!(g[0].len(), 4);
/// ```
impl<T, const COLS: usize, const ROWS: usize> From<[[T; COLS]; ROWS]> for Grid<T, COLS, ROWS> {
    fn from(from: [[T; COLS]; ROWS]) -> Self {
        Grid(from)
    }
}

impl<T, const COLS: usize, const ROWS: usize> Deref for Grid<T, COLS, ROWS> {
    type Target = [[T; COLS]; ROWS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const COLS: usize, const ROWS: usize> DerefMut for Grid<T, COLS, ROWS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Allows us to index a `Grid` as a slice.
///
/// # Examples
///
/// ```
/// use common::grid::Grid;
/// let g = Grid::from([[(); 3]; 3]);
/// assert_eq!(g[0], [(), (), ()]);
/// ```
///
/// # Panics
///
/// Panics when `index` > N.
///
/// ```should_panic
/// use common::grid::Grid;
/// let g = Grid::from([[(); 3]; 3]);
/// assert_eq!(g[(4, 0)], ());
/// ```
impl<T, const COLS: usize, const ROWS: usize> Index<usize> for Grid<T, COLS, ROWS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Allows us to mutably index a `Grid` as a slice.
///
/// # Panics
///
/// Same panic conditions as `Index`.
impl<T, const COLS: usize, const ROWS: usize> IndexMut<usize> for Grid<T, COLS, ROWS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// Allows us to index a `Grid` using a [`crate::grid::Position`].
///
/// # Examples
///
/// ```
/// use common::grid::Grid;
/// let g = Grid::from([[(); 3]; 3]);
/// assert_eq!(g[(0, 0)], ());
/// assert_eq!(g[(1, 2)], ());
/// ```
///
/// # Panics
///
/// Panics when the given index is out of range for the internal slice. Specifically, if any of the
/// following conditions are true:
/// - `index.0` > `M`
/// - `index.1` > `N`
///
/// ```should_panic
/// use common::grid::Grid;
/// let g = Grid::from([[(); 3]; 3]);
/// assert_eq!(g[(4, 0)], ());
/// ```
impl<T, const COLS: usize, const ROWS: usize> Index<Position> for Grid<T, COLS, ROWS> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

/// Allows us to mutably index a `Grid` using a [`crate::grid::Position`].
///
/// # Panics
///
/// Same panic conditions as `Index`.
impl<T, const COLS: usize, const ROWS: usize> IndexMut<Position> for Grid<T, COLS, ROWS> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

#[cfg(test)]
mod GridTests {
    use super::*;

    #[test]
    pub fn test_grid_rotate_left() {
        let mut g = Grid::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        assert_eq!(g[0], [1, 2, 3, 4]);
        g.rotate_left(0);
        assert_eq!(g[0], [2, 3, 4, 1]);
        g.rotate_left(0);
        assert_eq!(g[0], [3, 4, 1, 2]);
        g.rotate_left(0);
        assert_eq!(g[0], [4, 1, 2, 3]);
        g.rotate_left(0);
        assert_eq!(g[0], [1, 2, 3, 4]);

        assert_eq!(g[3], [13, 14, 15, 16]);
        g.rotate_left(3);
        assert_eq!(g[3], [14, 15, 16, 13]);
    }

    #[test]
    pub fn test_grid_rotate_right() {
        let mut g = Grid::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        assert_eq!(g[1], [5, 6, 7, 8]);
        g.rotate_right(1);
        assert_eq!(g[1], [8, 5, 6, 7]);
        g.rotate_right(1);
        assert_eq!(g[1], [7, 8, 5, 6]);
        g.rotate_right(1);
        assert_eq!(g[1], [6, 7, 8, 5]);
        g.rotate_right(1);
        assert_eq!(g[1], [5, 6, 7, 8]);
    }

    fn compare_col<T: Eq + std::fmt::Debug, const N: usize, const M: usize>(
        g: &Grid<T, N, M>,
        col_idx: usize,
        col: [T; M],
    ) {
        for (row_idx, row_val) in col.iter().enumerate() {
            assert_eq!(g[(col_idx, row_idx)], *row_val);
        }
    }

    #[test]
    pub fn test_grid_rotate_up() {
        let mut g = Grid::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        compare_col(&g, 0, [1, 5, 9, 13]);
        g.rotate_up(0);
        compare_col(&g, 0, [5, 9, 13, 1]);
        g.rotate_up(0);
        compare_col(&g, 0, [9, 13, 1, 5]);
        g.rotate_up(0);
        compare_col(&g, 0, [13, 1, 5, 9]);
        g.rotate_up(0);
        compare_col(&g, 0, [1, 5, 9, 13]);
    }

    #[test]
    pub fn test_grid_rotate_down() {
        let mut g = Grid::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        compare_col(&g, 1, [2, 6, 10, 14]);
        g.rotate_down(1);
        compare_col(&g, 1, [14, 2, 6, 10]);
        g.rotate_down(1);
        compare_col(&g, 1, [10, 14, 2, 6]);
        g.rotate_down(1);
        compare_col(&g, 1, [6, 10, 14, 2]);
        g.rotate_down(1);
        compare_col(&g, 1, [2, 6, 10, 14]);
    }
}
