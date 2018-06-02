use failure::Context;

#[derive(Debug)]
pub struct Error(Context<ErrorKind>);

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Io(#[cause] ::std::io::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
