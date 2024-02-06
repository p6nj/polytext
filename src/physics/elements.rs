use crate::letters::Letter;
use std::fmt::Display;
use strum::EnumIter;

#[derive(Debug, Default, Clone, PartialEq, EnumIter)]
pub(crate) enum Element {
    Block,
    Ice,
    Hard,
    On,
    Off,
    Coin,
    #[default]
    Ground,
    FrozenCoin,
    FrozenBlock,
    Void,
}

impl Element {
    fn visible(&self) -> bool {
        !matches!(self, Self::Off | Self::Void)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Element as Into<char>>::into(self))
    }
}

impl From<&Element> for char {
    fn from(el: &Element) -> Self {
        match el {
            Element::Block => 'B',
            Element::Ice => 'I',
            Element::Hard => 'H',
            Element::On => 'O',
            Element::Off => '_',
            Element::Coin => 'C',
            Element::Ground => 'G',
            Element::FrozenCoin => '@',
            Element::FrozenBlock => '$',
            Element::Void => ' ',
        }
    }
}

impl From<Element> for char {
    fn from(value: Element) -> Self {
        (&value).into()
    }
}

impl Display for Letter<Element> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fill(Element::into))
    }
}

impl From<Letter<bool>> for Letter<Element> {
    fn from(value: Letter<bool>) -> Self {
        value.map(|e| match e {
            true => Element::Ground,
            false => Element::Void,
        })
    }
}
