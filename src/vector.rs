use std::{
    default::Default,
    ops::{Add, Mul},
};

pub struct RowVec<T> {
    pub(crate) data: Vec<T>,
}

pub struct ColVec<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Mul<ColVec<T>> for RowVec<T>
where
    T: Mul<T, Output = T> + Add<T, Output = T> + Default,
{
    type Output = T;

    fn mul(self, rhs: ColVec<T>) -> Self::Output {
        self.data.into_iter().zip(rhs.data.into_iter()).fold(
            <T as Default>::default(),
            |sum, item| {
                let diff = item.0 * item.1;
                sum + diff
            },
        )
    }
}
