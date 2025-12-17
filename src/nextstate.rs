mod ns;
mod opt;
mod res;
mod term;

pub use self::ns::NextState;
pub use self::opt::NextStateOption;
pub use self::res::NextStateResult;
pub use self::term::NextStateTerminal;

#[cfg(test)]
mod tests;
