/// Types which transition into a [Self::Next]
pub trait Transition: Sized {
    /// The type transitioned into
    type Next;

    /// Transition into [Self::Next]
    fn into_next(self) -> Self::Next;
}
