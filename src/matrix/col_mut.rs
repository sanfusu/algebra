use std::{
    mem::transmute,
    ops::{Index, IndexMut},
    ptr::slice_from_raw_parts_mut,
};

use crate::vector::ColVec;

use super::{col::Col, Matrix, RawMatrix};

pub struct RawColMut<'a, T> {
    matrix: RawMatrix<'a, T>,
    col: usize,
}

impl<'a, T> RawColMut<'a, T> {
    pub fn flat_idx(&self, idx: usize) -> Option<usize> {
        if idx > self.matrix.row {
            None
        } else {
            Some(idx * self.matrix.col + self.col)
        }
    }
}

pub struct ColMut<'a, T> {
    pub(crate) matrix: &'a mut Matrix<T>,
    pub(crate) col: usize,
}

impl<'a, T> ColMut<'a, T> {
    pub fn as_raw(&mut self) -> RawColMut<T> {
        RawColMut {
            matrix: self.matrix.as_raw(),
            col: self.col,
        }
    }
    pub fn get(&mut self, index: usize) -> Option<&'a mut T> {
        // SAFETY: 'self >= &'a，且 element 是对 matrix.data 的引用，而非 row 的引用。
        let mut row = self.matrix.row_mut(index)?;
        let element = row.get(self.col);
        unsafe { transmute(element) }
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
        let flat_idx = self.col.flat_idx(self.idx)?;
        self.idx += 1;
        let flat_slice = unsafe {
            &mut *slice_from_raw_parts_mut(
                self.col.matrix.data.as_ptr(),
                self.col.matrix.col * self.col.matrix.row,
            )
        };
        Some(&mut flat_slice[flat_idx])
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
