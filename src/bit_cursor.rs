use std::io::{Read, Seek, SeekFrom, Write};

use bitvec::{field::BitField, order::BitOrder, slice::BitSlice, store::BitStore, vec::BitVec};

use crate::bit_read::BitRead;

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

impl<T, O> BitCursor<BitVec<T, O>>
where
    T: BitStore,
    O: BitOrder,
{
    pub fn remaining_slice(&self) -> &BitSlice<T, O> {
        let remaining_slice = self.inner.as_bitslice();
        let len = self.pos.min(remaining_slice.len() as u64);
        &remaining_slice[(len as usize)..]
    }

    pub fn remaining_slice_mut(&mut self) -> &mut BitSlice<T, O> {
        let remaining_slice = self.inner.as_mut_bitslice();
        let len = self.pos.min(remaining_slice.len() as u64);
        &mut remaining_slice[(len as usize)..]
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.remaining_slice().len() as u64
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

impl<T, O> Seek for BitCursor<BitVec<T, O>>
where
    T: BitStore,
    O: BitOrder,
{
    fn seek(&mut self, style: SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(self.pos);
            }
            SeekFrom::End(n) => (self.inner.as_bitslice().len() as u64, n),
            SeekFrom::Current(n) => (self.pos, n),
        };
        match base_pos.checked_add_signed(offset) {
            Some(n) => {
                self.pos = n;
                Ok(self.pos)
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}

impl<T, O> BitCursor<BitVec<T, O>>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    fn read_bits(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut self.remaining_slice(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T, O> Read for BitCursor<BitVec<T, O>>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.pos % 8 {
            0 => match self.remaining_slice().read(buf) {
                Ok(n) => {
                    self.pos += (n * 8) as u64;
                    Ok(n)
                }
                Err(e) => Err(e),
            },
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level read when not on byte boundary",
            )),
        }
    }
}

impl<T, O> BitRead for BitCursor<BitVec<T, O>>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    fn read_bits(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        BitCursor::read_bits(self, buf)
    }
}

impl<T, O> Write for BitCursor<BitVec<T, O>>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.pos % 8 {
            0 => match self.remaining_slice_mut().write(buf) {
                Ok(n) => {
                    self.pos += (n * 8) as u64;
                    Ok(n)
                }
                Err(e) => Err(e),
            },
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level write when not on byte boundary",
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::{Seek, SeekFrom};

    use bitvec::{order::Msb0, vec::BitVec};
    use ux::u1;

    use super::BitCursor;

    #[test]
    fn test_read() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![0b11110000, 0b00001111]);
        let mut cursor = BitCursor::new(data);

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
    fn test_seek() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![0b11001100, 0b00110011]);
        let mut cursor = BitCursor::new(data);

        let mut read_buf = [u1::new(0); 2];

        cursor.seek(SeekFrom::End(-2)).expect("valid seek");
        // Should now be reading the last 2 bits
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 0);

        // Now 4 bits from the end
        cursor.seek(SeekFrom::Current(-4)).expect("valid seek");
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);

        cursor.seek(SeekFrom::Start(4)).expect("valid seek");
        assert_eq!(cursor.read_bits(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
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
}
