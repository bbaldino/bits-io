use std::io::{Read, Seek, SeekFrom};

use crate::{
    bit_read::BitRead,
    bit_slice::{AsBitSlice, BitSlice},
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
    T: AsBitSlice,
{
    pub fn remaining_slice(&self) -> BitSlice {
        let len = self.pos.min(self.inner.as_bit_slice().len() as u64);
        self.inner.as_bit_slice().get_slice(len..).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.inner.as_bit_slice().len() as u64
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

impl<T> Seek for BitCursor<T>
where
    T: AsBitSlice,
{
    fn seek(&mut self, style: std::io::SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(self.pos);
            }
            SeekFrom::End(n) => (self.inner.as_bit_slice().len() as u64, n),
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

impl<T> BitRead for BitCursor<T>
where
    T: AsBitSlice,
{
    fn read(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let n = BitRead::read(&mut self.remaining_slice(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T> Read for BitCursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.pos % 8 {
            0 => {
                // Our position is in bits, so convert it to bytes
                let current_byte_pos = (self.pos / 8) as usize;
                // 'inner' is already a u8 slice, so we just grab its length
                let remaining_slice_len_bytes = self.inner.as_ref().len();
                let len = current_byte_pos.min(remaining_slice_len_bytes);

                let mut this = &self.inner.as_ref()[len..];
                match Read::read(&mut this, buf) {
                    Ok(n) => {
                        self.pos += (n * 8) as u64;
                        Ok(n)
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level read when not on byte boundary",
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{Seek, SeekFrom};

    use ux::u1;

    use super::BitCursor;
    use crate::{bit_read::BitRead, bit_read_exts::BitReadExts};

    #[test]
    fn test_read() {
        let data = vec![0b11110000, 0b00001111];
        let mut cursor = BitCursor::new(data);

        let mut read_buf = [u1::new(0); 4];
        assert_eq!(cursor.read(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(1), u1::new(1), u1::new(1), u1::new(1)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(0), u1::new(0), u1::new(0), u1::new(0)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(0), u1::new(0), u1::new(0), u1::new(0)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 4);
        assert_eq!(read_buf, [u1::new(1), u1::new(1), u1::new(1), u1::new(1)]);

        assert_eq!(cursor.read(&mut read_buf).unwrap(), 0);
    }

    #[test]
    fn test_seek() {
        let data = vec![0b11001100, 0b00110011];
        let mut cursor = BitCursor::new(data);

        let mut read_buf = [u1::new(0); 2];

        cursor.seek(SeekFrom::End(-2)).expect("valid seek");
        // Should now be reading the last 2 bits
        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
        assert_eq!(cursor.read(&mut read_buf).unwrap(), 0);

        // Now 4 bits from the end
        cursor.seek(SeekFrom::Current(-4)).expect("valid seek");
        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(0), u1::new(0)]);

        cursor.seek(SeekFrom::Start(4)).expect("valid seek");
        assert_eq!(cursor.read(&mut read_buf).unwrap(), 2);
        assert_eq!(read_buf, [u1::new(1), u1::new(1)]);
    }

    #[test]
    fn test_read_bytes() {
        let data: Vec<u8> = vec![1, 2, 3, 4];
        let mut cursor = BitCursor::new(data);

        let mut buf = [0u8; 2];
        std::io::Read::read(&mut cursor, &mut buf).expect("valid read");
        assert_eq!(buf, [1, 2]);
        std::io::Read::read(&mut cursor, &mut buf).expect("valid read");
        assert_eq!(buf, [3, 4]);
    }
}
