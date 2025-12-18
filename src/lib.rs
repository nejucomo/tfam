//! A state transition trait framework
#![deny(unsafe_code)]

pub mod interm;

pub mod next;

use crate::next::NextTerm;

pub trait IntoNextRes<D, E>: Into<NextTerm<Self, D, Result<(), E>>> {}

impl<B, D, E> IntoNextRes<D, E> for B where B: Into<NextTerm<Self, D, Result<(), E>>> {}
