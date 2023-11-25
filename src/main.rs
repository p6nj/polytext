use letters::Letter;

use elements::Element;
mod elements;
mod letters;

fn main() {
    print!(
        "{}",
        Letter::<Element>::from(&Letter::new('M') >> &Letter::new('N'))
    );
    print!("{}", &Letter::new('M') << &Letter::new('N'));
}
