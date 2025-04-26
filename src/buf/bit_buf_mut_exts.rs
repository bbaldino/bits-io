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
    {
        let mut bits = BitVec::repeat(false, N);
        let value_slice = bits.as_mut_bitslice();
        // Convert the given value into the given integral type we're told it should map to (V).
        // E.g. u8, u16, u32.
        let value_integral: V = value.into();
        O::store(value_slice, value_integral);
        self.try_put_bit_slice(value_slice)?;
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
        self.put_uN::<O, 9, u9, u16>(value)
    }
    fn put_u10<O: ByteOrder>(&mut self, value: u10) -> std::io::Result<()> {
        self.put_uN::<O, 10, u10, u16>(value)
    }
    fn put_u11<O: ByteOrder>(&mut self, value: u11) -> std::io::Result<()> {
        self.put_uN::<O, 11, u11, u16>(value)
    }
    fn put_u12<O: ByteOrder>(&mut self, value: u12) -> std::io::Result<()> {
        self.put_uN::<O, 12, u12, u16>(value)
    }
    fn put_u13<O: ByteOrder>(&mut self, value: u13) -> std::io::Result<()> {
        self.put_uN::<O, 13, u13, u16>(value)
    }
    fn put_u14<O: ByteOrder>(&mut self, value: u14) -> std::io::Result<()> {
        self.put_uN::<O, 14, u14, u16>(value)
    }
    fn put_u15<O: ByteOrder>(&mut self, value: u15) -> std::io::Result<()> {
        self.put_uN::<O, 15, u15, u16>(value)
    }
    fn put_u16<O: ByteOrder>(&mut self, value: u16) -> std::io::Result<()> {
        self.put_uN::<O, 16, u16, u16>(value)
    }
    fn put_u17<O: ByteOrder>(&mut self, value: u17) -> std::io::Result<()> {
        self.put_uN::<O, 17, u17, u32>(value)
    }
    fn put_u18<O: ByteOrder>(&mut self, value: u18) -> std::io::Result<()> {
        self.put_uN::<O, 18, u18, u32>(value)
    }
    fn put_u19<O: ByteOrder>(&mut self, value: u19) -> std::io::Result<()> {
        self.put_uN::<O, 19, u19, u32>(value)
    }
    fn put_u20<O: ByteOrder>(&mut self, value: u20) -> std::io::Result<()> {
        self.put_uN::<O, 20, u20, u32>(value)
    }
    fn put_u21<O: ByteOrder>(&mut self, value: u21) -> std::io::Result<()> {
        self.put_uN::<O, 21, u21, u32>(value)
    }
    fn put_u22<O: ByteOrder>(&mut self, value: u22) -> std::io::Result<()> {
        self.put_uN::<O, 22, u22, u32>(value)
    }
    fn put_u23<O: ByteOrder>(&mut self, value: u23) -> std::io::Result<()> {
        self.put_uN::<O, 23, u23, u32>(value)
    }
    fn put_u24<O: ByteOrder>(&mut self, value: u24) -> std::io::Result<()> {
        self.put_uN::<O, 24, u24, u32>(value)
    }
    fn put_u25<O: ByteOrder>(&mut self, value: u25) -> std::io::Result<()> {
        self.put_uN::<O, 25, u25, u32>(value)
    }
    fn put_u26<O: ByteOrder>(&mut self, value: u26) -> std::io::Result<()> {
        self.put_uN::<O, 26, u26, u32>(value)
    }
    fn put_u27<O: ByteOrder>(&mut self, value: u27) -> std::io::Result<()> {
        self.put_uN::<O, 27, u27, u32>(value)
    }
    fn put_u28<O: ByteOrder>(&mut self, value: u28) -> std::io::Result<()> {
        self.put_uN::<O, 28, u28, u32>(value)
    }
    fn put_u29<O: ByteOrder>(&mut self, value: u29) -> std::io::Result<()> {
        self.put_uN::<O, 29, u29, u32>(value)
    }
    fn put_u30<O: ByteOrder>(&mut self, value: u30) -> std::io::Result<()> {
        self.put_uN::<O, 30, u30, u32>(value)
    }
    fn put_u31<O: ByteOrder>(&mut self, value: u31) -> std::io::Result<()> {
        self.put_uN::<O, 31, u31, u32>(value)
    }
    fn put_u32<O: ByteOrder>(&mut self, value: u32) -> std::io::Result<()> {
        self.put_uN::<O, 32, u32, u32>(value)
    }
}

impl<T: BitBufMut + ?Sized> BitBufMutExts for T {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_put() {
        let mut bits_mut = BitsMut::new();
        bits_mut.put_u1(u1::new(0b1)).unwrap();
        bits_mut.put_u3(u3::new(0b001)).unwrap();
        bits_mut.put_u5(u5::new(0b00001)).unwrap();
        bits_mut.put_u7(u7::new(0b0000001)).unwrap();

        assert_eq!(
            &bits_mut[..],
            bits![1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_put_big_endian() {
        {
            let mut bits_mut = BitsMut::new();
            let value = u9::new(0b1_01010101);
            bits_mut.put_u9::<BigEndian>(value).unwrap();
            assert_eq!(&bits_mut[..], bits![1, 0, 1, 0, 1, 0, 1, 0, 1]);
        }
        {
            let mut bits_mut = BitsMut::new();
            let value = u21::new(0b10101_01010101_01010101);
            bits_mut.put_u21::<BigEndian>(value).unwrap();
            assert_eq!(
                &bits_mut[..],
                bits![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
            );
        }
    }

    #[test]
    fn test_put_little_endian() {
        {
            let mut bits_mut = BitsMut::new();
            let value = u9::new(0b1_00001111);
            bits_mut.put_u9::<LittleEndian>(value).unwrap();
            assert_eq!(&bits_mut[..], bits![0, 0, 0, 0, 1, 1, 1, 1, 1]);
        }
        {
            let mut bits_mut = BitsMut::new();
            let value = u21::new(0b00110_00001111_00011100);
            bits_mut.put_u21::<LittleEndian>(value).unwrap();
            assert_eq!(
                &bits_mut[..],
                bits![0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0]
            );
        }
    }
}
