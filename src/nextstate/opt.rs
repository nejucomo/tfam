use derive_more::{From, Into};

use crate::nextstate::NextState;

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
#[from(Inner<S, D>, NextState<S, D>)]
pub struct NextStateOption<S, D>(Inner<S, D>);

type Inner<S, D> = Option<NextState<S, D>>;

impl<S, D> NextStateOption<S, D> {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn new(state: S, data: D) -> Self {
        Self::from(NextState::new(state, data))
    }
}

impl<S, D> From<(S, D)> for NextStateOption<S, D> {
    fn from(pair: (S, D)) -> Self {
        NextState::from(pair).into()
    }
}
