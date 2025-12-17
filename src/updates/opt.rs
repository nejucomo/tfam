use derive_more::{From, Into};

use crate::updates::Update;

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
#[from(Inner<S, D>, Update<S, D>)]
pub struct UpdateOption<S, D>(Inner<S, D>);

type Inner<S, D> = Option<Update<S, D>>;

impl<S, D> From<(S, D)> for UpdateOption<S, D> {
    fn from(pair: (S, D)) -> Self {
        Update::from(pair).into()
    }
}

impl<S, D> UpdateOption<S, D> {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn new(state: S, data: D) -> Self {
        Self::from(Update::new(state, data))
    }
}
