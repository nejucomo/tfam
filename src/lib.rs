//! A state transition trait framework
#![deny(unsafe_code)]

pub trait Transformer<Input, Output> {
    fn transform(self, input: Input) -> Output;
}
