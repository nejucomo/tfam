use crate::seq::endless::{Sdata, SdataMap};

use self::SdTerm::{Next, Terminal};

#[derive(Copy, Clone, Debug)]
pub enum SdTerm<S, D, T> {
    Next(Sdata<S, D>),
    Terminal(T),
}

impl<S, D, T> SdTerm<S, D, T> {
    pub fn map_next<F, SM, DM>(self, f: F) -> SdTerm<SM, DM, T>
    where
        F: FnOnce(Sdata<S, D>) -> Sdata<SM, DM>,
    {
        match self {
            Next(sdata) => Next(f(sdata)),
            Terminal(term) => Terminal(term),
        }
    }
}

impl<S, D, T> From<Sdata<S, D>> for SdTerm<S, D, T> {
    fn from(ns: Sdata<S, D>) -> Self {
        Next(ns)
    }
}

impl<S, D, T> SdataMap<S, D> for SdTerm<S, D, T> {
    type Mapped<SM, DM> = SdTerm<SM, DM, T>;

    fn map_state<F, SM>(self, f: F) -> Self::Mapped<SM, D>
    where
        F: FnOnce(S) -> SM,
    {
        self.map_next(|n| n.map_state(f))
    }

    fn map_data<F, DM>(self, f: F) -> Self::Mapped<S, DM>
    where
        F: FnOnce(D) -> DM,
    {
        self.map_next(|n| n.map_data(f))
    }
}
