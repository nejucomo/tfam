use crate::NextTerm::{self, Next, Terminal};
use crate::maps::MapData as _;
use crate::{StateData, Transition};

pub trait SeqTerminal<D, T>: Transition<Next: Into<NextTerm<Self, D, T>>> {
    fn into_next_sdterm(self) -> NextTerm<Self, D, T> {
        self.into_next().into()
    }

    fn for_each_into_term<F>(self, mut f: F) -> T
    where
        F: FnMut(D) -> Option<T>,
    {
        let mut st = self;

        loop {
            match st.into_next_sdterm().map_data(&mut f) {
                Next(StateData { state, data: None }) => st = state,

                Terminal(term)
                | Next(StateData {
                    data: Some(term), ..
                }) => return term,
            }
        }
    }
}

impl<B, D, T> SeqTerminal<D, T> for B where B: Transition<Next: Into<NextTerm<B, D, T>>> {}
