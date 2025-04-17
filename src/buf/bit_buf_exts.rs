use super::bit_buf::BitBuf;
use crate::prelude::*;

pub trait BitBufExts: BitBuf {
    fn get_fixed<const N: usize, U: TryFrom<u8>>(&mut self) -> std::io::Result<U>
    where
        U::Error: std::fmt::Debug,
    {
        let bits = self.chunk();
        if bits.len() < N {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        let mut val = 0u8;
        for i in 0..N {
            val <<= 1;
            val |= bits[i] as u8;
        }
        self.advance(N);
        Ok(U::try_from(val).map_err(|_| std::io::ErrorKind::InvalidData)?)
    }

    fn get_bool(&mut self) -> std::io::Result<bool> {
        Ok(self.get_u1()?.into())
    }

    fn get_u1(&mut self) -> std::io::Result<u1> {
        self.get_fixed::<1, _>()
    }

    fn get_u2(&mut self) -> std::io::Result<u2> {
        self.get_fixed::<2, _>()
    }

    fn get_u3(&mut self) -> std::io::Result<u3> {
        self.get_fixed::<3, _>()
    }

    fn get_u4(&mut self) -> std::io::Result<u4> {
        self.get_fixed::<4, _>()
    }

    fn get_u5(&mut self) -> std::io::Result<u5> {
        self.get_fixed::<5, _>()
    }

    fn get_u6(&mut self) -> std::io::Result<u6> {
        self.get_fixed::<6, _>()
    }

    fn get_u7(&mut self) -> std::io::Result<u7> {
        self.get_fixed::<7, _>()
    }

    fn get_u8(&mut self) -> std::io::Result<u8> {
        self.get_fixed::<8, _>()
    }

    #[allow(non_snake_case)]
    fn get_uN<O: ByteOrder, const N: usize, U>(&mut self) -> std::io::Result<U>
    where
        U: TryFrom<u32>,
        U::Error: std::fmt::Debug,
    {
        let bits = self.chunk();
        if bits.len() < N {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }
        let val = O::read_u32_from_bits(&bits[..N]);
        self.advance(N);
        Ok(U::try_from(val).map_err(|_| std::io::ErrorKind::InvalidData)?)
    }

    fn get_u9<O: ByteOrder>(&mut self) -> std::io::Result<u9> {
        self.get_uN::<O, 9, u9>()
    }
    fn get_u10<O: ByteOrder>(&mut self) -> std::io::Result<u10> {
        self.get_uN::<O, 10, u10>()
    }
    fn get_u11<O: ByteOrder>(&mut self) -> std::io::Result<u11> {
        self.get_uN::<O, 11, u11>()
    }
    fn get_u12<O: ByteOrder>(&mut self) -> std::io::Result<u12> {
        self.get_uN::<O, 12, u12>()
    }
    fn get_u13<O: ByteOrder>(&mut self) -> std::io::Result<u13> {
        self.get_uN::<O, 13, u13>()
    }
    fn get_u14<O: ByteOrder>(&mut self) -> std::io::Result<u14> {
        self.get_uN::<O, 14, u14>()
    }
    fn get_u15<O: ByteOrder>(&mut self) -> std::io::Result<u15> {
        self.get_uN::<O, 15, u15>()
    }
    fn get_u16<O: ByteOrder>(&mut self) -> std::io::Result<u16> {
        self.get_uN::<O, 16, u16>()
    }
    fn get_u17<O: ByteOrder>(&mut self) -> std::io::Result<u17> {
        self.get_uN::<O, 17, u17>()
    }
    fn get_u18<O: ByteOrder>(&mut self) -> std::io::Result<u18> {
        self.get_uN::<O, 18, u18>()
    }
    fn get_u19<O: ByteOrder>(&mut self) -> std::io::Result<u19> {
        self.get_uN::<O, 19, u19>()
    }
    fn get_u20<O: ByteOrder>(&mut self) -> std::io::Result<u20> {
        self.get_uN::<O, 20, u20>()
    }
    fn get_u21<O: ByteOrder>(&mut self) -> std::io::Result<u21> {
        self.get_uN::<O, 21, u21>()
    }
    fn get_u22<O: ByteOrder>(&mut self) -> std::io::Result<u22> {
        self.get_uN::<O, 22, u22>()
    }
    fn get_u23<O: ByteOrder>(&mut self) -> std::io::Result<u23> {
        self.get_uN::<O, 23, u23>()
    }
    fn get_u24<O: ByteOrder>(&mut self) -> std::io::Result<u24> {
        self.get_uN::<O, 24, u24>()
    }
    fn get_u25<O: ByteOrder>(&mut self) -> std::io::Result<u25> {
        self.get_uN::<O, 25, u25>()
    }
    fn get_u26<O: ByteOrder>(&mut self) -> std::io::Result<u26> {
        self.get_uN::<O, 26, u26>()
    }
    fn get_u27<O: ByteOrder>(&mut self) -> std::io::Result<u27> {
        self.get_uN::<O, 27, u27>()
    }
    fn get_u28<O: ByteOrder>(&mut self) -> std::io::Result<u28> {
        self.get_uN::<O, 28, u28>()
    }
    fn get_u29<O: ByteOrder>(&mut self) -> std::io::Result<u29> {
        self.get_uN::<O, 29, u29>()
    }
    fn get_u30<O: ByteOrder>(&mut self) -> std::io::Result<u30> {
        self.get_uN::<O, 30, u30>()
    }
    fn get_u31<O: ByteOrder>(&mut self) -> std::io::Result<u31> {
        self.get_uN::<O, 31, u31>()
    }
    fn get_u32<O: ByteOrder>(&mut self) -> std::io::Result<u32> {
        self.get_uN::<O, 32, u32>()
    }
}

impl<T: BitBuf + ?Sized> BitBufExts for T {}

#[cfg(test)]
mod tests {
    use crate::buf::bits::Bits;

    use super::*;

    #[test]
    fn test_bit_buf_exts() {
        let mut bits = Bits::copy_from_slice(bits![0, 1, 1, 0, 0, 1, 1, 1]);

        let value = bits.get_u4().unwrap();
        assert_eq!(value, u4::new(0b0110));
        let value = bits.get_u1().unwrap();
        assert_eq!(value, u1::new(0));
        let value = bits.get_u3().unwrap();
        assert_eq!(value, u3::new(0b111));
    }

    #[test]
    fn test_get_big_endian() {
        let data = bits![
            1, 0, 1, 0, 1, 0, 1, 0, 1, // u9
            1, 1, 1, 1, 0, 0, 0, 0, 1, 1, // u10
            0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, // u11
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, // u12
        ];
        let mut bits = Bits::copy_from_slice(data);

        assert_eq!(bits.get_u9::<BigEndian>().unwrap(), u9::new(0b101010101));
        assert_eq!(bits.get_u10::<BigEndian>().unwrap(), u10::new(0b1111000011));
        assert_eq!(
            bits.get_u11::<BigEndian>().unwrap(),
            u11::new(0b00110011000)
        );
        assert_eq!(
            bits.get_u12::<BigEndian>().unwrap(),
            u12::new(0b011111111000)
        );
    }

    #[test]
    fn test_get_little_endian() {
        #[rustfmt::skip]
        let data = bits![
            1, 0, 1, 0, 1, 0, 1, 0, 1, // u9 = 0b101010101 → LittleEndian: 0b101010101
            1, 1, 1, 1, 0, 0, 0, 0, 1, 1, // u10 = 0b1111000011 → LittleEndian: 0b1100001111 = 0x30F
            0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, // u11 = 0b00110011000 → LittleEndian: 0b00011001100 = 0x0CC
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, // u12 = 0b011111111000 → LittleEndian: 0b000111111110 = 0x1FE
        ];
        let mut bits = Bits::copy_from_slice(data);

        assert_eq!(bits.get_u9::<LittleEndian>().unwrap(), u9::new(0b101010101));
        assert_eq!(
            bits.get_u10::<LittleEndian>().unwrap(),
            u10::new(0b1100001111)
        );
        assert_eq!(
            bits.get_u11::<LittleEndian>().unwrap(),
            u11::new(0b00011001100)
        );
        assert_eq!(
            bits.get_u12::<LittleEndian>().unwrap(),
            u12::new(0b000111111110)
        );
    }
}
