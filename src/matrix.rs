use std::{ops::Index, usize};

pub struct Matrix<T> {
    _data: Vec<T>,
    pub col: usize,
    pub row: usize,
}
impl<T> Matrix<T> {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            _data: Vec::new(),
            col,
            row,
        }
    }
    pub fn row(&self, index: usize) -> Option<Row<T>> {
        if index >= self.row {
            None
        } else {
            Some(Row {
                matrix: self,
                row: index,
            })
        }
    }
    pub fn col(&self, index: usize) -> Option<Col<T>> {
        if index >= self.col {
            None
        } else {
            Some(Col {
                matrix: self,
                col: index,
            })
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.row(index).unwrap().as_slice()
    }
}

pub struct Row<'a, T> {
    matrix: &'a Matrix<T>,
    row: usize,
}
impl<'a, T> Row<'a, T> {
    pub fn as_slice(&self) -> &'a [T] {
        let start = self.row * self.matrix.col;
        let end = start + self.matrix.col;
        &self.matrix._data[start..end]
    }
    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index >= self.matrix.col {
            None
        } else {
            Some(&self.as_slice()[index])
        }
    }
}
impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[self.row][index]
    }
}

pub struct Col<'a, T> {
    matrix: &'a Matrix<T>,
    col: usize,
}
impl<'a, T> Col<'a, T> {
    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index >= self.matrix.row {
            None
        } else {
            self.matrix.row(index)?.get(self.col)
        }
    }
}
impl<'a, T> IntoIterator for Col<'a, T> {
    type Item = &'a T;

    type IntoIter = ColEleIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColEleIter { col: self, idx: 0 }
    }
}
pub struct ColEleIter<'a, T> {
    col: Col<'a, T>,
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
