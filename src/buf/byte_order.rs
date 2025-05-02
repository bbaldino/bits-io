use bitvec::field::BitField;
use funty::Integral;

use crate::prelude::*;

/// This trait defines operations to load and store integral values from a buffer, and enables
/// implementing them in different ways for the different byte orders (Big Endian and Little
/// Endian).
pub trait ByteOrder {
    fn load<O: BitStore, U: Integral>(src: &BitSlice<O>) -> U;
    fn load_u16(src: &[u8]) -> u16;
    fn load_u24(src: &[u8]) -> u24;
    fn load_u32(src: &[u8]) -> u32;

    fn store<O: BitStore, U: Integral>(dest: &mut BitSlice<O>, value: U);
    fn store_u16(dest: &mut [u8], value: u16);
    fn store_u24(dest: &mut [u8], value: u24);
    fn store_u32(dest: &mut [u8], value: u32);
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

impl ByteOrder for BigEndian {
    fn load<O: BitStore, U: Integral>(src: &BitSlice<O>) -> U {
        src.load_be()
    }

    fn load_u16(src: &[u8]) -> u16 {
        u16::from_be_bytes(src.try_into().unwrap())
    }

    fn load_u24(src: &[u8]) -> u24 {
        let mut value = 0u32;
        value |= src[0] as u32;
        value <<= 8;
        value |= src[1] as u32;
        value <<= 8;
        value |= src[2] as u32;

        u24::new(value)
    }

    fn load_u32(src: &[u8]) -> u32 {
        u32::from_be_bytes(src.try_into().unwrap())
    }

    fn store<O: BitStore, U: Integral>(dest: &mut BitSlice<O>, value: U) {
        dest.store_be(value);
    }

    fn store_u16(dest: &mut [u8], value: u16) {
        dest[0] = (value >> 8) as u8;
        dest[1] = value as u8;
    }

    fn store_u24(dest: &mut [u8], value: u24) {
        let value: u32 = value.into();
        dest[0] = (value >> 16) as u8;
        dest[1] = (value >> 8) as u8;
        dest[2] = value as u8;
    }

    fn store_u32(dest: &mut [u8], value: u32) {
        dest[0] = (value >> 24) as u8;
        dest[1] = (value >> 16) as u8;
        dest[2] = (value >> 8) as u8;
        dest[3] = value as u8;
    }
}

impl ByteOrder for LittleEndian {
    fn load<O: BitStore, U: Integral>(src: &BitSlice<O>) -> U {
        src.load_le()
    }

    fn load_u16(src: &[u8]) -> u16 {
        u16::from_le_bytes(src.try_into().unwrap())
    }

    fn load_u24(src: &[u8]) -> u24 {
        let mut value = 0u32;
        value |= src[2] as u32;
        value <<= 8;
        value |= src[1] as u32;
        value <<= 8;
        value |= src[0] as u32;
        u24::new(value)
    }

    fn load_u32(src: &[u8]) -> u32 {
        u32::from_le_bytes(src.try_into().unwrap())
    }

    fn store<O: BitStore, U: Integral>(dest: &mut BitSlice<O>, value: U) {
        dest.store_le(value)
    }

    fn store_u16(dest: &mut [u8], value: u16) {
        dest[0] = value as u8;
        dest[1] = (value >> 8) as u8;
    }

    fn store_u24(dest: &mut [u8], value: u24) {
        let value: u32 = value.into();
        dest[0] = value as u8;
        dest[1] = (value >> 8) as u8;
        dest[2] = (value >> 16) as u8;
    }

    fn store_u32(dest: &mut [u8], value: u32) {
        dest[0] = value as u8;
        dest[1] = (value >> 8) as u8;
        dest[2] = (value >> 16) as u8;
        dest[3] = (value >> 24) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: more test cases here (different data sizes)

    #[test]
    fn test_big_endian_write() {
        let mut buf = [0u8; 2];
        {
            // Simulate writing 4 bits
            let dest = &mut BitSlice::from_slice_mut(&mut buf);
            BigEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_be_bytes(buf);
            assert_eq!(value, 0xABCD);
        }

        // Now test that writing into a BitSlice<BitSafeU8> also works.
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
            BigEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_be_bytes(buf);
            assert_eq!(value, 0xABCD);
        }
    }

    #[test]
    fn test_big_endian_read() {
        let value = 0xABCDu16;
        let value_bytes = value.to_be_bytes();
        let src = BitSlice::from_slice(&value_bytes);
        let read_value: u16 = BigEndian::load(src);
        assert_eq!(value, read_value);
    }

    #[test]
    fn test_little_endian_write() {
        let mut buf = [0u8; 2];
        {
            // Simulate writing 4 bits
            let dest = &mut BitSlice::from_slice_mut(&mut buf);
            LittleEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_le_bytes(buf);
            assert_eq!(value, 0xABCD);
        }

        // Now test that writing into a BitSlice<BitSafeU8> also works.
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
            LittleEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_le_bytes(buf);
            assert_eq!(value, 0xABCD);
        }
    }

    #[test]
    fn test_little_endian_read() {
        let value = 0xABCDu16;
        let value_bytes = value.to_le_bytes();
        let src = BitSlice::from_slice(&value_bytes);
        let read_value: u16 = LittleEndian::load(src);
        assert_eq!(value, read_value);
    }
}
