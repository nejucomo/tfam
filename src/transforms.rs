use crate::nextstate::{NextState, NextStateOption, NextStateResult, NextStateTerminal};

pub trait Transform: Sized {
    type Into;

    fn transform(self) -> Self::Into;
}

pub trait UpdateOption<D>: Transform
where
    Self::Into: Into<NextStateOption<Self, D>>,
{
}

impl<B, D> UpdateOption<D> for B
where
    B: Transform,
    B::Into: Into<NextStateOption<Self, D>>,
{
}

pub trait UpdateResult<D, E>: Transform
where
    Self::Into: Into<NextStateResult<Self, D, E>>,
{
}

impl<B, D, E> UpdateResult<D, E> for B
where
    B: Transform,
    B::Into: Into<NextStateResult<Self, D, E>>,
{
}

pub trait UpdateTerminal<D, T>: Transform
where
    Self::Into: Into<NextStateTerminal<Self, D, T>>,
{
}

impl<B, D, T> UpdateTerminal<D, T> for B
where
    B: Transform,
    B::Into: Into<NextStateTerminal<Self, D, T>>,
{
}

pub trait Update<D>: Transform
where
    Self::Into: Into<NextState<Self, D>>,
{
}

impl<B, D> Update<D> for B
where
    B: Transform,
    B::Into: Into<NextState<Self, D>>,
{
}
