#![feature(integer_atomics)]
#![feature(exclusive_range_pattern)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate bytes;
extern crate rustls;

mod codec;
mod error;
mod protocol;
