//! `TerminatingSequence` integration tests

use test_case::test_case;
use tfam::seq::TerminalSequence;
use tfam::{StateData, Transition};

// Test treating an `EndlessSequence` as a `TerminatingSequence`:
#[derive(Copy, Clone)]
struct Naturals(usize);

impl Transition for Naturals {
    type Next = StateData<Self, usize>;

    fn into_next(self) -> Self::Next {
        let next = Naturals(self.0 + 1);
        StateData::new(next, next.0)
    }
}

#[test_case(Naturals(0) => 15)]
fn sum_first_five<S>(termseq: S) -> usize
where
    S: TerminalSequence<usize, ()>,
{
    let mut stepsleft = 5;
    let mut sum = 0;

    termseq.for_each_into_term(|u| {
        if stepsleft > 0 {
            sum += u;
            stepsleft -= 1;
            None
        } else {
            Some(())
        }
    });

    sum
}
