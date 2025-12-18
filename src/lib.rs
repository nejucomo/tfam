//! A trait family around the elementary [Transition] trait
#![deny(unsafe_code)]

mod transition;

pub mod maps;
pub mod seq;
pub use self::transition::Transition;
