use crate::letters::Letter;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) enum Element {
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

impl From<Element> for char {
    fn from(el: Element) -> Self {
        match el {
            Element::Block => 'B',
            Element::Ice => 'I',
            Element::Hard => 'H',
            Element::OnOff(on) => match on {
                true => 'O',
                false => '_',
            },
            Element::Coin => 'C',
            Element::Ground => 'G',
            Element::FrozenCoin => '@',
            Element::FrozenBlock => '$',
            Element::Void => ' ',
        }
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
