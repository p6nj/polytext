use letters::Letter;
mod letters;

fn main() {
    ('A'..='Z').map(Letter::new).for_each(|l| print!("{l}"));
}
