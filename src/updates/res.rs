mod froms;

use derive_more::{From, Into};

use crate::updates::Update;
use crate::updates::term::TermInner;

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, Into)]
pub struct UpdateResult<S, D, E>(Inner<S, D, E>);

type Inner<S, D, E> = TermInner<S, D, Result<(), E>>;

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
