use std::{
    default::Default,
    ops::{AddAssign, Mul},
};

pub struct RowVec<T> {
    pub(crate) data: Vec<T>,
}

pub struct ColVec<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Mul<ColVec<T>> for RowVec<T>
where
    T: Mul<T, Output = T> + Default + AddAssign,
{
    type Output = T;

    fn mul(self, rhs: ColVec<T>) -> Self::Output {
        let mut sum = T::default();
        for num in self.data.into_iter().zip(rhs.data.into_iter()) {
            sum += num.0 * num.1;
        }
        sum
    }
}
