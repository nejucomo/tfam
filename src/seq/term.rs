use crate::NextTerm::{self, Next, Terminal};
use crate::maps::MapData as _;
use crate::{StateData, Transition};

/// Any [Transition] into [NextTerm]`<Self, D, T>` is a [TerminalSequence] of `D` outputs terminating with a `T` value
pub trait TerminalSequence<D, T>: Transition<Next: Into<NextTerm<Self, D, T>>> {
    /// Convert from [Transition::into_next] directly into a [NextTerm]
    fn into_next_term(self) -> NextTerm<Self, D, T> {
        self.into_next().into()
    }

    /// Call `f` on each output, then return the terminal
    fn for_each_into_term<F>(self, mut f: F) -> T
    where
        F: FnMut(D) -> Option<T>,
    {
        let mut st = self;

        loop {
            match st.into_next_term().map_data(&mut f) {
                Next(StateData { state, data: None }) => st = state,

                Terminal(term)
                | Next(StateData {
                    data: Some(term), ..
                }) => return term,
            }
        }
    }
}

impl<B, D, T> TerminalSequence<D, T> for B where B: Transition<Next: Into<NextTerm<B, D, T>>> {}
