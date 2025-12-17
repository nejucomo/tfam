mod froms;

use derive_more::{From, Into};
use either::Either::{self, Right};

use crate::updates::Update;

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
pub struct UpdateTerminal<S, D, T>(TermInner<S, D, T>);

pub(super) type TermInner<S, D, T> = Either<Update<S, D>, T>;

impl<S, D, T> UpdateTerminal<S, D, T> {
    pub fn new(state: S, data: D) -> Self {
        Self::from(Update::new(state, data))
    }

    pub fn terminal(term: T) -> Self {
        Self::from(Right(term))
    }
}
