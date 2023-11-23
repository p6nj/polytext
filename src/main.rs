use std::fmt::Display;

use nalgebra::Matrix5x3;

fn main() {
    let a = Letter::new('A');
    println!("{a}");
}

struct Letter(Matrix5x3<bool>);

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
            'D' => Letter(Matrix5x3::default()),
            'E' => Letter(Matrix5x3::default()),
            'F' => Letter(Matrix5x3::default()),
            'G' => Letter(Matrix5x3::default()),
            'H' => Letter(Matrix5x3::default()),
            'I' => Letter(Matrix5x3::default()),
            'J' => Letter(Matrix5x3::default()),
            'K' => Letter(Matrix5x3::default()),
            'L' => Letter(Matrix5x3::default()),
            'M' => Letter(Matrix5x3::default()),
            'N' => Letter(Matrix5x3::default()),
            'O' => Letter(Matrix5x3::default()),
            'P' => Letter(Matrix5x3::default()),
            'Q' => Letter(Matrix5x3::default()),
            'R' => Letter(Matrix5x3::default()),
            'S' => Letter(Matrix5x3::default()),
            'T' => Letter(Matrix5x3::default()),
            'U' => Letter(Matrix5x3::default()),
            'V' => Letter(Matrix5x3::default()),
            'W' => Letter(Matrix5x3::default()),
            'X' => Letter(Matrix5x3::default()),
            'Y' => Letter(Matrix5x3::default()),
            'Z' => Letter(Matrix5x3::default()),
            _ => unimplemented!("This character isn't in the font set."),
        }
    }
}
