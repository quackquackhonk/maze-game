use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Type alias for Positions on the Board
pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Grid<T, const N: usize, const M: usize>([[T; N]; M]);

impl<T, const N: usize, const M: usize> Grid<T, N, M> {
    pub fn rotate_left(&mut self, index: usize) {
        self[index].rotate_left(1)
    }
    pub fn rotate_right(&mut self, index: usize) {
        self[index].rotate_right(1)
    }
    pub fn rotate_up(&mut self, col_num: usize) {
        for row_index in 1..self.len() {
            let (top_rows, bottom_rows) = self.split_at_mut(row_index);
            std::mem::swap(
                &mut top_rows[top_rows.len() - 1][col_num],
                &mut bottom_rows[0][col_num],
            );
        }
    }
    pub fn rotate_down(&mut self, col_num: usize) {
        for row_index in (0..(self.len() - 1)).rev() {
            let (top_rows, bottom_rows) = self.split_at_mut(row_index + 1);
            std::mem::swap(
                &mut top_rows[top_rows.len() - 1][col_num],
                &mut bottom_rows[0][col_num],
            );
        }
    }
}

/// [`std::ops::From`] implementation for a `Grid`.
///
/// # Examples
/// ```
/// use Common::grid::Grid;
/// let g = Grid::from([[(); 4]; 5]);
/// assert_eq!(g.len(), 5);
/// assert_eq!(g[0].len(), 4);
/// ```
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
