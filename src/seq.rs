//! Blanket extension traits for [Transition](crate::Transition)s which produce sequences of data
mod endless;
mod finite;
mod term;

pub use self::endless::EndlessSequence;
pub use self::finite::Sequence;
pub use self::term::TerminalSequence;
