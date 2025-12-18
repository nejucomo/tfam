mod term;

use derive_more::{Constructor, From, Into};

pub use self::term::SdTerm;

#[derive(Copy, Clone, Debug, From, Into, Constructor)]
pub struct Sdata<S, D> {
    pub state: S,
    pub data: D,
}

pub trait SdataMap<S, D> {
    type Mapped<SM, DM>;

    fn map_state<F, SM>(self, f: F) -> Self::Mapped<SM, D>
    where
        F: FnOnce(S) -> SM;

    fn map_data<F, DM>(self, f: F) -> Self::Mapped<S, DM>
    where
        F: FnOnce(D) -> DM;
}

impl<S, D> SdataMap<S, D> for Sdata<S, D> {
    type Mapped<SM, DM> = Sdata<SM, DM>;

    fn map_state<F, SM>(self, f: F) -> Self::Mapped<SM, D>
    where
        F: FnOnce(S) -> SM,
    {
        Sdata {
            state: f(self.state),
            data: self.data,
        }
    }

    fn map_data<F, DM>(self, f: F) -> Self::Mapped<S, DM>
    where
        F: FnOnce(D) -> DM,
    {
        Sdata {
            state: self.state,
            data: f(self.data),
        }
    }
}
