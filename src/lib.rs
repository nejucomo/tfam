//! A state transition trait framework
#![deny(unsafe_code)]

mod transition;

pub mod seq;
pub use self::transition::Transition;
