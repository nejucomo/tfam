use derive_more::{From, Into};
use either::Either::{Left, Right};

use crate::updates::term::TermInner;
use crate::updates::{Update, UpdateOption, UpdateTerminal};

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
pub struct UpdateResult<S, D, E>(Inner<S, D, E>);

type Inner<S, D, E> = TermInner<S, D, Result<(), E>>;

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

impl<S, D, E> UpdateResult<S, D, E> {
    pub fn new(state: S, data: D) -> Self {
        Self::from(Update::new(state, data))
    }

    pub fn terminal_ok() -> Self {
        Ok(()).into()
    }

    pub fn terminal_err(err: E) -> Self {
        Err(err).into()
    }
}
