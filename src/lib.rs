#![feature(integer_atomics)]
#![feature(exclusive_range_pattern)]
#![feature(range_contains)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate bytes;
extern crate rand;
extern crate rustls;

mod error;
mod protocol;
