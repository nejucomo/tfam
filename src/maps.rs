pub trait MapState<S> {
    type MappedState<S2>;

    fn map_state<F, S2>(self, f: F) -> Self::MappedState<S2>
    where
        F: FnOnce(S) -> S2;
}

pub trait MapData<D> {
    type MappedData<D2>;

    fn map_data<F, D2>(self, f: F) -> Self::MappedData<D2>
    where
        F: FnOnce(D) -> D2;
}
