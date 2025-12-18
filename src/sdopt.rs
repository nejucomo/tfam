use derive_more::{From, Into};

use crate::StateData;

/// An optional [StateData] update; isomorphic to `Option<StateData<S, D>>` and `NextTerm<S, D, ()>`
#[derive(Copy, Clone, Debug, From, Into)]
pub struct StateDataOpt<S, D>(Option<StateData<S, D>>);
