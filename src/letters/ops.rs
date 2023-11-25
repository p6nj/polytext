use super::Letter;
use std::ops::{Add, BitAnd, Shl, Shr, Sub};

impl<T> Add for Letter<T> {
    type Output = Self;
    #[allow(unused_variables)]
    fn add(self, rhs: Self) -> Self::Output {
        todo!("This fn should merge ground layers into the final polymorphic letter.")
    }
}

impl Sub for &Letter<bool> {
    type Output = Letter<bool>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 && !e2)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr for &Letter<bool> {
    type Output = Letter<bool>;
    fn shr(self, rhs: Self) -> Self::Output {
        self - rhs
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Shl for &Letter<bool> {
    type Output = Letter<bool>;
    fn shl(self, rhs: Self) -> Self::Output {
        rhs - self
    }
}

impl BitAnd for &Letter<bool> {
    type Output = Letter<bool>;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 && e2)
    }
}
