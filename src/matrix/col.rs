use crate::vector::ColVec;

use super::Matrix;

pub struct Col<'a, T> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) col: usize,
}
impl<'a, T> Col<'a, T> {
    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index >= self.matrix.row {
            None
        } else {
            self.matrix.row(index)?.get(self.col)
        }
    }
    pub fn iter(&self) -> ColEleIter<T> {
        self.into_iter()
    }
}

impl<'a, T> Col<'a, T>
where
    T: Clone,
{
    pub fn to_vec(self) -> ColVec<T> {
        ColVec {
            data: self.into_iter().collect(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Col<'a, T> {
    type Item = &'a T;

    type IntoIter = ColEleIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColEleIter { col: self, idx: 0 }
    }
}
pub struct ColEleIter<'a, T> {
    col: &'a Col<'a, T>,
    idx: usize,
}
impl<'a, T> Iterator for ColEleIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.col.get(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}

impl<'a, T> IntoIterator for Col<'a, T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = IntoColEleIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoColEleIter { col: self, idx: 0 }
    }
}
pub struct IntoColEleIter<'a, T>
where
    T: Clone,
{
    col: Col<'a, T>,
    idx: usize,
}

impl<'a, T> Iterator for IntoColEleIter<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.col.get(self.idx)?.clone();
        self.idx += 1;
        Some(result)
    }
}

pub struct ColMatrix<'a, T> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) idx: usize,
}

impl<'a, T> Iterator for ColMatrix<'a, T> {
    type Item = Col<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.col(self.idx)?;
        self.idx += 1;
        Some(result)
    }
}
