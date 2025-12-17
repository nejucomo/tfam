mod froms;

use derive_more::{From, Into};
use either::Either::{self, Right};

use crate::nextstate::NextState;

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
pub struct NextStateTerminal<S, D, T>(TermInner<S, D, T>);

pub(super) type TermInner<S, D, T> = Either<NextState<S, D>, T>;

impl<S, D, T> NextStateTerminal<S, D, T> {
    pub fn new(state: S, data: D) -> Self {
        Self::from(NextState::new(state, data))
    }

    pub fn terminal(term: T) -> Self {
        Self::from(Right(term))
    }
}
