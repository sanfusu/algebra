use std::ops::Index;

use crate::vector::RowVec;

use super::Matrix;

pub struct Row<'a, T> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) row: usize,
}
impl<'a:'b, 'b, T> Row<'a, T> {
    pub fn as_slice(&'b self) -> &'a [T] {
        &self.matrix[self.row]
    }
    pub fn get(&'b self, index: usize) -> Option<&'a T> {
        if index >= self.matrix.col {
            None
        } else {
            Some(&self.as_slice()[index])
        }
    }
}
impl<'a, T> Row<'a, T>
where
    T: Clone,
{
    pub fn to_vec(&self) -> RowVec<T> {
        RowVec {
            data: self.as_slice().to_owned(),
        }
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[self.row][index]
    }
}

impl<'a, T> IntoIterator for &'a Row<'a, T> {
    type Item = &'a T;

    type IntoIter = RowEleIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowEleIter { row: self, idx: 0 }
    }
}
pub struct RowEleIter<'a, T> {
    row: &'a Row<'a, T>,
    idx: usize,
}
impl<'a, T> Iterator for RowEleIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.row.get(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}
impl<'a, T> IntoIterator for Row<'a, T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = IntoRowEleIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoRowEleIter { row: self, idx: 0 }
    }
}
pub struct IntoRowEleIter<'a, T> {
    row: Row<'a, T>,
    idx: usize,
}
impl<'a, T> Iterator for IntoRowEleIter<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.row.get(self.idx)?.clone();
        self.idx += 1;
        Some(result)
    }
}

pub struct RowMatrix<'a, T> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) idx: usize,
}
impl<'a, T> Iterator for RowMatrix<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.row(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}
