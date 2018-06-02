use super::{Decoder, Encoder};
use bytes::{Buf, BufMut, IntoBuf};
use error::{Error, ErrorKind, Result};
use rand::distributions::{Distribution, Standard};

const DEFAULT_CID_LENGTH: usize = 8;
const MIN_CID_LENGTH: usize = 4;
const MAX_CID_LENGTH: usize = 18;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ConnectionID {
    len: usize,
    data: [u8; MAX_CID_LENGTH],
}

impl ConnectionID {
    fn new(len: usize) -> ConnectionID {
        // TODO: assert len
        ConnectionID {
            len: len,
            data: [0; MAX_CID_LENGTH],
        }
    }
}

impl ::std::ops::Deref for ConnectionID {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data[..self.len]
    }
}

impl ::std::ops::DerefMut for ConnectionID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..self.len]
    }
}

impl Decoder for ConnectionID {
    fn decode<T: Buf>(&mut self, src: &mut T) -> Result<usize> {
        src.copy_to_slice(self);
        Ok(self.len)
    }
}

impl Encoder for ConnectionID {
    fn encode<T: BufMut>(&self, dst: &mut T) -> Result<usize> {
        dst.put_slice(self);
        Ok(self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test() {
        let mut cid = ConnectionID::new(8);
        let mut input = vec![
            0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
            0b11111111,
        ];
        cid.decode(&mut input.clone().into_buf());
        assert_eq!(&*cid, input.as_slice());
    }

    #[test]
    fn encode_test() {
        let mut cid = ConnectionID::new(8);
        (&mut cid.data[..cid.len]).clone_from_slice(
            vec![
                0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
                0b11111111,
            ].as_slice(),
        );
        let mut input = vec![];
        cid.encode(&mut input);
        assert_eq!(&*cid, input.as_slice());
    }
}
