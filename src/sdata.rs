use derive_more::{Constructor, From, Into};

use crate::maps::{MapData, MapState};

/// A state `S` and associated data `D`
#[derive(Copy, Clone, Debug, From, Into, Constructor)]
pub struct StateData<S, D> {
    /// The state
    pub state: S,
    /// The data
    pub data: D,
}

impl<S, D> MapState<S> for StateData<S, D> {
    type MappedState<S2> = StateData<S2, D>;

    fn map_state<F, S2>(self, f: F) -> Self::MappedState<S2>
    where
        F: FnOnce(S) -> S2,
    {
        StateData {
            state: f(self.state),
            data: self.data,
        }
    }
}

impl<S, D> MapData<D> for StateData<S, D> {
    type MappedData<D2> = StateData<S, D2>;

    fn map_data<F, D2>(self, f: F) -> Self::MappedData<D2>
    where
        F: FnOnce(D) -> D2,
    {
        StateData {
            state: self.state,
            data: f(self.data),
        }
    }
}
