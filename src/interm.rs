use crate::next::Next as Nxt;
use crate::next::NextTerm::{self, Next, Terminal};

pub trait IntoNextTerm<D, T>: Sized {
    fn into_next_term(self) -> NextTerm<Self, D, T>;

    fn for_each_term<F>(self, mut f: F) -> T
    where
        F: FnMut(D) -> Option<T>,
    {
        let mut st = self;

        loop {
            match st.into_next_term() {
                Next(Nxt { state, data }) => {
                    if let Some(term) = f(data) {
                        return term;
                    } else {
                        st = state;
                    }
                }
                Terminal(term) => return term,
            }
        }
    }
}
