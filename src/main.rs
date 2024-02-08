#![allow(dead_code)]
use physics::{paths, Element, Event};
mod letters;
mod physics;
use strum::IntoEnumIterator;

fn main() {
    println!("{}", Element::Coin + Event::P);
    // Element::iter().for_each(|e| println!("{e}"));
    dbg!(paths(Element::Block));
}
