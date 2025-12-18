mod term;

use derive_more::{Constructor, From, Into};

pub use self::term::NextTerm;

#[derive(Copy, Clone, Debug, From, Into, Constructor)]
pub struct Next<S, D> {
    pub state: S,
    pub data: D,
}
