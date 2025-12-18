use crate::StateData;
use crate::Transition;

/// Any [Transition] into [StateData]`<Self, D>` is an [EndlessSequence] of `D` outputs
pub trait EndlessSequence<D>: Transition<Next: Into<StateData<Self, D>>> {}

impl<B, D> EndlessSequence<D> for B where B: Transition<Next: Into<StateData<Self, D>>> {}
