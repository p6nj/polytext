use nalgebra::Matrix5x3;
use std::fmt::Display;
use std::ops::{Add, BitAnd, Shl, Shr, Sub};
mod models;

pub(super) struct Letter(Matrix5x3<bool>);

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill_with('X'))
    }
}

impl Letter {
    fn map<F: FnMut(bool) -> bool>(&self, f: F) -> Self {
        Letter(self.0.map(f))
    }
    fn zip_map<F: FnMut(bool, bool) -> bool>(&self, rhs: &Self, f: F) -> Self {
        Letter(self.0.zip_map(&rhs.0, f))
    }
    fn fill_with(&self, c: char) -> Matrix5x3<char> {
        self.0.map(|b| match b {
            true => c,
            false => ' ',
        })
    }
}

impl Default for Letter {
    fn default() -> Self {
        Self(Matrix5x3::default())
    }
}

impl Add for Letter {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!("This fn should merge typed letters into the final polymorphic letter.")
    }
}

impl Sub for &Letter {
    type Output = Letter;
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 && !e2)
    }
}

impl Shr for &Letter {
    type Output = Letter;
    fn shr(self, rhs: Self) -> Self::Output {
        self - rhs
    }
}

impl Shl for &Letter {
    type Output = Letter;
    fn shl(self, rhs: Self) -> Self::Output {
        rhs - self
    }
}

impl BitAnd for &Letter {
    type Output = Letter;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, |e1, e2| e1 && e2)
    }
}
