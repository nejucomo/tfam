use either::Either::{Left, Right};

use crate::nextstate::{NextState, NextStateOption, NextStateTerminal};

impl<S, D, T> From<NextState<S, D>> for NextStateTerminal<S, D, T> {
    fn from(up: NextState<S, D>) -> Self {
        Left(up).into()
    }
}

impl<S, D, T> From<(S, D)> for NextStateTerminal<S, D, T> {
    fn from(up: (S, D)) -> Self {
        NextState::from(up).into()
    }
}

impl<S, D> From<NextStateOption<S, D>> for NextStateTerminal<S, D, ()> {
    fn from(upop: NextStateOption<S, D>) -> Self {
        Option::from(upop).into()
    }
}

impl<S, D> From<Option<NextState<S, D>>> for NextStateTerminal<S, D, ()> {
    fn from(opup: Option<NextState<S, D>>) -> Self {
        opup.map(Left).unwrap_or(Right(())).into()
    }
}
