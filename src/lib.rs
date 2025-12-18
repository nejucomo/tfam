//! A state transition trait framework
#![deny(unsafe_code)]

mod maps;
mod sd;
mod transition;

pub mod seq;
pub use self::maps::{MapData, MapState};
pub use self::sd::StateData;
pub use self::transition::Transition;
