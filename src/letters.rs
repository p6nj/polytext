use nalgebra::Matrix5x3;
use std::fmt::Display;
mod models;
mod ops;

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
