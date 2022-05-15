use crate::bitcursor::{BitCursor, BitCursorResult};
use byteorder::{ReadBytesExt, NetworkEndian};
use ux::*;


/// A trait similar to std::io::Read, for reading bit-sized amounts from
/// a BitCursor.
pub trait BitRead: Sized {
    fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self>;
}

impl BitRead for bool {
    fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
        Ok(cursor.read_bit()? == 1)
    }
}

macro_rules! impl_bit_read {
    ($num_bits:expr,$type:ty,u8) => {
        impl BitRead for $type {
            fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
                Ok(<$type>::new(cursor.read_bits_as_u8($num_bits)?))
            }
        }
    };
    ($num_bits:expr,$type:ty,u16) => {
        impl BitRead for $type {
            fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
                Ok(<$type>::new(cursor.read_bits_as_u16($num_bits)?))
            }
        }
    };
    ($num_bits:expr,$type:ty,u32) => {
        impl BitRead for $type {
            fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
                Ok(<$type>::new(cursor.read_bits_as_u32($num_bits)?))
            }
        }
    };
}

impl_bit_read!(2, u2, u8);
impl_bit_read!(3, u3, u8);
impl_bit_read!(4, u4, u8);
impl_bit_read!(5, u5, u8);
impl_bit_read!(6, u6, u8);
impl_bit_read!(7, u7, u8);

impl_bit_read!(9, u9, u16);
impl_bit_read!(10, u10, u16);
impl_bit_read!(11, u11, u16);
impl_bit_read!(12, u12, u16);
impl_bit_read!(13, u13, u16);
impl_bit_read!(14, u14, u16);
impl_bit_read!(15, u15, u16);

impl_bit_read!(17, u17, u32);
impl_bit_read!(18, u18, u32);
impl_bit_read!(19, u19, u32);
impl_bit_read!(20, u20, u32);
impl_bit_read!(21, u21, u32);
impl_bit_read!(22, u22, u32);
impl_bit_read!(23, u23, u32);
impl_bit_read!(24, u24, u32);
impl_bit_read!(25, u25, u32);
impl_bit_read!(26, u26, u32);
impl_bit_read!(27, u27, u32);
impl_bit_read!(28, u28, u32);
impl_bit_read!(29, u29, u32);
impl_bit_read!(30, u30, u32);
impl_bit_read!(31, u31, u32);

impl BitRead for u8 {
    fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
        ReadBytesExt::read_u8(cursor).map_err(std::io::Error::into)
    }
}

// TODO: good way not to assume NetworkEndian for these?

impl BitRead for u16 {
    fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
        ReadBytesExt::read_u16::<NetworkEndian>(cursor).map_err(std::io::Error::into)
    }
}

impl BitRead for u32 {
    fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
        ReadBytesExt::read_u32::<NetworkEndian>(cursor).map_err(std::io::Error::into)
    }
}

impl BitRead for u128 {
    fn bit_read(cursor: &mut BitCursor) -> BitCursorResult<Self> {
        ReadBytesExt::read_u128::<NetworkEndian>(cursor).map_err(std::io::Error::into)
    }
}
