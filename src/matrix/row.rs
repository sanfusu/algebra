use std::ops::Index;

use super::Matrix;

pub struct Row<'a, T> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) row: usize,
}
impl<'a, T> Row<'a, T> {
    pub fn as_slice(&self) -> &'a [T] {
        &self.matrix[self.row]
    }
    pub fn get(&self, index: usize) -> Option<&'a T> {
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
    pub fn to_vec(&self) -> Vec<T> {
        self.as_slice().to_owned()
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[self.row][index]
    }
}
pub struct RowEleIter<'a, T> {
    row: Row<'a, T>,
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

pub struct Rows<'a, T> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) idx: usize,
}
impl<'a, T> Iterator for Rows<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.row(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}
