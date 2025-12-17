use either::Either::{Left, Right};

use crate::updates::{Update, UpdateOption, UpdateTerminal};

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
