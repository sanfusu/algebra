use self::{
    col::{Col, ColMatrix},
    col_mut::ColMut,
    raw::RawMatrix,
    row::{Row, RowMatrix},
    row_mut::RowMut,
};
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::NonNull,
    usize,
};

pub mod col;
pub mod col_mut;
pub mod ops;
pub(crate) mod raw;
pub mod row;
pub mod row_mut;
#[derive(Debug)]
pub struct MatrixRearrangeFailed;

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    pub col: usize,
    pub row: usize,
}
impl<T> Matrix<T> {
    pub(crate) fn as_raw(&mut self) -> RawMatrix<T> {
        RawMatrix {
            data: unsafe { NonNull::new_unchecked(self.data.as_mut_ptr()) },
            col: self.col,
            row: self.row,
            phantom: PhantomData,
        }
    }
    pub fn new(data: Vec<T>) -> Self {
        Self {
            col: data.len(),
            data,
            row: 1,
        }
    }
    pub fn rearrange(self, row: usize, col: usize) -> Result<Self, MatrixRearrangeFailed> {
        if row * col != self.data.len() {
            Err(MatrixRearrangeFailed)
        } else {
            Ok(Self { col, row, ..self })
        }
    }
    pub fn as_rowmatrix(&self) -> RowMatrix<T> {
        RowMatrix {
            matrix: self,
            idx: 0,
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
    pub fn row_mut<'a>(&'a mut self, index: usize) -> Option<RowMut<'a, T>> {
        if index >= self.row {
            None
        } else {
            Some(RowMut {
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
    pub fn col_mut(&mut self, index: usize) -> Option<ColMut<T>> {
        if index >= self.col {
            None
        } else {
            Some(ColMut {
                matrix: self,
                col: index,
            })
        }
    }
    pub fn as_colmatrix(&self) -> ColMatrix<T> {
        ColMatrix {
            matrix: self,
            idx: 0,
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.col;
        let end = start + self.col;
        &self.data[start..end]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.col;
        let end = start + self.col;
        &mut self.data[start..end]
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    fn create_matrix() -> Matrix<u8> {
        Matrix::new(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
    #[test]
    fn matrix_rearrange_test() {
        let matrix = create_matrix();
        let m2x5 = matrix.rearrange(2, 5).unwrap();
        for row in m2x5.as_rowmatrix() {
            println!("{:?}", row.as_slice());
        }
        let m5x2 = m2x5.rearrange(5, 2).unwrap();
        for row in m5x2.as_rowmatrix() {
            println!("{:?}", row.as_slice());
        }
    }
    #[test]
    fn matrix_add_test() {
        let m5x2 = create_matrix().rearrange(5, 2).unwrap();
        let m5x2_clone = m5x2.clone();

        let m5x2_sum = m5x2 + m5x2_clone;

        for row in m5x2_sum.as_rowmatrix() {
            println!("{:?}", row.as_slice());
        }
    }
    #[test]
    fn matrix_col_iter() {
        let m5x2 = create_matrix().rearrange(5, 2).unwrap();
        let _col1 = m5x2.col(1).unwrap();
        println!("{:?}", m5x2.data);
    }
    #[test]
    fn matrix_transpose() {
        let m2x5 = Matrix::new(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            .rearrange(2, 5)
            .unwrap();
        for row in m2x5.as_rowmatrix() {
            println!("{:?}", row.as_slice());
        }
        let m5x2 = m2x5.transpose();
        for row in m5x2.as_rowmatrix() {
            println!("{:?}", row.as_slice());
        }
    }
    #[test]
    fn matrix_iter_mut() {
        let mut m3x3 = Matrix::new(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8])
            .rearrange(3, 3)
            .unwrap();
        let mut binding = m3x3.row_mut(2).unwrap();
        let _row2 = binding.as_slice();
        for ele in _row2 {
            *ele *= 2;
        }
        println!("{:?}", m3x3.data);
        let mut col = m3x3.col_mut(2).unwrap();

        col.iter_mut().for_each(|ele| {
            *ele *= 2;
        });
        col[0] = 1;
        col[1] = 1;

        m3x3.col_mut(0).unwrap()[0] = 1;
        println!("{:?}", m3x3);
    }
}
