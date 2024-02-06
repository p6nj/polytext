#![allow(dead_code)]
use physics::{Element,Event};
mod letters;
mod physics;

fn main() {
    println!("{}", Element::Coin + Event::P);
}
