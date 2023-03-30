use std::ops::{Index, IndexMut};

use crate::vector::ColVec;

use super::{col::Col, raw::RawColMut, Matrix};

pub struct ColMut<'a, T> {
    pub(crate) matrix: &'a mut Matrix<T>,
    pub(crate) col: usize,
}

// self 的类型为 ColMut<'a,T> 因此这里不需要 &'a mut self 就可以确保生命周期的一致性
impl<'a, T> ColMut<'a, T> {
    pub(crate) fn as_raw(&mut self) -> RawColMut<'a, T> {
        RawColMut {
            // SAFETY: ColMut<'a,T> 只能由 matrix.col_mut() 创建，而 matrix 的生命周期
            // 和其返回结果一致。因此可以约束 RawColMut 中的引用与 self.matrix 的生命周期一致。
            matrix: unsafe { self.matrix.as_raw() },
            col: self.col,
        }
    }
    pub fn get(&mut self, index: usize) -> Option<&'a mut T> {
        self.as_raw().get(index)
    }
    pub fn iter_mut(&mut self) -> IterMut<'a, T> {
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

/// 对 ColMut 的引用的迭代器
pub struct IterMut<'a, T: 'a> {
    // 这里使用 &'b ColMut<'a,T> 会导致 Iterator::next 难以实现
    // 因为 Iterator::next(&mut self) 中的 self 无法指定生命周期。
    col: RawColMut<'a, T>,
    idx: usize,
}
impl<'a, T> IntoIterator for &mut ColMut<'a, T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.col.get(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}

pub struct IntoIter<'a, T: 'a> {
    col: ColMut<'a, T>,
    idx: usize,
}
impl<'a, T: 'a> Iterator for IntoIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.col.as_raw();
        let result = raw.get(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}
impl<'a, T> IntoIterator for ColMut<'a, T> {
    type Item = &'a mut T;

    type IntoIter = IntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { col: self, idx: 0 }
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
