mod next;

use crate::Transition;
use crate::seq::endless::{Sdata, SdataMap};

use self::next::SdTerm::{Next, Terminal};

pub use self::next::SdTerm;

pub trait SeqTerm<D, T>: Transition<Next: Into<SdTerm<Self, D, T>>> {
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

impl<B, D, T> SeqTerm<D, T> for B where B: Transition<Next: Into<SdTerm<B, D, T>>> {}
