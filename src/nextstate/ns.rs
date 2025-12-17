use derive_more::{Constructor, From, Into};

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into, Constructor)]
pub struct NextState<S, D> {
    pub state: S,
    pub data: D,
}
