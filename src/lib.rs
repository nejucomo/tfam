#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs)]

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
