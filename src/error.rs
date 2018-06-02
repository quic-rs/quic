use failure::Context;

#[derive(Debug)]
pub struct Error(Context<ErrorKind>);

#[derive(Debug, Fail)]
pub enum ErrorKind {}

pub type Result<T> = ::std::result::Result<T, Error>;
