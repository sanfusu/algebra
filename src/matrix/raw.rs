use std::{marker::PhantomData, ptr::NonNull};

pub(crate) struct RawMatrix<'a, T> {
    pub(crate) data: NonNull<T>,
    pub(crate) col: usize,
    pub(crate) row: usize,
    pub(crate) phantom: PhantomData<&'a mut T>,
}

pub(crate) struct RawColMut<'a, T> {
    pub(crate) matrix: RawMatrix<'a, T>,
    pub(crate) col: usize,
}
impl<'a, T> RawColMut<'a, T> {
    pub(crate) fn flat_idx(&self, idx: usize) -> Option<usize> {
        if idx < self.matrix.row {
            Some(idx * self.matrix.col + self.col)
        } else {
            None
        }
    }
}
