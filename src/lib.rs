//! A trait family around the elementary [Transition] trait
#![deny(unsafe_code)]

mod nterm;
mod sdata;
mod sdopt;
mod transition;

pub mod maps;
pub mod seq;
pub use self::nterm::NextTerm;
pub use self::sdata::StateData;
pub use self::sdopt::StateDataOpt;
pub use self::transition::Transition;
