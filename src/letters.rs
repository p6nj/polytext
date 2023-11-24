use nalgebra::Matrix5x3;
use std::fmt::Display;
mod models;

pub(super) struct Letter(Matrix5x3<bool>);

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill_with('X'))
    }
}

impl Letter {
    fn fill_with(&self, c: char) -> Matrix5x3<char> {
        self.0.map(|b| match b {
            true => c,
            false => ' ',
        })
    }
}
