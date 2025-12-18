use derive_more::{Constructor, From, Into};

use crate::maps::{MapData, MapState};

#[derive(Copy, Clone, Debug, From, Into, Constructor)]
pub struct SeqNext<S, D> {
    pub state: S,
    pub data: D,
}

impl<S, D> MapState<S> for SeqNext<S, D> {
    type MappedState<S2> = SeqNext<S2, D>;

    fn map_state<F, S2>(self, f: F) -> Self::MappedState<S2>
    where
        F: FnOnce(S) -> S2,
    {
        SeqNext {
            state: f(self.state),
            data: self.data,
        }
    }
}

impl<S, D> MapData<D> for SeqNext<S, D> {
    type MappedData<D2> = SeqNext<S, D2>;

    fn map_data<F, D2>(self, f: F) -> Self::MappedData<D2>
    where
        F: FnOnce(D) -> D2,
    {
        SeqNext {
            state: self.state,
            data: f(self.data),
        }
    }
}
