mod next;

use crate::Transition;

pub use self::next::{Sdata, SdataMap};

pub trait SeqEndless<D>: Transition<Next: Into<Sdata<Self, D>>> {}

impl<B, D> SeqEndless<D> for B where B: Transition<Next: Into<Sdata<Self, D>>> {}
