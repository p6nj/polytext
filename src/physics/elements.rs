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
