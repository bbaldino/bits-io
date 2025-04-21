use funty::Integral;

use crate::prelude::*;

pub trait BitBufMutExts: BitBufMut {
    #[allow(non_snake_case)]
    fn put_uN<O: ByteOrder, const N: usize, U, V: Integral>(
        &mut self,
        value: U,
    ) -> std::io::Result<()>
    where
        U: Into<V>,
        // U::Error: std::fmt::Debug,
    {
        let mut bits = BitVec::repeat(false, N);
        let value_slice = bits.as_mut_bitslice();
        // Convert the given value into the given integral type we're told it should map to (V).
        // E.g. u8, u16, u32.
        let value_integral: V = value.into();
        println!("put_uN putting {N} bits.  value slice: {value_slice:?}, value integral: {value_integral}");
        O::store(value_slice, value_integral);
        println!("put_uN byte-order included slice: {value_slice:?}");
        self.put_slice(value_slice);
        Ok(())
    }

    fn put_bool(&mut self, value: bool) -> std::io::Result<()> {
        self.put_u1(u1::new(value as u8))
    }

    fn put_u1(&mut self, value: u1) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 1, u1, u8>(value)
    }

    fn put_u2(&mut self, value: u2) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 2, u2, u8>(value)
    }

    fn put_u3(&mut self, value: u3) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 3, u3, u8>(value)
    }

    fn put_u4(&mut self, value: u4) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 4, u4, u8>(value)
    }

    fn put_u5(&mut self, value: u5) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 5, u5, u8>(value)
    }

    fn put_u6(&mut self, value: u6) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 6, u6, u8>(value)
    }

    fn put_u7(&mut self, value: u7) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 7, u7, u8>(value)
    }

    fn put_u8(&mut self, value: u8) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 8, u8, u8>(value)
    }

    fn put_u9<O: ByteOrder>(&mut self, value: u9) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 9, u9, u16>(value)
    }
    fn put_u10<O: ByteOrder>(&mut self, value: u10) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 10, u10, u16>(value)
    }
    fn put_u11<O: ByteOrder>(&mut self, value: u11) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 11, u11, u16>(value)
    }
    fn put_u12<O: ByteOrder>(&mut self, value: u12) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 12, u12, u16>(value)
    }
    fn put_u13<O: ByteOrder>(&mut self, value: u13) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 13, u13, u16>(value)
    }
    fn put_u14<O: ByteOrder>(&mut self, value: u14) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 14, u14, u16>(value)
    }
    fn put_u15<O: ByteOrder>(&mut self, value: u15) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 15, u15, u16>(value)
    }
    fn put_u16<O: ByteOrder>(&mut self, value: u16) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 16, u16, u16>(value)
    }
    fn put_u17<O: ByteOrder>(&mut self, value: u17) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 17, u17, u32>(value)
    }
    fn put_u18<O: ByteOrder>(&mut self, value: u18) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 18, u18, u32>(value)
    }
    fn put_u19<O: ByteOrder>(&mut self, value: u19) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 19, u19, u32>(value)
    }
    fn put_u20<O: ByteOrder>(&mut self, value: u20) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 20, u20, u32>(value)
    }
    fn put_u21<O: ByteOrder>(&mut self, value: u21) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 21, u21, u32>(value)
    }
    fn put_u22<O: ByteOrder>(&mut self, value: u22) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 22, u22, u32>(value)
    }
    fn put_u23<O: ByteOrder>(&mut self, value: u23) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 23, u23, u32>(value)
    }
    fn put_u24<O: ByteOrder>(&mut self, value: u24) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 24, u24, u32>(value)
    }
    fn put_u25<O: ByteOrder>(&mut self, value: u25) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 25, u25, u32>(value)
    }
    fn put_u26<O: ByteOrder>(&mut self, value: u26) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 26, u26, u32>(value)
    }
    fn put_u27<O: ByteOrder>(&mut self, value: u27) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 27, u27, u32>(value)
    }
    fn put_u28<O: ByteOrder>(&mut self, value: u28) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 28, u28, u32>(value)
    }
    fn put_u29<O: ByteOrder>(&mut self, value: u29) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 29, u29, u32>(value)
    }
    fn put_u30<O: ByteOrder>(&mut self, value: u30) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 30, u30, u32>(value)
    }
    fn put_u31<O: ByteOrder>(&mut self, value: u31) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 31, u31, u32>(value)
    }
    fn put_u32<O: ByteOrder>(&mut self, value: u32) -> std::io::Result<()> {
        self.put_uN::<BigEndian, 32, u32, u32>(value)
    }
}

impl<T: BitBufMut + ?Sized> BitBufMutExts for T {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // TODO: this test is crashing!
    #[test]
    fn test_put() {
        let mut bits_mut = BitsMut::new();
        bits_mut.put_u1(u1::new(0b1)).unwrap();
        println!("buf after put u1: {bits_mut:?}");
        bits_mut.put_u3(u3::new(0b001)).unwrap();
        println!("buf after put u3: {bits_mut:?}");
        // bits_mut.put_u5(u5::new(0b00001)).unwrap();
        // bits_mut.put_u7(u7::new(0b0000001)).unwrap();
        //
        // assert_eq!(
        //     &bits_mut[..],
        //     bits![1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1]
        // );
    }

    // #[test]
    // fn test_put_and_get_u4() {
    //     let mut buf = BitsMut::from(bitvec![0; 4]);
    //     buf.put_u4(u4::new(0b1010)).unwrap();
    //     buf.advance(-4);
    //     let val = buf.get_u4().unwrap();
    //     assert_eq!(val, u4::new(0b1010));
    // }
    //
    // #[test]
    // fn test_put_and_get_u8() {
    //     let mut buf = BitsMut::from(bitvec![0; 8]);
    //     buf.put_u8(0b11110000).unwrap();
    //     buf.advance(-8);
    //     let val = buf.get_u8().unwrap();
    //     assert_eq!(val, 0b11110000);
    // }
    //
    // #[test]
    // fn test_put_and_get_u12_le() {
    //     let mut buf = BitsMut::from(bitvec![0; 12]);
    //     buf.put_u12::<LittleEndian>(u12::new(0b1010_1100_0011))
    //         .unwrap();
    //     buf.advance(-12);
    //     let val = buf.get_u12::<LittleEndian>().unwrap();
    //     assert_eq!(val, u12::new(0b1010_1100_0011));
    // }
    //
    // #[test]
    // fn test_put_and_get_u20_be() {
    //     let mut buf = BitsMut::from(bitvec![0; 20]);
    //     buf.put_u20::<BigEndian>(u20::new(0xABCDE)).unwrap();
    //     buf.advance(-20);
    //     let val = buf.get_u20::<BigEndian>().unwrap();
    //     assert_eq!(val, u20::new(0xABCDE));
    // }
}
