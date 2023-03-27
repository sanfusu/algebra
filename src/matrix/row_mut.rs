use std::{
    ops::{Index, IndexMut},
    slice::IterMut,
};

use super::Matrix;

pub struct RowMut<'a, T> {
    pub(crate) matrix: &'a mut Matrix<T>,
    pub(super) row: usize,
}

impl<'a, T> RowMut<'a, T> {
    pub fn as_slice(&mut self) -> &mut [T] {
        &mut self.matrix[self.row]
    }
    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.matrix.col {
            None
        } else {
            Some(&mut self.as_slice()[index])
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.as_slice().iter_mut()
    }
}

impl<'a, T> IntoIterator for &'a mut RowMut<'a, T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter_mut()
    }
}

impl<'a, T> IndexMut<usize> for RowMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> Index<usize> for RowMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.matrix.row(self.row).unwrap().get(index).unwrap()
    }
}
