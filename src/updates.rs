mod opt;
mod res;
mod term;
mod up;

pub use self::opt::UpdateOption;
pub use self::res::UpdateResult;
pub use self::term::UpdateTerminal;
pub use self::up::Update;

#[cfg(test)]
mod tests;
