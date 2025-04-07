use std::{
    fmt::LowerHex,
    io::{Read, Seek, SeekFrom, Write},
};

use bitvec::{
    access::BitSafeU8,
    order::Msb0,
    slice::BitSlice,
    vec::BitVec,
    view::{AsBits, AsMutBits},
};

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
    T: AsRef<BitSlice<u8, Msb0>>,
{
    pub fn split(&self) -> (&BitSlice<u8, Msb0>, &BitSlice<u8, Msb0>) {
        let slice = self.inner.as_ref();
        let pos = self.pos.min(slice.len() as u64);
        slice.split_at(pos as usize)
    }
}

impl<T> BitCursor<T>
where
    T: AsMut<BitSlice<u8, Msb0>>,
{
    pub fn split_mut(
        &mut self,
    ) -> (
        &mut BitSlice<BitSafeU8, Msb0>,
        &mut BitSlice<BitSafeU8, Msb0>,
    ) {
        let slice = self.inner.as_mut();
        let pos = self.pos.min(slice.len() as u64);
        slice.split_at_mut(pos as usize)
    }
}

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

// impl BitCursor<BitVec<u8, Msb0>> {
//     /// Create a BitCursor from a [`Vec<u8>`]
//     pub fn from_vec(data: Vec<u8>) -> Self {
//         Self {
//             inner: BitVec::from_vec(data),
//             pos: 0,
//         }
//     }
//
//     /// Get the data between the current cursor position and the end of the data as a [`BitSlice`]
//     pub fn remaining_slice(&self) -> &BitSlice<u8, Msb0> {
//         let len = self.pos.min(self.inner.capacity() as u64);
//         &self.inner.as_bitslice()[(len as usize)..]
//     }
//
//     /// Get the data between the current cursor position and the end of the data as a mutable [`BitSlice`]
//     pub fn remaining_slice_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
//         let start = self.pos.min(self.inner.capacity() as u64);
//         &mut self.inner.as_mut_bitslice()[(start as usize)..]
//     }
//
//     /// Returns true if the remaining slice is empty
//     pub fn is_empty(&self) -> bool {
//         self.pos >= self.remaining_slice().len() as u64
//     }
// }

// impl BitCursor<&BitSlice<u8, Msb0>> {
//     /// Get the data between the current cursor position and the end of the data as a [`BitSlice`]
//     pub fn remaining_slice(&self) -> &BitSlice<u8, Msb0> {
//         let len = self.pos.min(self.inner.len() as u64);
//         &self.inner[(len as usize)..]
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.pos >= self.remaining_slice().len() as u64
//     }
// }

// impl BitCursor<&[u8]> {
//     pub fn remaining_slice(&self) -> &BitSlice<u8, Msb0> {
//         // Here we have to mulitply the slice length by 8, since it's in bytes
//         let len = self.pos.min((self.inner.len() * 8) as u64);
//         &self.inner.view_bits::<Msb0>()[(len as usize)..]
//     }
// }

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
    T: AsRef<BitSlice<u8, Msb0>>,
{
    fn seek(&mut self, style: SeekFrom) -> std::io::Result<u64> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(self.pos);
            }
            SeekFrom::End(n) => (self.inner.as_ref().len() as u64, n),
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

impl<T> Read for BitCursor<T>
where
    T: AsRef<BitSlice<u8, Msb0>>,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = Read::read(&mut BitCursor::split(self).1, buf)?;
        self.pos += (n * 8) as u64;

        Ok(n)
    }
}

// impl Read for BitCursor<BitVec<u8, Msb0>> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         if self.pos % 8 != 0 {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Attempted byte-level read when not on byte boundary",
//             ));
//         }
//         match self.remaining_slice().read(buf) {
//             Ok(n) => {
//                 self.pos += (n * 8) as u64;
//                 Ok(n)
//             }
//             Err(e) => Err(e),
//         }
//     }
// }

// impl Read for BitCursor<&[u8]> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         if self.pos % 8 != 0 {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Attempted byte-level read when not on byte boundary",
//             ));
//         }
//         match self.remaining_slice().read(buf) {
//             Ok(n) => {
//                 self.pos += (n * 8) as u64;
//                 Ok(n)
//             }
//             Err(e) => Err(e),
//         }
//     }
// }
//
// impl Read for BitCursor<&BitSlice<u8, Msb0>> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         if self.pos % 8 != 0 {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "Attempted byte-level read when not on byte boundary",
//             ));
//         }
//         match self.remaining_slice().read(buf) {
//             Ok(n) => {
//                 self.pos += (n * 8) as u64;
//                 Ok(n)
//             }
//             Err(e) => Err(e),
//         }
//     }
// }

impl<T> BitRead for BitCursor<T>
where
    T: AsRef<BitSlice<u8, Msb0>>,
{
    fn read_bits(&mut self, buf: &mut [nsw_types::u1]) -> std::io::Result<usize> {
        let n = BitRead::read_bits(&mut self.inner.as_ref(), buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl<T> Write for BitCursor<T>
where
    T: AsMut<BitSlice<u8, Msb0>>,
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
    T: AsMut<BitSlice<u8, Msb0>>,
    BitCursor<T>: std::io::Write,
{
    fn write_bits(&mut self, buf: &[nsw_types::u1]) -> std::io::Result<usize> {
        let n = BitWrite::write_bits(&mut self.inner.as_mut(), buf)?;
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
    use std::io::Write;
    use std::io::{Seek, SeekFrom};

    use bitvec::view::AsBits;
    use bitvec::{bits, order::Msb0, vec::BitVec};
    use bytes::BufMut;
    use bytes::Bytes;
    use bytes::BytesMut;
    use nsw_types::u1;
    use nsw_types::u4;

    use crate::bit_write_exts::BitWriteExts;
    use crate::byte_order::NetworkOrder;
    use crate::{bit_read::BitRead, bit_read_exts::BitReadExts};

    use super::BytesMutWrapper;
    use super::{BitCursor, BytesWrapper};

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

    // TODO: use AsBits/AsBitsMut instead of AsRef/AsMut?
    fn foo<T>(value: T)
    where
        T: AsBits<u8>,
    {
    }

    #[test]
    fn test_remaining_slice_u8() {
        let data: Vec<u8> = vec![0b00001111, 0b10101010];

        foo(&data[..]);

        // let mut cursor = BitCursor::new(&data[..]);
        // cursor.read_u4().unwrap();
        //
        // let (before, after) = cursor.split();
        // assert_eq!(after, bits![u8, Msb0; 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_split() {
        let data: Vec<u8> = vec![0b11110011, 0b10101010];
        let bytes = BytesWrapper(Bytes::from(data));

        let mut cursor = BitCursor::new(bytes);
        cursor.seek(SeekFrom::Current(4)).unwrap();
        let (before, after) = cursor.split();
        dbg!(&before);
        dbg!(&after);

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
        let bytes = BytesMut::with_capacity(4);
        let mut writer = bytes.writer();
        writer.write_all(&[0, 0, 0, 0]).unwrap();
        let bytes = BytesMutWrapper(writer.into_inner());
        let mut cursor = BitCursor::new(bytes);
        cursor.seek(SeekFrom::Start(16)).unwrap();

        {
            let (mut before, mut after) = cursor.split_mut();

            before
                .write_u16::<NetworkOrder>(0b1111111100000000)
                .unwrap();
            after.write_u16::<NetworkOrder>(0b1100110000110011).unwrap();
        }

        let data = cursor.into_inner();

        assert_eq!(
            Bytes::from(vec![0b11111111, 0b00000000, 0b11001100, 0b00110011]),
            data.0
        );
    }
}
