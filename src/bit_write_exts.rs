use crate::{bit_write::BitWrite, byte_order::ByteOrder, prelude::*};
use nsw_types::*;

#[allow(non_snake_case)]
fn write_uN<B: ByteOrder, const N: usize, W: BitWrite + ?Sized>(
    writer: &mut W,
    value: u32,
) -> std::io::Result<()> {
    let mut tmp = BitVec::repeat(false, N);
    B::write_u32_to_bits(tmp.as_mut_bitslice(), value);
    writer.write_all_bits(tmp.as_bitslice())
}

/// A trait which extends BitWrite to add explicit write methods for non-standard-width types.
pub trait BitWriteExts: BitWrite {
    fn write_bool(&mut self, value: bool) -> std::io::Result<()> {
        self.write_u1(value.into())
    }

    fn write_u1(&mut self, value: u1) -> std::io::Result<()> {
        write_uN::<BigEndian, 1, _>(self, value.into())
    }

    fn write_u2(&mut self, value: u2) -> std::io::Result<()> {
        write_uN::<BigEndian, 2, _>(self, value.into())
    }

    fn write_u3(&mut self, value: u3) -> std::io::Result<()> {
        write_uN::<BigEndian, 3, _>(self, value.into())
    }

    fn write_u4(&mut self, value: u4) -> std::io::Result<()> {
        write_uN::<BigEndian, 4, _>(self, value.into())
    }

    fn write_u5(&mut self, value: u5) -> std::io::Result<()> {
        write_uN::<BigEndian, 5, _>(self, value.into())
    }

    fn write_u6(&mut self, value: u6) -> std::io::Result<()> {
        write_uN::<BigEndian, 6, _>(self, value.into())
    }

    fn write_u7(&mut self, value: u7) -> std::io::Result<()> {
        write_uN::<BigEndian, 7, _>(self, value.into())
    }

    fn write_u8(&mut self, value: u8) -> std::io::Result<()> {
        write_uN::<BigEndian, 8, _>(self, value.into())
    }

    fn write_u9<B: ByteOrder>(&mut self, value: u9) -> std::io::Result<()> {
        write_uN::<B, 9, _>(self, value.into())
    }

    fn write_u10<B: ByteOrder>(&mut self, value: u10) -> std::io::Result<()> {
        write_uN::<B, 10, _>(self, value.into())
    }

    fn write_u11<B: ByteOrder>(&mut self, value: u11) -> std::io::Result<()> {
        write_uN::<B, 11, _>(self, value.into())
    }

    fn write_u12<B: ByteOrder>(&mut self, value: u12) -> std::io::Result<()> {
        write_uN::<B, 12, _>(self, value.into())
    }

    fn write_u13<B: ByteOrder>(&mut self, value: u13) -> std::io::Result<()> {
        write_uN::<B, 13, _>(self, value.into())
    }

    fn write_u14<B: ByteOrder>(&mut self, value: u14) -> std::io::Result<()> {
        write_uN::<B, 14, _>(self, value.into())
    }

    fn write_u15<B: ByteOrder>(&mut self, value: u15) -> std::io::Result<()> {
        write_uN::<B, 15, _>(self, value.into())
    }

    fn write_u16<B: ByteOrder>(&mut self, value: u16) -> std::io::Result<()> {
        write_uN::<B, 16, _>(self, value.into())
    }

    fn write_u17<B: ByteOrder>(&mut self, value: u17) -> std::io::Result<()> {
        write_uN::<B, 17, _>(self, value.into())
    }

    fn write_u18<B: ByteOrder>(&mut self, value: u18) -> std::io::Result<()> {
        write_uN::<B, 18, _>(self, value.into())
    }

    fn write_u19<B: ByteOrder>(&mut self, value: u19) -> std::io::Result<()> {
        write_uN::<B, 19, _>(self, value.into())
    }

    fn write_u20<B: ByteOrder>(&mut self, value: u20) -> std::io::Result<()> {
        write_uN::<B, 20, _>(self, value.into())
    }

    fn write_u21<B: ByteOrder>(&mut self, value: u21) -> std::io::Result<()> {
        write_uN::<B, 21, _>(self, value.into())
    }

    fn write_u22<B: ByteOrder>(&mut self, value: u22) -> std::io::Result<()> {
        write_uN::<B, 22, _>(self, value.into())
    }

    fn write_u23<B: ByteOrder>(&mut self, value: u23) -> std::io::Result<()> {
        write_uN::<B, 23, _>(self, value.into())
    }

    fn write_u24<B: ByteOrder>(&mut self, value: u24) -> std::io::Result<()> {
        write_uN::<B, 24, _>(self, value.into())
    }

    fn write_u25<B: ByteOrder>(&mut self, value: u25) -> std::io::Result<()> {
        write_uN::<B, 25, _>(self, value.into())
    }

    fn write_u26<B: ByteOrder>(&mut self, value: u26) -> std::io::Result<()> {
        write_uN::<B, 26, _>(self, value.into())
    }

    fn write_u27<B: ByteOrder>(&mut self, value: u27) -> std::io::Result<()> {
        write_uN::<B, 27, _>(self, value.into())
    }

    fn write_u28<B: ByteOrder>(&mut self, value: u28) -> std::io::Result<()> {
        write_uN::<B, 28, _>(self, value.into())
    }

    fn write_u29<B: ByteOrder>(&mut self, value: u29) -> std::io::Result<()> {
        write_uN::<B, 29, _>(self, value.into())
    }

    fn write_u30<B: ByteOrder>(&mut self, value: u30) -> std::io::Result<()> {
        write_uN::<B, 30, _>(self, value.into())
    }

    fn write_u31<B: ByteOrder>(&mut self, value: u31) -> std::io::Result<()> {
        write_uN::<B, 31, _>(self, value.into())
    }

    fn write_u32<B: ByteOrder>(&mut self, value: u32) -> std::io::Result<()> {
        write_uN::<B, 32, _>(self, value)
    }
}

impl<T> BitWriteExts for T where T: BitWrite + ?Sized {}
