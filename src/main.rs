use letters::Letter;
mod letters;

fn main() {
    print!("{}", &Letter::new('M') >> &Letter::new('N'));
}
