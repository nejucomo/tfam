use superstate::interm::IntoNextTerm;
use test_case::test_case;

use superstate::next::{
    Next,
    NextTerm::{self, Terminal},
};

struct ParseQueue<'a, X>(&'a [X]);

impl<'a, X> IntoNextTerm<u8, Result<(), <u8 as TryFrom<X>>::Error>> for ParseQueue<'a, X>
where
    X: Copy,
    u8: TryFrom<X>,
{
    fn into_next_term(self) -> NextTerm<Self, u8, Result<(), <u8 as TryFrom<X>>::Error>> {
        if let Some((&x, q)) = self.0.split_first() {
            match u8::try_from(x) {
                Ok(u) => Next::new(ParseQueue(q), u).into(),
                Err(e) => Terminal(Err(e)),
            }
        } else {
            // We're done:
            Terminal(Ok(()))
        }
    }
}

#[test_case([' ', '\n'] => Ok(52))]
fn parsequeue<X, const K: usize>(xs: [X; K]) -> Result<u8, <u8 as TryFrom<X>>::Error>
where
    X: Copy,
    u8: TryFrom<X>,
{
    let mut sum = 0;

    ParseQueue(xs.as_slice()).for_each_term(|u| {
        sum += u;
        None
    })?;

    Ok(sum)
}
