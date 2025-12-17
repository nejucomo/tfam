use test_case::test_case;

use crate::nextstate::{NextState, NextStateOption, NextStateResult, NextStateTerminal};

#[test_case(('x', 7))]
fn update_from_impl_existence_check<S>(src: S)
where
    NextState<char, u8>: From<S>,
{
    assert_eq!(NextState::from(src), NextState::new('x', 7));
}

#[test_case(('x', 7) => NextStateOption::new('x', 7))]
#[test_case(NextState::new('x', 7) => NextStateOption::new('x', 7))]
#[test_case(Some(NextState::new('x', 7)) => NextStateOption::new('x', 7))]
#[test_case(None => NextStateOption::none())]
fn updateoption_from_impl_existence_check<S>(src: S) -> NextStateOption<char, u8>
where
    NextStateOption<char, u8>: From<S>,
{
    NextStateOption::from(src)
}

#[test_case(('x', 7) => NextStateTerminal::new('x', 7))]
#[test_case(NextState::new('x', 7) => NextStateTerminal::new('x', 7))]
#[test_case(Some(NextState::new('x', 7)) => NextStateTerminal::new('x', 7))]
#[test_case(None => NextStateTerminal::terminal(()))]
fn updateterminal_from_impl_existence_check<S>(src: S) -> NextStateTerminal<char, u8, ()>
where
    NextStateTerminal<char, u8, ()>: From<S>,
{
    NextStateTerminal::from(src)
}

#[test_case(('x', 7) => NextStateResult::new('x', 7))]
#[test_case(NextState::new('x', 7) => NextStateResult::new('x', 7))]
#[test_case(Some(NextState::new('x', 7)) => NextStateResult::new('x', 7))]
#[test_case(None => NextStateResult::terminal_ok())]
#[test_case(Ok(()) => NextStateResult::terminal_ok())]
#[test_case(Err("Uh Oh!") => NextStateResult::terminal_err("Uh Oh!"))]
fn updateresult_from_impl_existence_check<S>(src: S) -> NextStateResult<char, u8, &'static str>
where
    NextStateResult<char, u8, &'static str>: From<S>,
{
    NextStateResult::from(src)
}
