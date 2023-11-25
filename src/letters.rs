use nalgebra::Matrix5x3;
use std::fmt::Display;
use std::ops::{Add, BitAnd, Shl, Shr, Sub};
mod models;

pub(super) struct Letter<T>(Matrix5x3<T>);

impl Display for Letter<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill_with('X'))
    }
}

impl<T: std::clone::Clone + std::cmp::PartialEq + std::fmt::Debug + 'static> Letter<T> {
    #[allow(dead_code)]
    fn map<F: FnMut(T) -> T>(&self, f: F) -> Self {
        Letter(self.0.map(f))
    }
    fn zip_map<F: FnMut(T, T) -> T>(&self, rhs: &Self, f: F) -> Self {
        Letter(self.0.zip_map(&rhs.0, f))
    }
}

impl Letter<bool> {
    fn fill_with(&self, c: char) -> Matrix5x3<char> {
        self.0.map(|b| match b {
            true => c,
            false => ' ',
        })
    }
}

impl<T: Default + std::clone::Clone + std::cmp::PartialEq + std::fmt::Debug + 'static> Default
    for Letter<T>
{
    fn default() -> Self {
        Self(Matrix5x3::default())
    }
}

impl<T> Add for Letter<T> {
    type Output = Self;
    #[allow(unused_variables)]
    fn add(self, rhs: Self) -> Self::Output {
        todo!("This fn should merge typed letters into the final polymorphic letter.")
    }
}

impl Sub for &Letter<bool> {
    type Output = Letter<bool>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 && !e2)
    }
}

impl Shr for &Letter<bool> {
    type Output = Letter<bool>;
    fn shr(self, rhs: Self) -> Self::Output {
        self - rhs
    }
}

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
