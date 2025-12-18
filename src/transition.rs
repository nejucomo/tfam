pub trait Transition: Sized {
    type Next;

    fn into_next(self) -> Self::Next;
}
