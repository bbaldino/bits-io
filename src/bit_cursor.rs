use std::{
    fmt::LowerHex,
    io::{Read, Seek, SeekFrom, Write},
};

use bitvec::{
    access::BitSafeU8,
    bits,
    order::Msb0,
    slice::BitSlice,
    vec::BitVec,
    view::{AsBits, AsMutBits},
};

use crate::{
    bit_read::BitRead,
    bit_seek::BitSeek,
    bit_write::BitWrite,
    borrow_bits::{BorrowBits, BorrowBitsMut},
};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct BitCursor<T> {
    inner: T,
    pos: u64,
}

impl<T> BitCursor<T> {
    /// Creates a new cursor wrapping the provided buffer.
    ///
    /// Cursor initial position is `0` even if the given buffer is not empty.
    pub fn new(inner: T) -> BitCursor<T> {
        BitCursor { inner, pos: 0 }
    }

    /// Gets a mutable reference to the inner value
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Gets a reference to the inner value
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Consumes the cursor, returning the inner value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Returns the position (in _bits_ since the start) of this cursor.
    pub fn position(&self) -> u64 {
        self.pos
    }

    /// Sets the position of this cursor (in _bits_ since the start)
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> BitCursor<T>
where
    T: BorrowBits,
{
    pub fn split(&self) -> (&BitSlice<u8, Msb0>, &BitSlice<u8, Msb0>) {
        let bits = self.inner.borrow_bits();
        bits.split_at(self.pos as usize)
    }
}

// impl<T> BitCursor<T> {
//     pub fn split<'a>(&'a self) -> (&'a BitSlice<u8, Msb0>, &'a BitSlice<u8, Msb0>)
//     where
//         &'a T: IntoBits<'a>,
//     {
//         let inner_ref = &self.inner;
//         let slice = inner_ref.into_bits();
//         let pos = self.pos.min(slice.len() as u64);
//         slice.split_at(pos as usize)
//     }
// }

// impl<T> BitCursor<T>
// where
//     T: AsRef<BitSlice<u8, Msb0>>,
// {
//     pub fn split(&self) -> (&BitSlice<u8, Msb0>, &BitSlice<u8, Msb0>) {
//         let slice = self.inner.as_ref();
//         let pos = self.pos.min(slice.len() as u64);
//         slice.split_at(pos as usize)
//     }
// }

impl<T> BitCursor<T>
where
    T: BorrowBitsMut,
{
    pub fn split_mut(
        &mut self,
    ) -> (
        &mut BitSlice<BitSafeU8, Msb0>,
        &mut BitSlice<BitSafeU8, Msb0>,
    ) {
        let bits = self.inner.borrow_bits_mut();
        let (left, right) = bits.split_at_mut(self.pos as usize);
        (left, right)
    }
}
// impl<T> BitCursor<T>
// where
//     T: AsMut<BitSlice<u8, Msb0>>,
// {
//     pub fn split_mut(
//         &mut self,
//     ) -> (
//         &mut BitSlice<BitSafeU8, Msb0>,
//         &mut BitSlice<BitSafeU8, Msb0>,
//     ) {
//         let slice = self.inner.as_mut();
//         let pos = self.pos.min(slice.len() as u64);
//         slice.split_at_mut(pos as usize)
//     }
// }

struct BytesWrapper(bytes::Bytes);

impl AsRef<BitSlice<u8, Msb0>> for BytesWrapper {
    fn as_ref(&self) -> &BitSlice<u8, Msb0> {
        self.0.as_bits()
    }
}

struct BytesMutWrapper(bytes::BytesMut);

impl AsMut<BitSlice<u8, Msb0>> for BytesMutWrapper {
    fn as_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
        self.0.as_mut_bits()
    }
}

impl AsRef<BitSlice<u8, Msb0>> for BytesMutWrapper {
    fn as_ref(&self) -> &BitSlice<u8, Msb0> {
        self.0.as_bits()
    }
}

impl<T> Clone for BitCursor<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        BitCursor {
            inner: self.inner.clone(),
            pos: self.pos,
        }
    }
}

impl<T> BitSeek for BitCursor<T>
where
    T: BorrowBits,
{
    fn bit_seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match pos {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(n);
            }
            SeekFrom::End(n) => (self.inner.borrow_bits().len() as u64, n),
            SeekFrom::Current(n) => (self.pos, n),
        };
        match base_pos.checked_add_signed(offset) {
            Some(n) => {
                self.pos = n;
                Ok(self.pos)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overlfowing position",
            )),
        }
    }
}

impl<T> Seek for BitCursor<T>
where
    T: BorrowBits,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(n) => self.bit_seek(SeekFrom::Start(n * 8)),
            SeekFrom::End(n) => self.bit_seek(SeekFrom::End(n * 8)),
            SeekFrom::Current(n) => self.bit_seek(SeekFrom::Current(n * 8)),
        }
    }
}

impl<T> Read for BitCursor<T>
where
    T: BorrowBits,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bits = self.inner.borrow_bits();
        let remaining = &bits[self.pos as usize..];
        let mut bytes_read = 0;

        for (i, chunk) in remaining.chunks(8).take(buf.len()).enumerate() {
            let mut byte = 0u8;
            for (j, bit) in chunk.iter().enumerate() {
                if *bit {
                    byte |= 1 << (7 - j);
                }
            }
            buf[i] = byte;
            bytes_read += 1;
        }

        self.pos += (bytes_read * 8) as u64;
        Ok(bytes_read)
    }
}
// impl<T> Read for BitCursor<T>
// where
//     for<'a> T: IntoBits<'a>,
// {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         let n = Read::read(&mut BitCursor::split(self).1, buf)?;
//         self.pos += (n * 8) as u64;
//
//         Ok(n)
//     }
// }

// fn bar<T>(_value: T)
// where
//     for<'a> T: IntoBits<'a>,
// {
// }
//
// fn baz(v: &BitSlice<u8, Msb0>) {
//     bar(v);
// }

// fn foo(mut v: BitCursor<&BitSlice<u8, Msb0>>) {
//     let mut data = Vec::with_capacity(4);
//     v.read(&mut data).unwrap();
// }

fn test() {
    let bv = BitVec::<u8, Msb0>::from_element(0b10101010);
    let mut c = BitCursor::new(bv);
    let mut buf = [0u8; 1];
    c.read_exact(&mut buf).unwrap();

    let v = vec![0b11001100];
    let mut c = BitCursor::new(v);
    c.read_exact(&mut buf).unwrap();

    let slice: &[u8] = &[0b11110000];
    let mut c = BitCursor::new(slice);
    c.read_exact(&mut buf).unwrap();

    let bs: &BitSlice<u8, Msb0> = bits![u8, Msb0; 1; 8];
    let mut c = BitCursor::new(bs);
    c.read_exact(&mut buf).unwrap();
}

// impl<T> Read for BitCursor<T>
// where
//     T: AsRef<BitSlice<u8, Msb0>>,
// {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         let n = Read::read(&mut BitCursor::split(self).1, buf)?;
//         self.pos += (n * 8) as u64;
//
//         Ok(n)
//     }
// }
impl<T> BitRead for BitCursor<T>
where
    T: BorrowBits,
{
    fn read_bits(&mut self, buf: &mut [nsw_types::u1]) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut BitCursor::split(self).1, buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

// impl<T> BitRead for BitCursor<T>
// where
//     T: AsRef<BitSlice<u8, Msb0>>,
//     // T: IntoBits,
// {
//     fn read_bits(&mut self, buf: &mut [nsw_types::u1]) -> std::io::Result<usize> {
//         let n = BitRead::read_bits(&mut BitCursor::split(self).1, buf)?;
//         self.pos += n as u64;
//         Ok(n)
//     }
// }

// impl<T> Write for BitCursor<T>
// where
//     T: BorrowBitsMut,
// {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         let n = Write::write(&mut BitCursor::split_mut(self).1, buf)?;
//         self.pos += (n * 8) as u64;
//         Ok(n)
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         Ok(())
//     }
// }

impl<T> Write for BitCursor<T>
where
    T: BorrowBitsMut,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = Write::write(&mut BitCursor::split_mut(self).1, buf)?;
        self.pos += (n * 8) as u64;
        Ok(n)
        // let bits = self.inner.borrow_bits_mut();
        // let dst = &mut bits[self.pos as usize..];
        // let mut bytes_written = 0;
        //
        // for (i, chunk) in dst.chunks_mut(8).zip(buf.iter()).enumerate() {
        //     for j in 0..chunk.len() {
        //         chunk.set(j, (buf[i] >> (7 - j)) & 1 != 0);
        //     }
        //     bytes_written += 1;
        // }
        //
        // self.pos += (bytes_written * 8) as u64;
        // Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<T> BitWrite for BitCursor<T>
where
    T: BorrowBitsMut,
    BitCursor<T>: std::io::Write,
{
    fn write_bits(&mut self, buf: &[nsw_types::u1]) -> std::io::Result<usize> {
        let n = BitWrite::write_bits(&mut BitCursor::split_mut(self).1, buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T> LowerHex for BitCursor<T>
where
    T: LowerHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "buf: {:x}, pos: {}", self.inner, self.pos)
    }
}

#[cfg(test)]
mod test {
    use std::io::{Seek, SeekFrom, Write};

    use bitvec::slice::BitSlice;
    use bitvec::{bits, order::Msb0, vec::BitVec};
    use nsw_types::*;

    use crate::bit_seek::BitSeek;
    use crate::bit_write_exts::BitWriteExts;
    use crate::borrow_bits::BorrowBits;
    use crate::byte_order::NetworkOrder;
    use crate::{bit_read::BitRead, bit_read_exts::BitReadExts};

    use super::BitCursor;

    /// Test helper to
    fn test_read_bits<T: BorrowBits>(mut cursor: BitCursor<T>) {
        let mut read_buf = [u1::new(0); 4];
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(1), u1::new(1), u1::new(1), u1::new(1)]);

        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(0), u1::new(0), u1::new(0), u1::new(0)]);

        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(0), u1::new(0), u1::new(0), u1::new(0)]);

        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(1), u1::new(1), u1::new(1), u1::new(1)]);

        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 0);
    }

    #[test]
    fn test_read_bits_bitvec() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![0b11110000, 0b00001111]);
        let cursor = BitCursor::new(data);

        test_read_bits(cursor);
    }

    #[test]
    fn test_read_bits_bit_slice() {
        let bits: &BitSlice<u8, Msb0> =
            bits![u8, Msb0; 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1];
        let cursor = BitCursor::new(bits);
        test_read_bits(cursor);
    }

    #[test]
    fn test_read_bits_u8_slice() {
        let data: &[u8] = &[0b11110000, 0b00001111];
        let cursor = BitCursor::new(data);
        test_read_bits(cursor);
    }

    #[test]
    fn test_read_bits_vec() {
        let data: Vec<u8> = vec![0b11110000, 0b00001111];
        let cursor = BitCursor::new(data);
        test_read_bits(cursor);
    }

    #[test]
    fn test_read_bytes() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![1, 2, 3, 4]);
        let mut cursor = BitCursor::new(data);

        let mut buf = [0u8; 2];
        std::io::Read::read(&mut cursor, &mut buf).expect("valid read");
        assert_eq!(buf, [1, 2]);
        std::io::Read::read(&mut cursor, &mut buf).expect("valid read");
        assert_eq!(buf, [3, 4]);
    }

    #[test]
    fn test_bit_seek() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![0b11001100, 0b00110011]);
        let mut cursor = BitCursor::new(data);

        let mut read_buf = [u1::new(0); 2];

        cursor.bit_seek(SeekFrom::End(-2)).expect("valid seek");
        // Should now be reading the last 2 bits
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 0);

        // Now 4 bits from the end
        cursor.bit_seek(SeekFrom::Current(-4)).expect("valid seek");
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);

        cursor.bit_seek(SeekFrom::Start(4)).expect("valid seek");
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
    }

    #[test]
    fn test_seek() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![0b11001100, 0b00110011]);
        let mut cursor = BitCursor::new(data);

        let mut read_buf = [u1::new(0); 2];
        cursor.seek(SeekFrom::End(-1)).unwrap();
        // Should now be reading the last byte
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);
        // Go back one byte
        cursor.seek(SeekFrom::Current(-1)).unwrap();
        // We should now be in bit position 2
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);
    }

    #[test]
    fn test_write_bits() {
        let buf: Vec<u8> = vec![0, 0];
        let mut cursor = BitCursor::new(buf);

        cursor.write_u4(u4::new(0b1100)).unwrap();
        cursor.write_u2(u2::new(0b11)).unwrap();
        cursor.write_u2(u2::new(0b00)).unwrap();
        cursor.write_u3(u3::new(0b110)).unwrap();
        cursor.write_u5(u5::new(0b01100)).unwrap();
        let buf = cursor.into_inner();
        assert_eq!(buf, [0b11001100, 0b11001100]);
    }

    #[test]
    fn test_split() {
        let data: Vec<u8> = vec![0b11110011, 0b10101010];

        let mut cursor = BitCursor::new(data);
        cursor.bit_seek(SeekFrom::Current(4)).unwrap();
        let (before, after) = cursor.split();

        assert_eq!(before, bits!(u8, Msb0; 1, 1, 1, 1));
        assert_eq!(after, bits!(u8, Msb0; 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0));

        let mut before_cursor = BitCursor::new(before);
        assert_eq!(before_cursor.read_u4().unwrap(), u4::new(0b1111));
        let mut after_cursor = BitCursor::new(after);
        assert_eq!(after_cursor.read_u4().unwrap(), u4::new(0b0011));
        assert_eq!(after_cursor.read_u8().unwrap(), 0b10101010u8);
    }

    #[test]
    fn test_split_mut() {
        let mut bytes = Vec::with_capacity(4);
        bytes.write_all(&[0, 0, 0, 0]).unwrap();
        let mut cursor = BitCursor::new(bytes);
        cursor.bit_seek(SeekFrom::Start(16)).unwrap();

        {
            let (mut before, mut after) = cursor.split_mut();

            before
                .write_u16::<NetworkOrder>(0b1111111100000000)
                .unwrap();
            after.write_u16::<NetworkOrder>(0b1100110000110011).unwrap();
        }

        let data = cursor.into_inner();

        assert_eq!(vec![0b11111111, 0b00000000, 0b11001100, 0b00110011], data);
    }
}
