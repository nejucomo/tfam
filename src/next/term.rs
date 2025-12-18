use crate::next::Next;

#[derive(Copy, Clone, Debug)]
pub enum NextTerm<S, D, T> {
    Next(Next<S, D>),
    Terminal(T),
}

impl<S, D, T> From<Next<S, D>> for NextTerm<S, D, T> {
    fn from(ns: Next<S, D>) -> Self {
        NextTerm::Next(ns)
    }
}
