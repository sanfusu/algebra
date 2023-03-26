use std::{
    mem::transmute,
    ops::{Index, IndexMut},
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

    type IntoIter = ColEleIterMut<'a, 'b, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: 'self >= ('a,'b)，因此可以安全的转换
        unsafe {
            transmute::<ColEleIterMut<'a, '_, T>, ColEleIterMut<'a, 'b, T>>(ColEleIterMut {
                col: self,
                idx: 0,
            })
        }
    }
}

pub struct ColEleIterMut<'a: 'b, 'b, T> {
    col: &'b mut ColMut<'a, T>,
    idx: usize,
}
impl<'a, 'b, T> Iterator for ColEleIterMut<'a, 'b, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: self.col.get 返回的生命周期为 'self，但是需要的是 'a
        // 由于 'self >= 'a，所以转换是安全的
        let src = self.col.get(self.idx);
        let dst = unsafe { transmute::<Option<&'_ mut T>, Option<&'a mut T>>(src) }?;
        self.idx += 1;
        Some(dst)
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
