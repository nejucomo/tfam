use crate::{StateData, Transition};

pub trait SeqEndless<D>: Transition<Next: Into<StateData<Self, D>>> {}

impl<B, D> SeqEndless<D> for B where B: Transition<Next: Into<StateData<Self, D>>> {}
