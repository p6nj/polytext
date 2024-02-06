use super::Letter;
use std::{
    fmt::Debug,
    ops::{Add, BitAnd, Not, Shl, Shr, Sub},
};

impl<T> Add for Letter<T> {
    type Output = Self;
    #[allow(unused_variables)]
    fn add(self, rhs: Self) -> Self::Output {
        todo!("This fn should merge ground layers into the final polymorphic letter.")
    }
}

impl<T> Sub for &Letter<T>
where
    T: Clone + Debug + PartialEq + Not<Output = T> + BitAnd<T, Output = T> + 'static,
{
    type Output = Letter<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 & !e2)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T> Shr for &Letter<T>
where
    for<'a> &'a Letter<T>: Sub<&'a Letter<T>, Output = Letter<T>>,
{
    type Output = Letter<T>;
    fn shr(self, rhs: Self) -> Self::Output {
        self - rhs
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T> Shl for &Letter<T>
where
    for<'a> &'a Letter<T>: Sub<&'a Letter<T>, Output = Letter<T>>,
{
    type Output = Letter<T>;
    fn shl(self, rhs: Self) -> Self::Output {
        rhs - self
    }
}

impl<T> BitAnd for &Letter<T>
where
    T: Clone + Debug + PartialEq + BitAnd<T, Output = T> + 'static,
{
    type Output = Letter<T>;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 & e2)
    }
}
