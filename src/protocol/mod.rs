mod cid;
mod codec;
mod pn;
mod varint;

use self::codec::Decoder;
use self::codec::Encoder;

pub const VERSION: u32 = 0xff00000C;
