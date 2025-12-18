mod next;

use crate::{MapData, StateData, Transition};

use self::next::SeqTermNext::{Next, Terminal};

pub use self::next::SeqTermNext;

pub trait SeqTerminal<D, T>: Transition<Next: Into<SeqTermNext<Self, D, T>>> {
    fn into_next_sdterm(self) -> SeqTermNext<Self, D, T> {
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

impl<B, D, T> SeqTerminal<D, T> for B where B: Transition<Next: Into<SeqTermNext<B, D, T>>> {}
