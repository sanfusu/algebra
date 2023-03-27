use std::ops::{Index, IndexMut};

use crate::vector::ColVec;

use super::{col::Col, raw::RawColMut, Matrix};

pub struct ColMut<'a, T> {
    pub(crate) matrix: &'a mut Matrix<T>,
    pub(crate) col: usize,
}

impl<'a, T> ColMut<'a, T> {
    pub(crate) fn as_raw(&mut self) -> RawColMut<T> {
        RawColMut {
            matrix: self.matrix.as_raw(),
            col: self.col,
        }
    }
    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        self.as_raw().get_mut(index)
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            col: self.as_raw(),
            idx: 0,
        }
    }
}

impl<'a, T: Clone> ColMut<'a, T> {
    pub fn to_vec(self) -> ColVec<T> {
        let col: Col<T> = self.into();
        col.to_vec()
    }
}

impl<'a, T> Into<Col<'a, T>> for ColMut<'a, T> {
    fn into(self) -> Col<'a, T> {
        Col {
            matrix: self.matrix,
            col: self.col,
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    col: RawColMut<'a, T>,
    idx: usize,
}
impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.col.get_mut(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}

impl<'a, T> IndexMut<usize> for ColMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get(index).unwrap()
    }
}
impl<'a, T> Index<usize> for ColMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.matrix.row(index).unwrap().get(self.col).unwrap()
    }
}
