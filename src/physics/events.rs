use super::Element;
use std::ops::Add;

#[allow(dead_code)]
pub(crate) enum Events {
    Blast,
    P,
    OnOff,
    Fire,
    Pow,
}

impl Add<Events> for Element {
    type Output = Self;
    fn add(self, rhs: Events) -> Self::Output {
        match self {
            Self::Block => match rhs {
                Events::Blast => Self::Void,
                Events::P => Self::Coin,
                _ => Self::Block,
            },
            Self::Ice | Self::Hard => match rhs {
                Events::Blast => Self::Void,
                _ => Self::Ice,
            },
            Self::OnOff(on) => match rhs {
                Events::OnOff => Self::OnOff(!on),
                _ => Self::OnOff(on),
            },
            Self::Coin => match rhs {
                Events::Blast => Self::Void,
                Events::P => Self::Block,
                Events::Pow => Self::Void,
                _ => Self::Coin,
            },
            Self::FrozenCoin => match rhs {
                Events::Fire => Self::Coin,
                Events::Blast => Self::Void,
                Events::P => Self::FrozenBlock,
                _ => Self::FrozenCoin,
            },
            Self::FrozenBlock => match rhs {
                Events::Fire => Self::Block,
                Events::Blast => Self::Void,
                Events::P => Self::FrozenCoin,
                _ => Self::FrozenBlock,
            },
            other => other,
        }
    }
}
