use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, From, Into)]
pub struct Update<S, D> {
    pub state: S,
    pub data: D,
}
