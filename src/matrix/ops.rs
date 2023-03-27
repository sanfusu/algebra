use std::ops::{Add, Mul};

use super::{
    col::{Col, IntoColEleIter},
    row::{Row, IntoRowEleIter},
    Matrix,
};

impl<T> Add for Matrix<T>
where
    T: Clone + std::ops::Add<Output = T>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs).unwrap()
    }
}
#[derive(Debug)]
pub struct MatrixDifferentArrangement;
impl<T> Matrix<T>
where
    T: Clone + std::ops::Add<Output = T>,
{
    pub fn add(self, rhs: Self) -> Result<Self, MatrixDifferentArrangement> {
        if (self.col != rhs.col) || (self.row != rhs.row) {
            Err(MatrixDifferentArrangement)
        } else {
            let mut out = self.clone();
            for ((l, r), result) in self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .zip(out.data.iter_mut())
            {
                *result = l + r;
            }
            Ok(out)
        }
    }
}
impl<T: Copy> Matrix<T> {
    pub fn transpose(&mut self) {
        if self.col != self.row {
            return;
        }
        if self.col == 1 || self.row == 1 {
            (self.col, self.row) = (self.row, self.col);
        }
        for r in 0..self.row {
            for j in (r + 1)..self.col {
                (self[r][j], self[j][r]) = (self[j][r], self[r][j])
            }
        }
    }
}

impl<'a, T> Mul<Col<'a, T>> for Row<'a, T>
where
    T: Default,
    T: Clone,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Col<'a, T>) -> Self::Output {
        let row_ele_iter: IntoRowEleIter<T> = self.into_iter();
        let other: IntoColEleIter<T> = rhs.into_iter();
        row_ele_iter.zip(other).fold(T::default(), |sum, (x, y)| {
            let diff = x * y;
            sum + diff
        })
    }
}
