use super::Element;
use std::ops::Mul;
use strum::EnumIter;

#[derive(EnumIter, PartialEq, Copy, Clone, Debug)]
pub(crate) enum Event {
    Blast,
    P,
    On,
    Off,
    Fire,
    Pow,
}

impl Mul<Event> for Element {
    type Output = Self;
    fn mul(self, rhs: Event) -> Self::Output {
        match self {
            Self::Block => match rhs {
                Event::Blast => Self::Void,
                Event::P => Self::Coin,
                _ => Self::Block,
            },
            Self::Ice | Self::Hard => match rhs {
                Event::Blast => Self::Void,
                _ => Self::Ice,
            },
            Self::On => match rhs {
                Event::Off => Self::Off,
                _ => Self::On,
            },
            Self::Off => match rhs {
                Event::On => Self::On,
                _ => Self::Off,
            },
            Self::Coin => match rhs {
                Event::Blast => Self::Void,
                Event::P => Self::Block,
                Event::Pow => Self::Void,
                _ => Self::Coin,
            },
            Self::FrozenCoin => match rhs {
                Event::Fire => Self::Coin,
                Event::Blast => Self::Void,
                Event::P => Self::FrozenBlock,
                _ => Self::FrozenCoin,
            },
            Self::FrozenBlock => match rhs {
                Event::Fire => Self::Block,
                Event::Blast => Self::Void,
                Event::P => Self::FrozenCoin,
                _ => Self::FrozenBlock,
            },
            other => other,
        }
    }
}
