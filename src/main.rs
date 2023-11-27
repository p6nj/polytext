use letters::Letter;

use physics::Element;
mod letters;
mod physics;

fn main() {
    print!(
        "{}",
        Letter::<Element>::from(&Letter::new('M') >> &Letter::new('N'))
    );
    print!(
        "{}",
        Letter::<Element>::from(&Letter::new('M') << &Letter::new('N'))
    );
}
