use crate::updates::{Update, UpdateOption, UpdateResult, UpdateTerminal};

pub trait Transform: Sized {
    type Into;

    fn transform(self) -> Self::Into;
}

pub trait TransformUpdateOption<D>: Transform
where
    Self::Into: Into<UpdateOption<Self, D>>,
{
}

impl<B, D> TransformUpdateOption<D> for B
where
    B: Transform,
    B::Into: Into<UpdateOption<Self, D>>,
{
}

pub trait TransformUpdateResult<D, E>: Transform
where
    Self::Into: Into<UpdateResult<Self, D, E>>,
{
}

impl<B, D, E> TransformUpdateResult<D, E> for B
where
    B: Transform,
    B::Into: Into<UpdateResult<Self, D, E>>,
{
}

pub trait TransformUpdateTerminal<D, T>: Transform
where
    Self::Into: Into<UpdateTerminal<Self, D, T>>,
{
}

impl<B, D, T> TransformUpdateTerminal<D, T> for B
where
    B: Transform,
    B::Into: Into<UpdateTerminal<Self, D, T>>,
{
}

pub trait TransformUpdate<D>: Transform
where
    Self::Into: Into<Update<Self, D>>,
{
}

impl<B, D> TransformUpdate<D> for B
where
    B: Transform,
    B::Into: Into<Update<Self, D>>,
{
}
