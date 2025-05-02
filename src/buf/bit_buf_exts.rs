use funty::Integral;

use super::bit_buf::BitBuf;
use crate::prelude::*;

pub trait BitBufExts: BitBuf {
    #[allow(non_snake_case)]
    fn get_uN<O: ByteOrder, const N: usize, U, V: Integral>(&mut self) -> std::io::Result<U>
    where
        U: TryFrom<V>,
        U::Error: std::fmt::Debug,
    {
        // If this buffer is chained to another there may be enough room to read the value but it
        // may not be continuguous.  If it is, then we can read directly instead of copying to an
        // intermediary first.
        let slice = self.chunk_bits();
        if slice.len() >= N {
            let value: V = O::load(&slice[..N]);
            self.advance_bits(N);

            Ok(U::try_from(value).map_err(|_| std::io::ErrorKind::InvalidData)?)
        } else {
            let mut bits = BitVec::repeat(false, N);
            let slice = bits.as_mut_bitslice();
            // Copy the raw bits into the slice
            self.try_copy_to_bit_slice(slice)?;
            // Now 'load' the value from that slice according to the given ByteOrder.
            let value: V = O::load(slice);

            Ok(U::try_from(value).map_err(|_| std::io::ErrorKind::InvalidData)?)
        }
    }

    fn get_bool(&mut self) -> std::io::Result<bool> {
        Ok(self.get_u1()?.into())
    }

    fn get_u1(&mut self) -> std::io::Result<u1> {
        self.get_uN::<BigEndian, 1, u1, u8>()
    }

    fn get_u2(&mut self) -> std::io::Result<u2> {
        self.get_uN::<BigEndian, 2, u2, u8>()
    }

    fn get_u3(&mut self) -> std::io::Result<u3> {
        self.get_uN::<BigEndian, 3, u3, u8>()
    }

    fn get_u4(&mut self) -> std::io::Result<u4> {
        self.get_uN::<BigEndian, 4, u4, u8>()
    }

    fn get_u5(&mut self) -> std::io::Result<u5> {
        self.get_uN::<BigEndian, 5, u5, u8>()
    }

    fn get_u6(&mut self) -> std::io::Result<u6> {
        self.get_uN::<BigEndian, 6, u6, u8>()
    }

    fn get_u7(&mut self) -> std::io::Result<u7> {
        self.get_uN::<BigEndian, 7, u7, u8>()
    }

    fn get_u8(&mut self) -> std::io::Result<u8> {
        if self.byte_aligned() {
            if self.remaining_bytes() < 1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    format!(
                        "Remaining bytes ({}) are less than the size of the dest (1)",
                        self.remaining_bytes(),
                    ),
                ));
            }
            let value = self.chunk_bytes()[0];
            self.advance_bytes(1);
            return Ok(value);
        }
        self.get_uN::<BigEndian, 8, u8, u8>()
    }

    fn get_u9<O: ByteOrder>(&mut self) -> std::io::Result<u9> {
        self.get_uN::<O, 9, u9, u16>()
    }
    fn get_u10<O: ByteOrder>(&mut self) -> std::io::Result<u10> {
        self.get_uN::<O, 10, u10, u16>()
    }
    fn get_u11<O: ByteOrder>(&mut self) -> std::io::Result<u11> {
        self.get_uN::<O, 11, u11, u16>()
    }
    fn get_u12<O: ByteOrder>(&mut self) -> std::io::Result<u12> {
        self.get_uN::<O, 12, u12, u16>()
    }
    fn get_u13<O: ByteOrder>(&mut self) -> std::io::Result<u13> {
        self.get_uN::<O, 13, u13, u16>()
    }
    fn get_u14<O: ByteOrder>(&mut self) -> std::io::Result<u14> {
        self.get_uN::<O, 14, u14, u16>()
    }
    fn get_u15<O: ByteOrder>(&mut self) -> std::io::Result<u15> {
        self.get_uN::<O, 15, u15, u16>()
    }
    fn get_u16<O: ByteOrder>(&mut self) -> std::io::Result<u16> {
        if self.byte_aligned() {
            if self.remaining_bytes() < 2 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    format!(
                        "Remaining bytes ({}) are less than the size of the dest (2)",
                        self.remaining_bytes(),
                    ),
                ));
            }
            let mut dest = [0u8; 2];
            self.try_copy_to_slice_bytes(&mut dest)?;
            return Ok(O::load_u16(&dest));
        }
        self.get_uN::<O, 16, u16, u16>()
    }
    fn get_u17<O: ByteOrder>(&mut self) -> std::io::Result<u17> {
        self.get_uN::<O, 17, u17, u32>()
    }
    fn get_u18<O: ByteOrder>(&mut self) -> std::io::Result<u18> {
        self.get_uN::<O, 18, u18, u32>()
    }
    fn get_u19<O: ByteOrder>(&mut self) -> std::io::Result<u19> {
        self.get_uN::<O, 19, u19, u32>()
    }
    fn get_u20<O: ByteOrder>(&mut self) -> std::io::Result<u20> {
        self.get_uN::<O, 20, u20, u32>()
    }
    fn get_u21<O: ByteOrder>(&mut self) -> std::io::Result<u21> {
        self.get_uN::<O, 21, u21, u32>()
    }
    fn get_u22<O: ByteOrder>(&mut self) -> std::io::Result<u22> {
        self.get_uN::<O, 22, u22, u32>()
    }
    fn get_u23<O: ByteOrder>(&mut self) -> std::io::Result<u23> {
        self.get_uN::<O, 23, u23, u32>()
    }
    fn get_u24<O: ByteOrder>(&mut self) -> std::io::Result<u24> {
        if self.byte_aligned() {
            if self.remaining_bytes() < 3 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    format!(
                        "Remaining bytes ({}) are less than the size of the dest (3)",
                        self.remaining_bytes(),
                    ),
                ));
            }
            let mut dest = [0u8; 3];
            self.try_copy_to_slice_bytes(&mut dest)?;
            return Ok(O::load_u24(&dest));
        }
        self.get_uN::<O, 24, u24, u32>()
    }
    fn get_u25<O: ByteOrder>(&mut self) -> std::io::Result<u25> {
        self.get_uN::<O, 25, u25, u32>()
    }
    fn get_u26<O: ByteOrder>(&mut self) -> std::io::Result<u26> {
        self.get_uN::<O, 26, u26, u32>()
    }
    fn get_u27<O: ByteOrder>(&mut self) -> std::io::Result<u27> {
        self.get_uN::<O, 27, u27, u32>()
    }
    fn get_u28<O: ByteOrder>(&mut self) -> std::io::Result<u28> {
        self.get_uN::<O, 28, u28, u32>()
    }
    fn get_u29<O: ByteOrder>(&mut self) -> std::io::Result<u29> {
        self.get_uN::<O, 29, u29, u32>()
    }
    fn get_u30<O: ByteOrder>(&mut self) -> std::io::Result<u30> {
        self.get_uN::<O, 30, u30, u32>()
    }
    fn get_u31<O: ByteOrder>(&mut self) -> std::io::Result<u31> {
        self.get_uN::<O, 31, u31, u32>()
    }
    fn get_u32<O: ByteOrder>(&mut self) -> std::io::Result<u32> {
        if self.byte_aligned() {
            if self.remaining_bytes() < 4 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    format!(
                        "Remaining bytes ({}) are less than the size of the dest (4)",
                        self.remaining_bytes(),
                    ),
                ));
            }
            let mut dest = [0u8; 4];
            self.try_copy_to_slice_bytes(&mut dest)?;
            return Ok(O::load_u32(&dest));
        }
        self.get_uN::<O, 32, u32, u32>()
    }
}

impl<T: BitBuf + ?Sized> BitBufExts for T {}

#[cfg(test)]
mod tests {
    use crate::buf::bits::Bits;

    use super::*;

    #[test]
    fn test_bit_buf_exts() {
        let mut bits = Bits::copy_from_bit_slice(bits![0, 1, 1, 0, 0, 1, 1, 1]);

        let value = bits.get_u4().unwrap();
        assert_eq!(value, u4::new(0b0110));
        let value = bits.get_u1().unwrap();
        assert_eq!(value, u1::new(0));
        let value = bits.get_u3().unwrap();
        assert_eq!(value, u3::new(0b111));
    }

    #[test]
    fn test_get_big_endian() {
        let u9_data = bits![1, 0, 1, 0, 1, 0, 1, 0, 1];
        let mut bits = Bits::copy_from_bit_slice(u9_data);
        assert_eq!(bits.get_u9::<BigEndian>().unwrap(), u9::new(0b101010101));

        let u10_data = bits![1, 0, 1, 0, 1, 0, 1, 0, 1, 1];
        let mut bits = Bits::copy_from_bit_slice(u10_data);
        assert_eq!(bits.get_u10::<BigEndian>().unwrap(), u10::new(0b1010101011));

        let u11_data = bits![0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0];
        let mut bits = Bits::copy_from_bit_slice(u11_data);
        assert_eq!(
            bits.get_u11::<BigEndian>().unwrap(),
            u11::new(0b00110011000)
        );

        let u12_data = bits![0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0];
        let mut bits = Bits::copy_from_bit_slice(u12_data);
        assert_eq!(
            bits.get_u12::<BigEndian>().unwrap(),
            u12::new(0b011111111000)
        );
    }

    #[test]
    fn test_get_little_endian() {
        // Little-endian form of 1_10101011
        let u9_data = bits![1, 0, 1, 0, 1, 0, 1, 1, 1];
        let mut bits = Bits::copy_from_bit_slice(u9_data);
        assert_eq!(
            bits.get_u9::<LittleEndian>().unwrap(),
            u9::new(0b1_10101011)
        );

        // Little-endian form of 11_10101011
        let u10_data = bits![1, 0, 1, 0, 1, 0, 1, 1, 1, 1];
        let mut bits = Bits::copy_from_bit_slice(u10_data);
        assert_eq!(
            bits.get_u10::<LittleEndian>().unwrap(),
            u10::new(0b11_10101011)
        );

        // Little-endian form of 110_10101011
        let u11_data = bits![1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0];
        let mut bits = Bits::copy_from_bit_slice(u11_data);
        assert_eq!(
            bits.get_u11::<LittleEndian>().unwrap(),
            u11::new(0b110_10101011)
        );

        // Little-endian form of 11001101_11101111
        let u16_data = bits![1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1];
        let mut bits = Bits::copy_from_bit_slice(u16_data);
        assert_eq!(
            bits.get_u16::<LittleEndian>().unwrap(),
            0b11001101_11101111u16
        );

        // Little-endian form of 11001101_11101111
        let u20_data = bits![1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0];
        let mut bits = Bits::copy_from_bit_slice(u20_data);
        assert_eq!(
            bits.get_u20::<LittleEndian>().unwrap(),
            u20::new(0b1010_10111100_11011110)
        );
    }
}
