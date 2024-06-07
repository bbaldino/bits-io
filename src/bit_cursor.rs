use std::{
    fmt::LowerHex,
    io::{Read, Seek, SeekFrom, Write},
    ops::Range,
};

use bitvec::{order::Msb0, slice::BitSlice, vec::BitVec, view::BitView};

use crate::{bit_read::BitRead, bit_write::BitWrite};

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

impl BitCursor<BitVec<u8, Msb0>> {
    /// Create a BitCursor from a [`Vec<u8>`]
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self {
            inner: BitVec::from_vec(data),
            pos: 0,
        }
    }

    /// Get the data between the current cursor position and the end of the data as a [`BitSlice`]
    pub fn remaining_slice(&self) -> &BitSlice<u8, Msb0> {
        let len = self.pos.min(self.inner.capacity() as u64);
        &self.inner.as_bitslice()[(len as usize)..]
    }

    /// Get the data between the current cursor position and the end of the data as a mutable [`BitSlice`]
    pub fn remaining_slice_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
        let start = self.pos.min(self.inner.capacity() as u64);
        &mut self.inner.as_mut_bitslice()[(start as usize)..]
    }

    // TODO: BitSlice doesn't support ranges on anything that's RangeBounds, it implements the
    // individual range types.  For now, just support Range here, and in future maybe impl Index
    // with different range types for this as well.
    /// Grab a sub-cursor representing the given range.  The range is relevant to the _current_
    /// position of the cursor.
    pub fn sub_cursor(&self, range: Range<usize>) -> BitCursor<&BitSlice<u8, Msb0>> {
        let slice = &self.remaining_slice()[range];
        BitCursor::new(slice)
    }

    /// Returns true if the remaining slice is empty
    pub fn is_empty(&self) -> bool {
        self.pos >= self.remaining_slice().len() as u64
    }
}

impl BitCursor<&BitSlice<u8, Msb0>> {
    /// Get the data between the current cursor position and the end of the data as a [`BitSlice`]
    pub fn remaining_slice(&self) -> &BitSlice<u8, Msb0> {
        let len = self.pos.min(self.inner.len() as u64);
        &self.inner[(len as usize)..]
    }

    // TODO: BitSlice doesn't support ranges on anything that's RangeBounds, it implements the
    // individual range types.  For now, just support Range here, and in future maybe impl Index
    // with different range types for this as well.
    /// Grab a sub-cursor representing the given range.  The range is relevant to the _current_
    /// position of the cursor.
    pub fn sub_cursor(&self, range: Range<usize>) -> BitCursor<&BitSlice<u8, Msb0>> {
        let slice = &self.remaining_slice()[range];
        BitCursor::new(slice)
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.remaining_slice().len() as u64
    }
}

impl BitCursor<&[u8]> {
    pub fn remaining_slice(&self) -> &BitSlice<u8, Msb0> {
        // Here we have to mulitply the slice length by 8, since it's in bytes
        let len = self.pos.min((self.inner.len() * 8) as u64);
        &self.inner.view_bits::<Msb0>()[(len as usize)..]
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

impl Seek for BitCursor<&BitSlice<u8, Msb0>> {
    fn seek(&mut self, style: SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(self.pos);
            }
            SeekFrom::End(n) => (self.inner.len() as u64, n),
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

impl Seek for BitCursor<BitVec<u8, Msb0>> {
    fn seek(&mut self, style: SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(self.pos);
            }
            SeekFrom::End(n) => (self.inner.len() as u64, n),
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

impl Seek for BitCursor<&[u8]> {
    fn seek(&mut self, style: SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(self.pos);
            }
            SeekFrom::End(n) => (self.inner.len() as u64, n),
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

impl Read for BitCursor<BitVec<u8, Msb0>> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pos % 8 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level read when not on byte boundary",
            ));
        }
        match self.remaining_slice().read(buf) {
            Ok(n) => {
                self.pos += (n * 8) as u64;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

impl Read for BitCursor<&[u8]> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pos % 8 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level read when not on byte boundary",
            ));
        }
        match self.remaining_slice().read(buf) {
            Ok(n) => {
                self.pos += (n * 8) as u64;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

impl Read for BitCursor<&BitSlice<u8, Msb0>> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.pos % 8 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level read when not on byte boundary",
            ));
        }
        match self.remaining_slice().read(buf) {
            Ok(n) => {
                self.pos += (n * 8) as u64;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

impl BitRead for BitCursor<BitVec<u8, Msb0>> {
    fn read_bits(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut self.remaining_slice(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl BitRead for BitCursor<&BitSlice<u8, Msb0>> {
    fn read_bits(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut self.remaining_slice(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl BitRead for BitCursor<&[u8]> {
    fn read_bits(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut self.remaining_slice(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl Write for BitCursor<BitVec<u8, Msb0>> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.pos % 8 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level write when not on byte boundary",
            ));
        }
        match self.remaining_slice_mut().write(buf) {
            Ok(n) => {
                self.pos += (n * 8) as u64;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Write for BitCursor<&mut BitSlice<u8, Msb0>> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.pos % 8 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level write when not on byte boundary",
            ));
        }
        match self.inner.write(buf) {
            Ok(n) => {
                self.pos += (n * 8) as u64;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl BitWrite for BitCursor<BitVec<u8, Msb0>> {
    fn write_bits(&mut self, buf: &[ux::u1]) -> std::io::Result<usize> {
        let n = BitWrite::write_bits(&mut self.remaining_slice_mut(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl BitWrite for BitCursor<&mut BitSlice<u8, Msb0>> {
    fn write_bits(&mut self, buf: &[ux::u1]) -> std::io::Result<usize> {
        let n = BitWrite::write_bits(&mut self.inner, buf)?;
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
    use std::io::{Seek, SeekFrom};

    use bitvec::{bits, order::Msb0, vec::BitVec};
    use ux::u1;

    use crate::{bit_read::BitRead, bit_read_exts::BitReadExts};

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

    #[test]
    fn test_sub_cursor_vec() {
        let data = BitVec::<u8, Msb0>::from_vec(vec![1, 2, 3, 4]);
        let mut cursor = BitCursor::new(data);

        let _ = cursor.read_u8().unwrap();
        let mut sub_cursor = cursor.sub_cursor(0..24);

        assert_eq!(sub_cursor.remaining_slice().len(), 24);
        assert_eq!(sub_cursor.read_u8().unwrap(), 2);
    }

    #[test]
    fn test_remaining_slice_u8() {
        let data: Vec<u8> = vec![0b00001111, 0b10101010];

        let mut cursor = BitCursor::new(&data[..]);
        cursor.read_u4().unwrap();

        let slice = cursor.remaining_slice();
        assert_eq!(slice, bits![u8, Msb0; 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0]);
    }
}
