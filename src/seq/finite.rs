use crate::Transition;
use crate::seq::StateDataOpt;

pub trait SeqFinite<D>: Transition<Next: Into<StateDataOpt<Self, D>>> {}
