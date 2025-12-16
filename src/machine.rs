/// The most general purpose state transition widget
pub trait Machine<Input> {
    /// The result of combining `self` and `Input`
    type Output;

    /// Update `self` with `input` to produce [Self::Output]
    fn update(self, input: Input) -> Self::Output;
}
