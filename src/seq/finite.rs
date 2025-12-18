use crate::StateDataOpt;
use crate::Transition;

pub trait SeqFinite<D>: Transition<Next: Into<StateDataOpt<Self, D>>> {}
