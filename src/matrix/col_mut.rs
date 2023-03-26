use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use super::{col::Col, Matrix};

pub struct ColMut<'a, T> {
    pub(crate) matrix: &'a mut Matrix<T>,
    pub(crate) col: usize,
}

impl<'a, T> ColMut<'a, T> {
    pub fn get(&mut self, index: usize) -> Option<&mut T> {
        let pos = index * self.matrix.col + self.col;
        self.matrix.data.get_mut(pos)
    }
}

impl<'a, T: Clone> ColMut<'a, T> {
    pub fn to_vec(self) -> Vec<T> {
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
impl<'a: 'b, 'b, T> IntoIterator for &'b mut ColMut<'a, T> {
    type Item = &'a mut T;

    type IntoIter = ColEleIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColEleIterMut {
            col: self.matrix.col,
            idx: 0,
            data: unsafe { NonNull::new_unchecked(self.matrix.data.as_mut_ptr()) },
            row: self.matrix.row,
            phantom: PhantomData,
        }
    }
}

pub struct ColEleIterMut<'a, T> {
    data: NonNull<T>,
    row: usize,
    col: usize,
    idx: usize,
    phantom: PhantomData<&'a mut T>,
}
impl<'a, T> Iterator for ColEleIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: 实际引用的数据是 matrix，所以这里 transmute 不会造成对临时变量应用的泄露。
        if self.idx >= self.row {
            None
        } else {
            let pos = self.idx * self.col + self.col;
            let result = unsafe { self.data.as_ptr().offset(pos as isize).as_mut() };
            self.idx += 1;
            result
        }
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
