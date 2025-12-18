use derive_more::{From, Into};

use crate::StateData;

#[derive(Copy, Clone, Debug, From, Into)]
pub struct StateDataOpt<S, D>(Option<StateData<S, D>>);
