use crate::byte_order::{BigEndian, ByteOrder};
use crate::prelude::*;
use nsw_types::*;

#[allow(non_snake_case)]
fn read_uN<R: BitRead + ?Sized, B: ByteOrder>(reader: &mut R, n: usize) -> std::io::Result<u32> {
    let mut tmp = BitVec::repeat(false, n);
    reader.read_bits_exact(tmp.as_mut_bitslice())?;
    Ok(B::read_u32_from_bits(tmp.as_bitslice()))
}

/// A trait which extends BitRead to add explicit read methods for non-standard-width types
pub trait BitReadExts: BitRead {
    fn read_bool(&mut self) -> std::io::Result<bool> {
        Ok(self.read_u1()? != 0)
    }

    fn read_u1(&mut self) -> std::io::Result<u1> {
        read_uN::<_, BigEndian>(self, 1).map(|v| u1::new(v as u8))
    }

    fn read_u2(&mut self) -> std::io::Result<u2> {
        read_uN::<_, BigEndian>(self, 2).map(|v| u2::new(v as u8))
    }

    fn read_u3(&mut self) -> std::io::Result<u3> {
        read_uN::<_, BigEndian>(self, 3).map(|v| u3::new(v as u8))
    }

    fn read_u4(&mut self) -> std::io::Result<u4> {
        read_uN::<_, BigEndian>(self, 4).map(|v| u4::new(v as u8))
    }

    fn read_u5(&mut self) -> std::io::Result<u5> {
        read_uN::<_, BigEndian>(self, 5).map(|v| u5::new(v as u8))
    }

    fn read_u6(&mut self) -> std::io::Result<u6> {
        read_uN::<_, BigEndian>(self, 6).map(|v| u6::new(v as u8))
    }

    fn read_u7(&mut self) -> std::io::Result<u7> {
        read_uN::<_, BigEndian>(self, 7).map(|v| u7::new(v as u8))
    }

    fn read_u8(&mut self) -> std::io::Result<u8> {
        read_uN::<_, BigEndian>(self, 8).map(|v| v as u8)
    }

    fn read_u9<B: ByteOrder>(&mut self) -> std::io::Result<u9> {
        read_uN::<_, BigEndian>(self, 9).map(|v| u9::new(v as u16))
    }

    fn read_u10<B: ByteOrder>(&mut self) -> std::io::Result<u10> {
        read_uN::<_, BigEndian>(self, 10).map(|v| u10::new(v as u16))
    }

    fn read_u11<B: ByteOrder>(&mut self) -> std::io::Result<u11> {
        read_uN::<_, BigEndian>(self, 11).map(|v| u11::new(v as u16))
    }

    fn read_u12<B: ByteOrder>(&mut self) -> std::io::Result<u12> {
        read_uN::<_, BigEndian>(self, 12).map(|v| u12::new(v as u16))
    }

    fn read_u13<B: ByteOrder>(&mut self) -> std::io::Result<u13> {
        read_uN::<_, BigEndian>(self, 13).map(|v| u13::new(v as u16))
    }

    fn read_u14<B: ByteOrder>(&mut self) -> std::io::Result<u14> {
        read_uN::<_, BigEndian>(self, 14).map(|v| u14::new(v as u16))
    }

    fn read_u15<B: ByteOrder>(&mut self) -> std::io::Result<u15> {
        read_uN::<_, BigEndian>(self, 15).map(|v| u15::new(v as u16))
    }

    fn read_u16<B: ByteOrder>(&mut self) -> std::io::Result<u16> {
        read_uN::<_, BigEndian>(self, 16).map(|v| v as u16)
    }

    fn read_u17<B: ByteOrder>(&mut self) -> std::io::Result<u17> {
        read_uN::<_, BigEndian>(self, 17).map(u17::new)
    }

    fn read_u18<B: ByteOrder>(&mut self) -> std::io::Result<u18> {
        read_uN::<_, BigEndian>(self, 18).map(u18::new)
    }

    fn read_u19<B: ByteOrder>(&mut self) -> std::io::Result<u19> {
        read_uN::<_, BigEndian>(self, 19).map(u19::new)
    }

    fn read_u20<B: ByteOrder>(&mut self) -> std::io::Result<u20> {
        read_uN::<_, BigEndian>(self, 20).map(u20::new)
    }

    fn read_u21<B: ByteOrder>(&mut self) -> std::io::Result<u21> {
        read_uN::<_, BigEndian>(self, 21).map(u21::new)
    }

    fn read_u22<B: ByteOrder>(&mut self) -> std::io::Result<u22> {
        read_uN::<_, BigEndian>(self, 22).map(u22::new)
    }

    fn read_u23<B: ByteOrder>(&mut self) -> std::io::Result<u23> {
        read_uN::<_, BigEndian>(self, 23).map(u23::new)
    }

    fn read_u24<B: ByteOrder>(&mut self) -> std::io::Result<u24> {
        read_uN::<_, BigEndian>(self, 24).map(u24::new)
    }

    fn read_u25<B: ByteOrder>(&mut self) -> std::io::Result<u25> {
        read_uN::<_, BigEndian>(self, 25).map(u25::new)
    }

    fn read_u26<B: ByteOrder>(&mut self) -> std::io::Result<u26> {
        read_uN::<_, BigEndian>(self, 26).map(u26::new)
    }

    fn read_u27<B: ByteOrder>(&mut self) -> std::io::Result<u27> {
        read_uN::<_, BigEndian>(self, 27).map(u27::new)
    }

    fn read_u28<B: ByteOrder>(&mut self) -> std::io::Result<u28> {
        read_uN::<_, BigEndian>(self, 28).map(u28::new)
    }

    fn read_u29<B: ByteOrder>(&mut self) -> std::io::Result<u29> {
        read_uN::<_, BigEndian>(self, 29).map(u29::new)
    }

    fn read_u30<B: ByteOrder>(&mut self) -> std::io::Result<u30> {
        read_uN::<_, BigEndian>(self, 30).map(u30::new)
    }

    fn read_u31<B: ByteOrder>(&mut self) -> std::io::Result<u31> {
        read_uN::<_, BigEndian>(self, 31).map(u31::new)
    }

    fn read_u32<B: ByteOrder>(&mut self) -> std::io::Result<u32> {
        read_uN::<_, BigEndian>(self, 32)
    }
}

impl<T: BitRead + ?Sized> BitReadExts for T {}
