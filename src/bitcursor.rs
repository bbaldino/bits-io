use std::{
    io::{Cursor, Read},
    ops::{BitOrAssign, ShlAssign},
};

use thiserror::Error;

use crate::types::*;
use byteorder::{NetworkEndian, ReadBytesExt};

#[derive(Debug, Error)]
pub enum BitCursorError {
    #[error("BufferOverflow: {0}")]
    BufferOverflow(String),
    #[error("IO error: {0}")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for BitCursorError {
    fn from(io_err: std::io::Error) -> Self {
        BitCursorError::IoError(io_err)
    }
}

type BitCursorResult<T> = Result<T, BitCursorError>;

/// Similar to |std::io::Cursor| but designed to keep track of a buffer of bytes where amounts less
/// than a single byte (i.e. some number of bits) can be read.
#[derive(Debug)]
pub struct BitCursor {
    byte_cursor: Cursor<Vec<u8>>,
    bit_pos: u8,
}

impl Read for BitCursor {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.bit_pos {
            0 => self.byte_cursor.read(buf),
            bp => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Cannot do a byte-level read when not on a byte boundary; cursor is currently on bit {}",
                    bp
                ),
            )),
        }
    }
}

/// A helper which allows reading any number of bits into the given type T.
fn bit_read_helper<T>(buf: &mut BitCursor, num_bits: usize) -> BitCursorResult<T>
where
    T: Default + ShlAssign<u8> + BitOrAssign + From<u8>,
{
    // TODO: this may not be performant enough in the long run, but works for now.  In the future
    // it could be optimized by reading bigger chunks that at a time and then either combining them
    // or masking them.  For example, if num_bits is 5, it's probably faster to:
    //   peek a u8
    //   mask off the appropriate bits on the left/right depending on the current bit position
    //   shift the result to the right
    let mut result = Default::default();
    for _ in 0..num_bits {
        result <<= 1;
        result |= buf.read_bit()?.into();
    }
    Ok(result)
}

impl BitCursor {
    pub fn new(data: Vec<u8>) -> Self {
        BitCursor {
            byte_cursor: Cursor::new(data),
            bit_pos: 0,
        }
    }

    fn increment_bit_pos(&mut self, num_bits: usize) {
        self.bit_pos += num_bits as u8;
        if self.bit_pos >= 7 {
            let num_bytes = self.bit_pos / 8;
            self.bit_pos %= 8;
            self.byte_cursor
                .set_position(self.byte_cursor.position() + num_bytes as u64);
        }
    }

    fn get_curr_byte(&self) -> BitCursorResult<u8> {
        if self.byte_cursor.position() >= self.byte_cursor.get_ref().len() as u64 {
            return Err(BitCursorError::BufferOverflow(format!(
                "Tried to access index {}, but underlying data has size {}",
                self.byte_cursor.position(),
                self.byte_cursor.get_ref().len()
            )));
        }
        Ok(self.byte_cursor.get_ref()[self.byte_cursor.position() as usize])
    }

    fn read_bit(&mut self) -> BitCursorResult<u8> {
        let mask = 1u8 << (7 - self.bit_pos);
        let curr_byte = self.get_curr_byte()?;
        let result = (curr_byte & mask) >> (7 - self.bit_pos);
        self.increment_bit_pos(1);
        Ok(result)
    }

    pub fn bytes_remaining(&self) -> usize {
        match self.bit_pos {
            0 => self.byte_cursor.get_ref().len() - self.byte_cursor.position() as usize,
            // If we're in the middle of a byte, don't count that as a full byte remaining
            // (Note that this is a somewhat arbitrary decision, but it's what makes more sense
            // to me as of now)
            _ => self.byte_cursor.get_ref().len() - self.byte_cursor.position() as usize - 1,
        }
    }

    pub fn read_bits_as_u8(&mut self, num_bits: usize) -> BitCursorResult<u8> {
        bit_read_helper::<u8>(self, num_bits)
    }

    pub fn read_bits_as_u16(&mut self, num_bits: usize) -> BitCursorResult<u16> {
        bit_read_helper::<u16>(self, num_bits)
    }

    pub fn read_bits_as_u32(&mut self, num_bits: usize) -> BitCursorResult<u32> {
        bit_read_helper::<u32>(self, num_bits)
    }

    pub fn read_bool(&mut self) -> BitCursorResult<bool> {
        Ok(self.read_bit()? == 1)
    }
    pub fn read_u2(&mut self) -> BitCursorResult<u2> {
        self.read_bits_as_u8(2)
    }
    pub fn read_u3(&mut self) -> BitCursorResult<u3> {
        self.read_bits_as_u8(3)
    }
    pub fn read_u4(&mut self) -> BitCursorResult<u4> {
        self.read_bits_as_u8(4)
    }
    pub fn read_u5(&mut self) -> BitCursorResult<u5> {
        self.read_bits_as_u8(5)
    }
    pub fn read_u6(&mut self) -> BitCursorResult<u6> {
        self.read_bits_as_u8(6)
    }
    pub fn read_u7(&mut self) -> BitCursorResult<u7> {
        self.read_bits_as_u8(7)
    }
    pub fn read_u8(&mut self) -> BitCursorResult<u8> {
        ReadBytesExt::read_u8(self).map_err(std::io::Error::into)
    }
    pub fn read_u14(&mut self) -> BitCursorResult<u14> {
        self.read_bits_as_u16(14)
    }
    pub fn read_u16(&mut self) -> BitCursorResult<u16> {
        ReadBytesExt::read_u16::<NetworkEndian>(self).map_err(std::io::Error::into)
    }
    pub fn read_u24(&mut self) -> BitCursorResult<u24> {
        ReadBytesExt::read_u24::<NetworkEndian>(self).map_err(std::io::Error::into)
    }
    pub fn read_u32(&mut self) -> BitCursorResult<u32> {
        ReadBytesExt::read_u32::<NetworkEndian>(self).map_err(std::io::Error::into)
    }
    pub fn read_u128(&mut self) -> BitCursorResult<u128> {
        ReadBytesExt::read_u128::<NetworkEndian>(self).map_err(std::io::Error::into)
    }
}

#[cfg(test)]
mod tests {
    use crate::bitcursor::BitCursor;

    #[test]
    fn read_on_non_byte_boundary() {
        let data: Vec<u8> = vec![0b11110000, 0b00001111];
        let mut cursor = BitCursor::new(data);

        let _ = cursor.read_u4().unwrap();
        assert!(cursor.read_u8().is_err());
    }

    #[test]
    fn read_too_far() {
        let data: Vec<u8> = vec![0b11110000];
        let mut cursor = BitCursor::new(data);

        assert!(cursor.read_u16().is_err());
    }

    #[test]
    fn read_too_far_bits() {
        let data: Vec<u8> = vec![0b11110000];
        let mut cursor = BitCursor::new(data);

        let _ = cursor.read_u7();
        assert!(cursor.read_u3().is_err());
    }
}
