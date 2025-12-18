//! Traits for mapping components

/// Map the contained state
pub trait MapState<S> {
    /// The result of mapping the state
    type MappedState<S2>;

    /// Map the state with `f`
    fn map_state<F, S2>(self, f: F) -> Self::MappedState<S2>
    where
        F: FnOnce(S) -> S2;
}

/// Map the contained data
pub trait MapData<D> {
    /// The result of mapping the data
    type MappedData<D2>;

    /// Map the data with `f`
    fn map_data<F, D2>(self, f: F) -> Self::MappedData<D2>
    where
        F: FnOnce(D) -> D2;
}
