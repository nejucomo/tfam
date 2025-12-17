use derive_more::{From, Into};
use either::Either::{self, Left, Right};

use crate::updates::{Update, UpdateOption};

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
pub struct UpdateTerminal<S, D, T>(TermInner<S, D, T>);

pub(super) type TermInner<S, D, T> = Either<Update<S, D>, T>;

impl<S, D, T> From<Update<S, D>> for UpdateTerminal<S, D, T> {
    fn from(up: Update<S, D>) -> Self {
        Left(up).into()
    }
}

impl<S, D, T> From<(S, D)> for UpdateTerminal<S, D, T> {
    fn from(up: (S, D)) -> Self {
        Update::from(up).into()
    }
}

impl<S, D> From<UpdateOption<S, D>> for UpdateTerminal<S, D, ()> {
    fn from(upop: UpdateOption<S, D>) -> Self {
        Option::from(upop).into()
    }
}

impl<S, D> From<Option<Update<S, D>>> for UpdateTerminal<S, D, ()> {
    fn from(opup: Option<Update<S, D>>) -> Self {
        opup.map(Left).unwrap_or(Right(())).into()
    }
}

impl<S, D, T> UpdateTerminal<S, D, T> {
    pub fn new(state: S, data: D) -> Self {
        Self::from(Update::new(state, data))
    }

    pub fn terminal(term: T) -> Self {
        Self::from(Right(term))
    }
}
