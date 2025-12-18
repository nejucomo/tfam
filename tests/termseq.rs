//! `TerminatingSequence` integration tests

use superstate::transition::{TerminatingSequence, Transition};
use test_case::test_case;

use superstate::next::Sdata;

// Test treating an `EndlessSequence` as a `TerminatingSequence`:
#[derive(Copy, Clone)]
struct Naturals(usize);

impl Transition for Naturals {
    type Next = Sdata<Self, usize>;

    fn into_next(self) -> Self::Next {
        let next = Naturals(self.0 + 1);
        Sdata::new(next, next.0)
    }
}

#[test_case(Naturals(0) => 15)]
fn sum_first_five<S>(termseq: S) -> usize
where
    S: TerminatingSequence<usize, ()>,
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
