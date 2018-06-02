use bytes::{Buf, BufMut};
use error::*;

pub(crate) trait Encoder {
    fn encode<T: BufMut>(&self, dst: &mut T) -> Result<()>;
}

pub(crate) trait Decoder: Sized {
    fn decode<T: Buf>(src: &mut T) -> Result<Self>;
}
