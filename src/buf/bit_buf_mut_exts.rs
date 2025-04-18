use bitvec::order::Msb0;
use bitvec::view::BitView;

use crate::prelude::*;

pub trait BitBufMutExts: BitBufMut {
    fn put_fixed<const N: usize, U: Into<u8>>(&mut self, value: U) -> std::io::Result<()> {
        let bits = self.chunk_mut();
        if bits.len() < N {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        let mut val = value.into();
        for i in (0..N).rev() {
            bits.set(i, (val & 1) != 0);
            val >>= 1;
        }
        self.advance_mut(N);
        Ok(())
    }

    // TODO: BytesMut converts everything to a slice and then just does put_slice, so it'd be nice
    // to do the same.  The issue is that the uX types don't convert cleanly into the BitSlice<u8,
    // Msb0> semantics: a u1 is in a u8, but the subset of those bits are in Msb0 order but shifted
    // to the "right" side of the value's storage.  It doesn't work to just treat it as Lsb0 order,
    // because then the relevant bits would be reversed: what we need to do is shift the bits to
    // the left.  Or, we could slice off the 'end' of the value where the bits are --> this works
    //
    // But do we still have an issue for values larger than a byte?  does the above process work
    // with endian-ness?  Can we convert the inner-storage value to le/be bytes and then bitslice
    // it?
    //
    //

    fn put_fixed2(&mut self, slice: &BitSlice) -> std::io::Result<()> {
        self.put_slice(slice);
        Ok(())
    }

    fn put_bool(&mut self, value: bool) -> std::io::Result<()> {
        self.put_u1(u1::new(value as u8))
    }

    fn put_u1(&mut self, value: u1) -> std::io::Result<()> {
        // TODO: this doesn't work, as this will left-pad a single bit value into 8 bits so it'll
        // get written as 0b00000001 instead of 0b1
        let value: u8 = value.into();
        let bits = &value.view_bits()[7..];
        self.put_fixed2(bits)
    }

    fn put_u2(&mut self, value: u2) -> std::io::Result<()> {
        self.put_fixed::<2, _>(value)
    }

    fn put_u3(&mut self, value: u3) -> std::io::Result<()> {
        self.put_fixed::<3, _>(value)
    }

    fn put_u4(&mut self, value: u4) -> std::io::Result<()> {
        self.put_fixed::<4, _>(value)
    }

    fn put_u5(&mut self, value: u5) -> std::io::Result<()> {
        self.put_fixed::<5, _>(value)
    }

    fn put_u6(&mut self, value: u6) -> std::io::Result<()> {
        self.put_fixed::<6, _>(value)
    }

    fn put_u7(&mut self, value: u7) -> std::io::Result<()> {
        self.put_fixed::<7, _>(value)
    }

    fn put_u8(&mut self, value: u8) -> std::io::Result<()> {
        self.put_fixed::<8, _>(value)
    }

    #[allow(non_snake_case)]
    fn put_uN<O: ByteOrder, const N: usize, U>(&mut self, value: U) -> std::io::Result<()>
    where
        U: Into<u32>,
    {
        let bits = self.chunk_mut();
        if bits.len() < N {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        O::write_u32_to_bits(&mut bits[..N], value.into());
        self.advance_mut(N);
        Ok(())
    }

    fn put_u9<O: ByteOrder>(&mut self, value: u9) -> std::io::Result<()> {
        self.put_uN::<O, 9, _>(value)
    }
    fn put_u10<O: ByteOrder>(&mut self, value: u10) -> std::io::Result<()> {
        self.put_uN::<O, 10, _>(value)
    }
    fn put_u11<O: ByteOrder>(&mut self, value: u11) -> std::io::Result<()> {
        self.put_uN::<O, 11, _>(value)
    }
    fn put_u12<O: ByteOrder>(&mut self, value: u12) -> std::io::Result<()> {
        self.put_uN::<O, 12, _>(value)
    }
    fn put_u13<O: ByteOrder>(&mut self, value: u13) -> std::io::Result<()> {
        self.put_uN::<O, 13, _>(value)
    }
    fn put_u14<O: ByteOrder>(&mut self, value: u14) -> std::io::Result<()> {
        self.put_uN::<O, 14, _>(value)
    }
    fn put_u15<O: ByteOrder>(&mut self, value: u15) -> std::io::Result<()> {
        self.put_uN::<O, 15, _>(value)
    }
    fn put_u16<O: ByteOrder>(&mut self, value: u16) -> std::io::Result<()> {
        self.put_uN::<O, 16, _>(value)
    }
    fn put_u17<O: ByteOrder>(&mut self, value: u17) -> std::io::Result<()> {
        self.put_uN::<O, 17, _>(value)
    }
    fn put_u18<O: ByteOrder>(&mut self, value: u18) -> std::io::Result<()> {
        self.put_uN::<O, 18, _>(value)
    }
    fn put_u19<O: ByteOrder>(&mut self, value: u19) -> std::io::Result<()> {
        self.put_uN::<O, 19, _>(value)
    }
    fn put_u20<O: ByteOrder>(&mut self, value: u20) -> std::io::Result<()> {
        self.put_uN::<O, 20, _>(value)
    }
    fn put_u21<O: ByteOrder>(&mut self, value: u21) -> std::io::Result<()> {
        self.put_uN::<O, 21, _>(value)
    }
    fn put_u22<O: ByteOrder>(&mut self, value: u22) -> std::io::Result<()> {
        self.put_uN::<O, 22, _>(value)
    }
    fn put_u23<O: ByteOrder>(&mut self, value: u23) -> std::io::Result<()> {
        self.put_uN::<O, 23, _>(value)
    }
    fn put_u24<O: ByteOrder>(&mut self, value: u24) -> std::io::Result<()> {
        self.put_uN::<O, 24, _>(value)
    }
    fn put_u25<O: ByteOrder>(&mut self, value: u25) -> std::io::Result<()> {
        self.put_uN::<O, 25, _>(value)
    }
    fn put_u26<O: ByteOrder>(&mut self, value: u26) -> std::io::Result<()> {
        self.put_uN::<O, 26, _>(value)
    }
    fn put_u27<O: ByteOrder>(&mut self, value: u27) -> std::io::Result<()> {
        self.put_uN::<O, 27, _>(value)
    }
    fn put_u28<O: ByteOrder>(&mut self, value: u28) -> std::io::Result<()> {
        self.put_uN::<O, 28, _>(value)
    }
    fn put_u29<O: ByteOrder>(&mut self, value: u29) -> std::io::Result<()> {
        self.put_uN::<O, 29, _>(value)
    }
    fn put_u30<O: ByteOrder>(&mut self, value: u30) -> std::io::Result<()> {
        self.put_uN::<O, 30, _>(value)
    }
    fn put_u31<O: ByteOrder>(&mut self, value: u31) -> std::io::Result<()> {
        self.put_uN::<O, 31, _>(value)
    }
    fn put_u32<O: ByteOrder>(&mut self, value: u32) -> std::io::Result<()> {
        self.put_uN::<O, 32, _>(value)
    }
}

impl<T: BitBufMut + ?Sized> BitBufMutExts for T {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // TODO: this test is crashing!
    #[test]
    fn test_put_and_get_u1() {
        let mut bits_mut = BitsMut::new();
        bits_mut.put_u1(u1::new(1)).unwrap();
        println!("slice after writing one bit: {:?}", &bits_mut[..]);
        let mut bits = Bits::from(&bits_mut[..]);
        let val = bits.get_u1().unwrap();
        assert_eq!(val, u1::new(1));
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
