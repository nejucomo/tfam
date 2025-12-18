use crate::next::SdTerm::{self, Next, Terminal};
use crate::next::{Sdata, SdataMap};

pub trait Transition: Sized {
    type Next;

    fn into_next(self) -> Self::Next;
}

pub trait TerminatingSequence<D, T>: Transition<Next: Into<SdTerm<Self, D, T>>> {
    fn into_next_sdterm(self) -> SdTerm<Self, D, T> {
        self.into_next().into()
    }

    fn for_each_into_term<F>(self, mut f: F) -> T
    where
        F: FnMut(D) -> Option<T>,
    {
        let mut st = self;

        loop {
            match st.into_next_sdterm().map_data(&mut f) {
                Next(Sdata { state, data: None }) => st = state,

                Terminal(term)
                | Next(Sdata {
                    data: Some(term), ..
                }) => return term,
            }
        }
    }
}

impl<B, D, T> TerminatingSequence<D, T> for B where B: Transition<Next: Into<SdTerm<B, D, T>>> {}
