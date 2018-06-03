use super::{Decoder, Encoder};
use bytes::{Buf, BufMut, IntoBuf};
use error::{Error, ErrorKind, Result};
use rand;
use std;

const DEFAULT_CID_LENGTH: usize = 8;
const MIN_CID_LENGTH: usize = 4;
const MAX_CID_LENGTH: usize = 18;
const CID_LENGTH_RANGE: std::ops::RangeInclusive<usize> = MIN_CID_LENGTH..=MAX_CID_LENGTH;

#[derive(Debug, Eq, PartialEq)]
pub struct ConnectionID {
    len: usize,
    data: [u8; MAX_CID_LENGTH],
}

impl ConnectionID {
    pub fn new(len: usize) -> ConnectionID {
        assert!(CID_LENGTH_RANGE.contains(&len));
        ConnectionID {
            len: len,
            data: [0; MAX_CID_LENGTH],
        }
    }

    pub fn generate() -> ConnectionID {
        rand::random()
    }

    pub fn len(&self) -> usize {
        // just safe to minus 3 because of the len field cannot be less than MIN_CID_LENGTH
        self.len - 3
    }
}

impl rand::distributions::Distribution<ConnectionID> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ConnectionID {
        let mut cid = ConnectionID::new(DEFAULT_CID_LENGTH);
        rng.fill_bytes(&mut cid);
        cid
    }
}

impl std::ops::Deref for ConnectionID {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data[..self.len]
    }
}

impl std::ops::DerefMut for ConnectionID {
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
    fn rand_test() {
        let cid = ConnectionID::generate();
        println!("{:?}", cid);
    }

    #[test]
    fn range_test() {
        assert_eq!(CID_LENGTH_RANGE.contains(&MIN_CID_LENGTH), true);
        assert_eq!(CID_LENGTH_RANGE.contains(&8), true);
        assert_eq!(CID_LENGTH_RANGE.contains(&MAX_CID_LENGTH), true);

        assert_eq!(CID_LENGTH_RANGE.contains(&0), false);
        assert_eq!(CID_LENGTH_RANGE.contains(&20), false);
    }

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
