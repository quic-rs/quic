use super::{Decoder, Encoder};
use bytes::{Buf, BufMut, IntoBuf};
use error::{Error, ErrorKind, Result};
use std::sync::atomic::{AtomicU32, Ordering};

const MAX_PN_1: u32 = 0b01111111;
const MAX_PN_2: u32 = 0b00111111_11111111;
const MAX_PN_4: u32 = 0b00111111_11111111_11111111_11111111;

const PN_1_FLAG: u8 = 0b0;
const PN_2_FLAG: u8 = 0b10;
const PN_4_FLAG: u8 = 0b11;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) struct PackageNumber(u32);

impl ::std::convert::Into<u32> for PackageNumber {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub(crate) struct Generater(AtomicU32);

impl Generater {
    // new create a Generater from 0
    pub fn new() -> Generater {
        Generater(0.into())
    }

    // next get the package number which increase by 1 each call.
    // return None when reach the MAX_PN_4, then must close the connection.
    pub fn next(&self) -> Option<PackageNumber> {
        let pn = self.0.fetch_add(1, Ordering::SeqCst);
        match pn {
            0..MAX_PN_4 => Some(PackageNumber(pn)),
            _ => None,
        }
    }
}

impl Encoder for PackageNumber {
    fn encode<T: BufMut>(&self, dst: &mut T) -> Result<usize> {
        Ok(match self.0 {
            0..=MAX_PN_1 => {
                dst.put_uint_be((self.0 | ((PN_1_FLAG as u32) << 7)) as u64, 1);
                1
            }
            0..=MAX_PN_2 => {
                dst.put_uint_be((self.0 | ((PN_2_FLAG as u32) << 14)) as u64, 2);
                2
            }
            0..=MAX_PN_4 => {
                dst.put_uint_be((self.0 | ((PN_4_FLAG as u32) << 30)) as u64, 4);
                4
            }
            v => panic!(
                "package number {} has overfolwn, maximum is {}",
                v, MAX_PN_4
            ),
        })
    }
}

impl Decoder for PackageNumber {
    fn decode<T: Buf>(&mut self, src: &mut T) -> Result<usize> {
        let first = src.get_u8();
        let (v, n) = match first >> 7 {
            PN_1_FLAG => ((first as u32) & MAX_PN_1, 1),
            _ => match first >> 6 {
                PN_2_FLAG => (((first as u32) << 8 | (src.get_u8() as u32)) & MAX_PN_2, 2),
                PN_4_FLAG => (
                    ((first as u32) << 24 | (src.get_uint_be(3) as u32)) & MAX_PN_4,
                    4,
                ),
                _ => unreachable!(),
            },
        };
        self.0 = v;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_pn(input: Vec<u8>) -> (PackageNumber, usize) {
        let mut v = PackageNumber(0);
        let n = v.decode(&mut input.into_buf()).unwrap();
        (v, n)
    }

    #[test]
    fn decode_pn1_test() {
        assert_eq!(decode_pn(vec![0b00000011]), (PackageNumber(3), 1));
    }

    #[test]
    fn decode_pn2_test() {
        assert_eq!(
            decode_pn(vec![0b10000001, 0b00000001]),
            (PackageNumber(257), 2)
        );
    }

    #[test]
    fn decode_pn4_test() {
        assert_eq!(
            decode_pn(vec![0b11000001, 0b00000001, 0b00000001, 0b0000000]),
            (PackageNumber(16843008), 4)
        );
    }

    fn encode_pn(input: PackageNumber) -> Vec<u8> {
        let mut dst = vec![];
        input.encode(&mut dst).unwrap();
        dst
    }

    #[test]
    fn encode_pn1_test() {
        assert_eq!(encode_pn(PackageNumber(3)), vec![0b00000011]);
    }

    #[test]
    fn encode_pn2_test() {
        assert_eq!(encode_pn(PackageNumber(257)), vec![0b10000001, 0b00000001]);
    }

    #[test]
    fn encode_pn4_test() {
        assert_eq!(
            encode_pn(PackageNumber(16843009)),
            vec![0b11000001, 0b00000001, 0b00000001, 0b0000001],
        );
    }

    #[test]
    fn builder_test() {
        let b = Generater::new();
        assert_eq!(b.next(), Some(PackageNumber(0)));
        assert_eq!(b.next(), Some(PackageNumber(1)));
        assert_eq!(b.next(), Some(PackageNumber(2)));
    }
}
