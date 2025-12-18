use crate::StateData;
use crate::maps::{MapData, MapState};

use self::NextTerm::{Next, Terminal};

/// A [Transition::Next](crate::Transition::Next) which can either produce a continuation state, `S`, and an output data, `D`, or else a terminal value `T`
#[derive(Copy, Clone, Debug)]
pub enum NextTerm<S, D, T> {
    /// A continuation state `S` with data `D`
    Next(StateData<S, D>),
    /// A terminal `T`
    Terminal(T),
}

impl<S, D, T> NextTerm<S, D, T> {
    /// Map the [Self::Next] case
    pub fn map_next<F, SM, DM>(self, f: F) -> NextTerm<SM, DM, T>
    where
        F: FnOnce(StateData<S, D>) -> StateData<SM, DM>,
    {
        match self {
            Next(sdata) => Next(f(sdata)),
            Terminal(term) => Terminal(term),
        }
    }
}

impl<S, D, T> From<StateData<S, D>> for NextTerm<S, D, T> {
    fn from(ns: StateData<S, D>) -> Self {
        Next(ns)
    }
}

impl<S, D, T> MapState<S> for NextTerm<S, D, T> {
    type MappedState<S2> = NextTerm<S2, D, T>;

    fn map_state<F, S2>(self, f: F) -> Self::MappedState<S2>
    where
        F: FnOnce(S) -> S2,
    {
        self.map_next(|n| n.map_state(f))
    }
}

impl<S, D, T> MapData<D> for NextTerm<S, D, T> {
    type MappedData<D2> = NextTerm<S, D2, T>;

    fn map_data<F, D2>(self, f: F) -> Self::MappedData<D2>
    where
        F: FnOnce(D) -> D2,
    {
        self.map_next(|n| n.map_data(f))
    }
}
