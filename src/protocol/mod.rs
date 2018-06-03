mod cid;
mod codec;
mod header;
mod pn;
mod varint;

pub use self::cid::ConnectionID;

use self::codec::{Decoder, Encoder};
use self::pn::{Generater, PackageNumber};
use self::varint::VarInt;

pub const VERSION: u32 = 0xff00000C;
