use std::{
    fmt::LowerHex,
    io::{Read, Seek, SeekFrom, Write},
};

use crate::{
    bit_read::BitRead,
    bit_seek::BitSeek,
    bit_write::BitWrite,
    borrow_bits::{BorrowBits, BorrowBitsMut},
    prelude::*,
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
    pub fn split(&self) -> (&BitSlice, &BitSlice) {
        let bits = self.inner.borrow_bits();
        bits.split_at(self.pos as usize)
    }
}

impl<T> BitCursor<T>
where
    T: BorrowBitsMut,
{
    pub fn split_mut(&mut self) -> (&mut BitSlice<BitSafeU8>, &mut BitSlice<BitSafeU8>) {
        let bits = self.inner.borrow_bits_mut();
        let (left, right) = bits.split_at_mut(self.pos as usize);
        (left, right)
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

impl<T> BitRead for BitCursor<T>
where
    T: BorrowBits,
{
    fn read_bits(&mut self, buf: &mut BitSlice) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut BitCursor::split(self).1, buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T> Write for BitCursor<T>
where
    T: BorrowBitsMut,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = Write::write(&mut BitCursor::split_mut(self).1, buf)?;
        self.pos += (n * 8) as u64;
        Ok(n)
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
    fn write_bits<O: BitStore>(&mut self, buf: &BitSlice<O>) -> std::io::Result<usize> {
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
    use std::fmt::Debug;
    use std::io::{Seek, SeekFrom};

    use crate::prelude::*;
    use bitvec::bits;
    use bitvec::bitvec;
    use bitvec::view::BitView;
    use nsw_types::*;

    use crate::bit_read::BitRead;
    use crate::bit_seek::BitSeek;
    use crate::bit_write_exts::BitWriteExts;
    use crate::borrow_bits::{BorrowBits, BorrowBitsMut};
    use crate::byte_order::NetworkOrder;

    use super::BitCursor;

    fn test_read_bits_hepler<T: BorrowBits>(buf: T, expected: &[u8]) {
        let expected_bits = expected.view_bits::<Msb0>();
        let mut cursor = BitCursor::new(buf);
        let mut read_buf = bitvec![u8, Msb0; 0; expected_bits.len()];
        assert_eq!(
            cursor.read_bits(&mut read_buf).unwrap(),
            expected_bits.len()
        );
        assert_eq!(read_buf, expected_bits);
    }

    #[test]
    fn test_read_bits() {
        let data = [0b11110000, 0b00001111];

        let vec = Vec::from(data);
        test_read_bits_hepler(vec, &data);

        let bitvec = BitVec::from_slice(&data);
        test_read_bits_hepler(bitvec, &data);

        let bitslice: &BitSlice = data.view_bits();
        test_read_bits_hepler(bitslice, &data);

        let u8_slice = &data[..];
        test_read_bits_hepler(u8_slice, &data);
    }

    #[test]
    fn test_read_bytes() {
        let data = BitVec::from_vec(vec![1, 2, 3, 4]);
        let mut cursor = BitCursor::new(data);

        let mut buf = [0u8; 2];
        std::io::Read::read(&mut cursor, &mut buf).expect("valid read");
        assert_eq!(buf, [1, 2]);
        std::io::Read::read(&mut cursor, &mut buf).expect("valid read");
        assert_eq!(buf, [3, 4]);
    }

    #[test]
    fn test_bit_seek() {
        let data = BitVec::from_vec(vec![0b11001100, 0b00110011]);
        let mut cursor = BitCursor::new(data);

        let mut read_buf = bitvec![u8, Msb0; 0; 4];

        cursor.bit_seek(SeekFrom::End(-2)).expect("valid seek");
        // Should now be reading the last 2 bits
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, bits![u8, Msb0; 1, 1, 0, 0]);
        // We already read to the end
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 0);

        // The read after the seek brought the cursor back to the end.  Now jump back 6 bits.
        cursor.bit_seek(SeekFrom::Current(-6)).expect("valid seek");
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, bits![u8, Msb0; 1, 1, 0, 0]);

        cursor.bit_seek(SeekFrom::Start(4)).expect("valid seek");
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, bits![u8, Msb0; 1, 1, 0, 0]);
    }

    #[test]
    fn test_seek() {
        let data = BitVec::from_vec(vec![0b11001100, 0b00110011]);
        let mut cursor = BitCursor::new(data);

        let mut read_buf = bitvec![u8, Msb0; 0; 2];
        cursor.seek(SeekFrom::End(-1)).unwrap();
        // Should now be reading the last byte
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, bits![u8, Msb0; 0, 0]);
        // Go back one byte
        cursor.seek(SeekFrom::Current(-1)).unwrap();
        // We should now be in bit position 2
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, bits![u8, Msb0; 0, 0]);
    }

    fn test_write_bits_helper<T: BorrowBitsMut>(buf: T) -> T {
        let mut cursor = BitCursor::new(buf);
        cursor.write_u4(u4::new(0b1100)).unwrap();
        cursor.write_u2(u2::new(0b11)).unwrap();
        cursor.write_u2(u2::new(0b00)).unwrap();
        cursor.write_u3(u3::new(0b110)).unwrap();
        cursor.write_u5(u5::new(0b01100)).unwrap();
        cursor.into_inner()
    }

    #[test]
    fn test_write_bits_bitvec() {
        let buf = BitVec::from_vec(vec![0; 2]);

        assert_eq!(
            test_write_bits_helper(buf),
            BitVec::from_vec(vec![0b11001100, 0b11001100])
        );
    }

    #[test]
    fn test_write_bits_vec() {
        let buf: Vec<u8> = vec![0, 0];

        assert_eq!(test_write_bits_helper(buf), [0b11001100, 0b11001100]);
    }

    #[test]
    fn test_write_bits_bit_slice() {
        let mut data = [0u8; 2];
        let buf: &mut BitSlice = data.view_bits_mut::<Msb0>();

        assert_eq!(
            test_write_bits_helper(buf),
            BitVec::from_vec(vec![0b11001100, 0b11001100]).as_bitslice()
        );
    }

    #[test]
    fn test_write_bits_u8_slice() {
        let mut buf = [0u8; 2];

        assert_eq!(
            test_write_bits_helper(&mut buf[..]),
            [0b11001100, 0b11001100]
        );
    }

    fn test_split_helper<T: BorrowBits>(buf: T, expected: &[u8]) {
        let expected_bits = expected.view_bits::<Msb0>();
        let mut cursor = BitCursor::new(buf);
        cursor.bit_seek(SeekFrom::Current(4)).unwrap();
        let (before, after) = cursor.split();

        assert_eq!(before, expected_bits[..4]);
        assert_eq!(after, expected_bits[4..]);
    }

    #[test]
    fn test_split() {
        let data = [0b11110011, 0b10101010];

        let vec = Vec::from(data);
        test_split_helper(vec, &data);

        let bitvec = BitVec::from_slice(&data);
        test_split_helper(bitvec, &data);

        let bitslice: &BitSlice = data.view_bits();
        test_split_helper(bitslice, &data);

        let u8_slice = &data[..];
        test_split_helper(u8_slice, &data);
    }

    // Maybe a bit paranoid, but this creates cursors using different inner types, splits the data,
    // then makes sure that cursors can be created from each split and the data read correctly
    #[test]
    fn test_cursors_from_splits() {
        let data = [0b11110011, 0b10101010];

        let vec = Vec::from(data);
        let mut vec_cursor = BitCursor::new(vec);
        vec_cursor.seek(SeekFrom::Start(1)).unwrap();
        let (left, right) = vec_cursor.split();
        test_read_bits_hepler(left, &data[..1]);
        test_read_bits_hepler(right, &data[1..]);

        let bitvec = BitVec::from_slice(&data);
        let mut bitvec_cursor = BitCursor::new(bitvec);
        bitvec_cursor.seek(SeekFrom::Start(1)).unwrap();
        let (left, right) = bitvec_cursor.split();
        test_read_bits_hepler(left, &data[..1]);
        test_read_bits_hepler(right, &data[1..]);

        let bitslice: &BitSlice = data.view_bits();
        let mut bitslice_cursor = BitCursor::new(bitslice);
        bitslice_cursor.seek(SeekFrom::Start(1)).unwrap();
        let (left, right) = bitslice_cursor.split();
        test_read_bits_hepler(left, &data[..1]);
        test_read_bits_hepler(right, &data[1..]);

        let u8_slice = &data[..];
        let mut u8_cursor = BitCursor::new(u8_slice);
        u8_cursor.seek(SeekFrom::Start(1)).unwrap();
        let (left, right) = u8_cursor.split();
        test_read_bits_hepler(left, &data[..1]);
        test_read_bits_hepler(right, &data[1..]);
    }

    // Assumes the given buf is 4 bytes long
    fn test_split_mut_helper<T, U, F>(buf: T, create_expected: F)
    where
        T: BorrowBitsMut + PartialEq<U> + Debug,
        U: Debug,
        F: FnOnce(&[u8]) -> U,
    {
        let mut cursor = BitCursor::new(buf);
        cursor.seek(SeekFrom::Start(2)).unwrap();
        {
            let (mut before, mut after) = cursor.split_mut();

            before
                .write_u16::<NetworkOrder>(0b1111111100000000)
                .unwrap();
            after.write_u16::<NetworkOrder>(0b1100110000110011).unwrap();
        }

        let data = cursor.into_inner();
        let expected = create_expected(&[0b11111111, 0b00000000, 0b11001100, 0b00110011]);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_split_mut() {
        let data = [0u8; 4];

        let vec = Vec::from(data);
        test_split_mut_helper(vec, |v| v.to_vec());

        let bitvec = BitVec::from_vec(vec![0u8; 4]);
        test_split_mut_helper(bitvec, |v| BitVec::from_vec(v.to_vec()));

        let mut data = [0u8; 4];
        let bitslice: &mut BitSlice = data.view_bits_mut();
        test_split_mut_helper(bitslice, |v| BitVec::from_vec(v.to_vec()));

        let mut data = [0u8; 4];
        let u8_slice = &mut data[..];
        test_split_mut_helper(u8_slice, |v| v.to_vec());
    }
}
