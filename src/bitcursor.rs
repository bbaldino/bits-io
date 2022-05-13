use std::{
    io::{Cursor, Read},
    ops::{BitOrAssign, ShlAssign},
};

use thiserror::Error;

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

pub type BitCursorResult<T> = Result<T, BitCursorError>;

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

    pub fn read_bit(&mut self) -> BitCursorResult<u8> {
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
}

#[cfg(test)]
mod tests {
    use ux::*;

    use crate::{bitcursor::BitCursor, bit_read::BitRead};

    #[test]
    fn test_read() {
        let data: Vec<u8> = vec![0b11110000, 0b00001111];
        let mut cursor = BitCursor::new(data);

        assert_eq!(u4::read(&mut cursor).unwrap(), u4::new(15));
        assert_eq!(u4::read(&mut cursor).unwrap(), u4::new(0));
        assert_eq!(u2::read(&mut cursor).unwrap(), u2::new(0));
        assert_eq!(u6::read(&mut cursor).unwrap(), u6::new(15));
    }

    #[test]
    fn read_on_non_byte_boundary() {
        let data: Vec<u8> = vec![0b11110000, 0b00001111];
        let mut cursor = BitCursor::new(data);

        let _ = u4::read(&mut cursor).unwrap();
        assert!(u8::read(&mut cursor).is_err());
    }

    #[test]
    fn read_too_far() {
        let data: Vec<u8> = vec![0b11110000];
        let mut cursor = BitCursor::new(data);

        assert!(u16::read(&mut cursor).is_err());
    }

    #[test]
    fn read_too_far_bits() {
        let data: Vec<u8> = vec![0b11110000];
        let mut cursor = BitCursor::new(data);

        let _ = u7::read(&mut cursor);
        assert!(u3::read(&mut cursor).is_err());
    }
}
