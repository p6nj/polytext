use std::{fmt::Display, ops::Add};

use crate::letters::Letter;

#[allow(dead_code)]
#[derive(Debug, Default, Clone, PartialEq)]
pub(super) enum Element {
    Block,
    Ice,
    Hard,
    OnOff(bool),
    Coin,
    #[default]
    Ground,
    FrozenCoin,
    FrozenBlock,
    Void,
}

#[allow(dead_code)]
enum Events {
    Blast,
    P,
    OnOff,
    Fire,
    Pow,
}

impl From<Letter<bool>> for Letter<Element> {
    fn from(value: Letter<bool>) -> Self {
        value.map(|e| match e {
            true => Element::Ground,
            false => Element::Void,
        })
    }
}

impl Display for Letter<Element> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill_with(Element::Ground, 'X'))
    }
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
