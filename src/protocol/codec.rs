use bytes::{Buf, BufMut};
use error::Result;

pub(super) trait Encoder {
    fn encode<T: BufMut>(&self, dst: &mut T) -> Result<usize>;
}

pub(super) trait Decoder: Sized {
    fn decode<T: Buf>(&mut self, src: &mut T) -> Result<usize>;
}
