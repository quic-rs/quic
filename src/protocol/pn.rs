use bytes::{Buf, BufMut};
use codec::*;
use error::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) type PackageNumber = u32;

const MAX_PN_1: PackageNumber = 0b01111111;
const MAX_PN_2: PackageNumber = 0b00111111_11111111;
const MAX_PN_4: PackageNumber = 0b00111111_11111111_11111111_11111111;

const PN_1_FLAG: u8 = 0b0;
const PN_2_FLAG: u8 = 0b10;
const PN_4_FLAG: u8 = 0b11;

#[derive(Debug)]
pub(crate) struct Builder(AtomicU32);

impl Builder {
    // new create a Builder from 0
    pub fn new() -> Builder {
        Builder(AtomicU32::new(0))
    }

    // next get the package number which increase by 1 each call.
    // return None when reach the MAX_PN_4, then must close the connection.
    pub fn next(&self) -> Option<PackageNumber> {
        let pn = self.0.fetch_add(1, Ordering::SeqCst);
        match pn {
            0..MAX_PN_4 => Some(pn),
            _ => None,
        }
    }
}

impl Encoder for PackageNumber {
    fn encode<T: BufMut>(&self, dst: &mut T) -> Result<()> {
        match self {
            0..=MAX_PN_1 => dst.put_uint_be((self | ((PN_1_FLAG as PackageNumber) << 7)) as u64, 1),
            0..=MAX_PN_2 => {
                dst.put_uint_be((self | ((PN_2_FLAG as PackageNumber) << 14)) as u64, 2)
            }
            0..=MAX_PN_4 => {
                dst.put_uint_be((self | ((PN_4_FLAG as PackageNumber) << 30)) as u64, 4)
            }
            v => panic!(
                "package number {} has overfolwn, maximum is {}",
                v, MAX_PN_4
            ),
        }
        Ok(())
    }
}

impl Decoder for PackageNumber {
    fn decode<T: Buf>(src: &mut T) -> Result<PackageNumber> {
        let first = src.get_u8();
        Ok(match first >> 7 {
            PN_1_FLAG => (first as PackageNumber) & MAX_PN_1,
            _ => match first >> 6 {
                PN_2_FLAG => {
                    ((first as PackageNumber) << 8 | (src.get_u8() as PackageNumber)) & MAX_PN_2
                }
                PN_4_FLAG => {
                    ((first as PackageNumber) << 24 | (src.get_uint_be(3) as PackageNumber))
                        & MAX_PN_4
                }
                _ => unreachable!(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::*;
    use codec::*;

    fn decode_pn(input: Vec<u8>) -> PackageNumber {
        PackageNumber::decode(&mut input.into_buf()).unwrap()
    }

    #[test]
    fn decode_pn1_test() {
        assert_eq!(decode_pn(vec![0b00000011]), 3);
    }

    #[test]
    fn decode_pn2_test() {
        assert_eq!(decode_pn(vec![0b10000001, 0b00000001]), 257);
    }

    #[test]
    fn decode_pn4_test() {
        assert_eq!(
            decode_pn(vec![0b11000001, 0b00000001, 0b00000001, 0b0000000]),
            16843008
        );
    }

    fn encode_pn(input: PackageNumber) -> Vec<u8> {
        let mut dst = vec![];
        input.encode(&mut dst).unwrap();
        dst
    }

    #[test]
    fn encode_pn1_test() {
        assert_eq!(encode_pn(3), vec![0b00000011]);
    }

    #[test]
    fn encode_pn2_test() {
        assert_eq!(encode_pn(257), vec![0b10000001, 0b00000001]);
    }

    #[test]
    fn encode_pn4_test() {
        assert_eq!(
            encode_pn(16843009),
            vec![0b11000001, 0b00000001, 0b00000001, 0b0000001],
        );
    }

    #[test]
    fn builder_test() {
        let b = Builder::new();
        assert_eq!(b.next(), Some(0));
        assert_eq!(b.next(), Some(1));
        assert_eq!(b.next(), Some(2));
    }
}
