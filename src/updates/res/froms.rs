use either::Either::{Left, Right};

use crate::updates::res::Inner;
use crate::updates::{Update, UpdateOption, UpdateResult, UpdateTerminal};

impl<S, D, E> From<(S, D)> for UpdateResult<S, D, E> {
    fn from(pair: (S, D)) -> Self {
        Update::from(pair).into()
    }
}

impl<S, D, E> From<Update<S, D>> for UpdateResult<S, D, E> {
    fn from(up: Update<S, D>) -> Self {
        Left(up).into()
    }
}

impl<S, D, E> From<UpdateTerminal<S, D, Result<(), E>>> for UpdateResult<S, D, E> {
    fn from(upterm: UpdateTerminal<S, D, Result<(), E>>) -> Self {
        Inner::from(upterm).into()
    }
}

impl<S, D, E> From<Result<(), E>> for UpdateResult<S, D, E> {
    fn from(res: Result<(), E>) -> Self {
        Right(res).into()
    }
}

impl<S, D, E> From<UpdateOption<S, D>> for UpdateResult<S, D, E> {
    fn from(upop: UpdateOption<S, D>) -> Self {
        Option::from(upop).into()
    }
}

impl<S, D, E> From<Option<Update<S, D>>> for UpdateResult<S, D, E> {
    fn from(opup: Option<Update<S, D>>) -> Self {
        opup.map(Self::from).unwrap_or_else(|| Self::terminal_ok())
    }
}
