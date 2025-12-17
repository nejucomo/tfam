use test_case::test_case;

use crate::updates::{Update, UpdateOption, UpdateResult, UpdateTerminal};

#[test_case(('x', 7))]
fn update_from_impl_existence_check<S>(src: S)
where
    Update<char, u8>: From<S>,
{
    assert_eq!(Update::from(src), Update::new('x', 7));
}

#[test_case(('x', 7) => UpdateOption::new('x', 7))]
#[test_case(Update::new('x', 7) => UpdateOption::new('x', 7))]
#[test_case(Some(Update::new('x', 7)) => UpdateOption::new('x', 7))]
#[test_case(None => UpdateOption::none())]
fn updateoption_from_impl_existence_check<S>(src: S) -> UpdateOption<char, u8>
where
    UpdateOption<char, u8>: From<S>,
{
    UpdateOption::from(src)
}

#[test_case(('x', 7) => UpdateTerminal::new('x', 7))]
#[test_case(Update::new('x', 7) => UpdateTerminal::new('x', 7))]
#[test_case(Some(Update::new('x', 7)) => UpdateTerminal::new('x', 7))]
#[test_case(None => UpdateTerminal::terminal(()))]
fn updateterminal_from_impl_existence_check<S>(src: S) -> UpdateTerminal<char, u8, ()>
where
    UpdateTerminal<char, u8, ()>: From<S>,
{
    UpdateTerminal::from(src)
}

#[test_case(('x', 7) => UpdateResult::new('x', 7))]
#[test_case(Update::new('x', 7) => UpdateResult::new('x', 7))]
#[test_case(Some(Update::new('x', 7)) => UpdateResult::new('x', 7))]
#[test_case(None => UpdateResult::terminal_ok())]
#[test_case(Ok(()) => UpdateResult::terminal_ok())]
#[test_case(Err("Uh Oh!") => UpdateResult::terminal_err("Uh Oh!"))]
fn updateresult_from_impl_existence_check<S>(src: S) -> UpdateResult<char, u8, &'static str>
where
    UpdateResult<char, u8, &'static str>: From<S>,
{
    UpdateResult::from(src)
}
