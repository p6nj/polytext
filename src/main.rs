use std::fmt::Display;

use nalgebra::Matrix5x3;

fn main() {
    ('A'..='Z').map(Letter::new).for_each(|l| println!("{l}"));
}

struct Letter(Matrix5x3<bool>);

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill_with('X'))
    }
}

impl Letter {
    fn new(c: char) -> Self {
        match c {
            'A' => Letter(Matrix5x3::new(
                true, true, true, true, false, true, true, true, true, true, false, true, true,
                false, true,
            )),
            'B' => Letter(Matrix5x3::new(
                true, true, false, true, false, true, true, true, false, true, false, true, true,
                true, false,
            )),
            'C' => Letter(Matrix5x3::new(
                true, true, true, true, false, false, true, false, false, true, false, false, true,
                true, true,
            )),
            'D' => Letter(Matrix5x3::new(
                true, true, false, true, false, true, true, false, true, true, false, true, true,
                true, false,
            )),
            'E' => Letter(Matrix5x3::new(
                true, true, true, true, false, false, true, true, false, true, false, false, true,
                true, true,
            )),
            'F' => Letter(Matrix5x3::new(
                true, true, true, true, false, false, true, true, false, true, false, false, true,
                false, false,
            )),
            'G' => Letter(Matrix5x3::new(
                true, true, true, true, false, false, true, false, false, true, false, true, true,
                true, true,
            )),
            'H' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, true, true, true, true, false, true, true,
                false, true,
            )),
            'I' => Letter(Matrix5x3::new(
                false, true, false, false, false, false, false, true, false, false, true, false,
                false, true, false,
            )),
            'J' => Letter(Matrix5x3::new(
                false, false, true, false, false, true, false, false, true, false, false, true,
                true, true, false,
            )),
            'K' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, true, true, false, true, false, true, true,
                false, true,
            )),
            'L' => Letter(Matrix5x3::new(
                true, false, false, true, false, false, true, false, false, true, false, false,
                true, true, true,
            )),
            'M' => Letter(Matrix5x3::new(
                true, false, true, true, true, true, true, false, true, true, false, true, true,
                false, true,
            )),
            'N' => Letter(Matrix5x3::new(
                true, true, true, true, false, true, true, false, true, true, false, true, true,
                false, true,
            )),
            'O' => Letter(Matrix5x3::new(
                true, true, true, true, false, true, true, false, true, true, false, true, true,
                true, true,
            )),
            'P' => Letter(Matrix5x3::new(
                true, true, true, true, false, true, true, true, true, true, false, false, true,
                false, false,
            )),
            'Q' => Letter(Matrix5x3::new(
                true, true, true, true, false, true, true, true, true, false, false, true, false,
                false, true,
            )),
            'R' => Letter(Matrix5x3::new(
                true, true, false, true, false, true, true, true, false, true, false, true, true,
                false, true,
            )),
            'S' => Letter(Matrix5x3::new(
                false, true, true, true, false, false, false, true, false, false, false, true,
                true, true, false,
            )),
            'T' => Letter(Matrix5x3::new(
                true, true, true, false, true, false, false, true, false, false, true, false,
                false, true, false,
            )),
            'U' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, true, false, true, true, false, true, true,
                true, true,
            )),
            'V' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, true, false, true, true, false, true, false,
                true, false,
            )),
            'W' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, true, false, true, true, true, true, true,
                false, true,
            )),
            'X' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, false, true, false, true, false, true, true,
                false, true,
            )),
            'Y' => Letter(Matrix5x3::new(
                true, false, true, true, false, true, false, true, false, false, true, false,
                false, true, false,
            )),
            'Z' => Letter(Matrix5x3::new(
                true, true, true, false, false, true, false, true, false, true, false, false, true,
                true, true,
            )),
            _ => unimplemented!("This character isn't in the font set."),
        }
    }
    fn fill_with(&self, c: char) -> Matrix5x3<char> {
        self.0.map(|b| match b {
            true => c,
            false => ' ',
        })
    }
}
