use crate::Transition;
use crate::{StateData, StateDataOpt};

/// Any [Transition] into [StateDataOpt]`<Self, D>` is a (finite) [Sequence] of `D` outputs
pub trait Sequence<D>: Transition<Next: Into<StateDataOpt<Self, D>>> {
    /// Convert from [Transition::into_next] directly into `Option<StateData<Self, D>>`
    fn into_next_opt(self) -> Option<StateData<Self, D>> {
        self.into_next().into().into()
    }
}
