use either::Either::{Left, Right};

use crate::nextstate::res::Inner;
use crate::nextstate::{NextState, NextStateOption, NextStateResult, NextStateTerminal};

impl<S, D, E> From<(S, D)> for NextStateResult<S, D, E> {
    fn from(pair: (S, D)) -> Self {
        NextState::from(pair).into()
    }
}

impl<S, D, E> From<NextState<S, D>> for NextStateResult<S, D, E> {
    fn from(up: NextState<S, D>) -> Self {
        Left(up).into()
    }
}

impl<S, D, E> From<NextStateTerminal<S, D, Result<(), E>>> for NextStateResult<S, D, E> {
    fn from(upterm: NextStateTerminal<S, D, Result<(), E>>) -> Self {
        Inner::from(upterm).into()
    }
}

impl<S, D, E> From<Result<(), E>> for NextStateResult<S, D, E> {
    fn from(res: Result<(), E>) -> Self {
        Right(res).into()
    }
}

impl<S, D, E> From<NextStateOption<S, D>> for NextStateResult<S, D, E> {
    fn from(upop: NextStateOption<S, D>) -> Self {
        Option::from(upop).into()
    }
}

impl<S, D, E> From<Option<NextState<S, D>>> for NextStateResult<S, D, E> {
    fn from(opup: Option<NextState<S, D>>) -> Self {
        opup.map(Self::from).unwrap_or_else(|| Self::terminal_ok())
    }
}
