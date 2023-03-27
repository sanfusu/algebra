use std::{
    marker::PhantomData,
    ptr::{slice_from_raw_parts_mut, NonNull},
};

pub(crate) struct RawMatrix<'a, T> {
    pub(crate) data: NonNull<T>,
    pub(crate) col: usize,
    pub(crate) row: usize,
    pub(crate) phantom: PhantomData<&'a mut T>,
}
impl<'a, T> RawMatrix<'a, T> {
    pub(crate) fn as_slice(&self) -> &'a mut [T] {
        unsafe { &mut *slice_from_raw_parts_mut(self.data.as_ptr(), self.col * self.row) }
    }
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
    pub(crate) fn get_mut(&self, idx: usize) -> Option<&'a mut T> {
        let flat_idx = self.flat_idx(idx);
        Some(&mut self.matrix.as_slice()[flat_idx?])
    }
}
