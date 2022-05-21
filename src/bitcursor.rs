use std::{
    io::{Cursor, Read},
    ops::{BitOrAssign, ShlAssign},
};

use thiserror::Error;

use crate::bit_seek::{BitSeek, BitSeekFrom};

#[derive(Debug, Error)]
pub enum BitCursorError {
    #[error("BufferOverflow: {0}")]
    BufferOverflow(String),
    #[error("BufferUnderflow: {0}")]
    BufferUnderflow(String),
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
                .set_position(self.curr_byte_position() + num_bytes as u64);
        }
    }

    fn length(&self) -> usize {
        self.byte_cursor.get_ref().len()
    }

    fn curr_byte_position(&self) -> u64 {
        self.byte_cursor.position()
    }

    fn get_curr_byte(&self) -> BitCursorResult<u8> {
        if self.curr_byte_position() >= self.length() as u64 {
            return Err(BitCursorError::BufferOverflow(format!(
                "tried to access index {}, but underlying data has size {}",
                self.curr_byte_position(),
                self.byte_cursor.get_ref().len()
            )));
        }
        Ok(self.byte_cursor.get_ref()[self.curr_byte_position() as usize])
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
            0 => self.byte_cursor.get_ref().len() - self.curr_byte_position() as usize,
            // If we're in the middle of a byte, don't count that as a full byte remaining
            // (Note that this is a somewhat arbitrary decision, but it's what makes more sense
            // to me as of now)
            _ => self.byte_cursor.get_ref().len() - self.curr_byte_position() as usize - 1,
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

impl BitSeek for BitCursor {
    fn seek(&mut self, pos: BitSeekFrom) -> BitCursorResult<(u64, u64)> {
        let new_total_bit_pos: i64 = match pos {
            BitSeekFrom::Start(bytes, bits) => {
                (bytes * 8 + bits) as i64
                //let total_bit_delta = bytes * 8 + bits;
                //((total_bit_delta / 8) as i64, (total_bit_delta % 8) as i64)
            },
            BitSeekFrom::Current(bytes, bits) => {
                let total_bit_delta = bytes * 8 + bits;
                let curr_total_bit_pos = self.curr_byte_position() * 8 + self.bit_pos as u64;
                curr_total_bit_pos as i64 + total_bit_delta
                //((new_total_bit_pos / 8) as i64, (new_total_bit_pos % 8) as i64)
            },
            BitSeekFrom::End(bytes, bits) => {
                let total_bit_delta = bytes * 8 + bits;
                let end_bit_pos = (self.length() * 8) as i64;
                end_bit_pos + total_bit_delta
                //((new_total_bit_pos / 8) as i64, (new_total_bit_pos % 8) as i64)
            },
        };
        let max_bit_pos = (self.length() * 8) as i64;
        if new_total_bit_pos > max_bit_pos {
            return Err(BitCursorError::BufferOverflow(format!(
                        "Tried to seek past end of buffer (position {} bits, but buffer has length {} bits)", new_total_bit_pos, max_bit_pos)));
        } else if new_total_bit_pos < 0 {
            return Err(BitCursorError::BufferUnderflow(format!(
                        "Tried to seek past beginning of buffer (position {} bits)", new_total_bit_pos)));
        }
        let new_byte_pos = new_total_bit_pos / 8;
        let new_bit_pos = new_total_bit_pos % 8;
        self.byte_cursor.set_position(new_byte_pos as u64);
        self.bit_pos = new_bit_pos as u8;
        Ok((new_byte_pos as u64, new_bit_pos as u64))
    }
}

#[cfg(test)]
mod tests {
    use ux::*;

    use crate::{bitcursor::BitCursor, bit_read::BitRead, bit_seek::BitSeekFrom, bit_seek::BitSeek};

    #[test]
    fn test_read() {
        let data: Vec<u8> = vec![0b11110000, 0b00001111];
        let mut cursor = BitCursor::new(data);

        assert_eq!(u4::bit_read(&mut cursor).unwrap(), u4::new(15));
        assert_eq!(u4::bit_read(&mut cursor).unwrap(), u4::new(0));
        assert_eq!(u2::bit_read(&mut cursor).unwrap(), u2::new(0));
        assert_eq!(u6::bit_read(&mut cursor).unwrap(), u6::new(15));
    }

    #[test]
    fn read_on_non_byte_boundary() {
        let data: Vec<u8> = vec![0b11110000, 0b00001111];
        let mut cursor = BitCursor::new(data);

        let _ = u4::bit_read(&mut cursor).unwrap();
        assert!(u8::bit_read(&mut cursor).is_err());
    }

    #[test]
    fn read_too_far() {
        let data: Vec<u8> = vec![0b11110000];
        let mut cursor = BitCursor::new(data);

        assert!(u16::bit_read(&mut cursor).is_err());
    }

    #[test]
    fn read_too_far_bits() {
        let data: Vec<u8> = vec![0b11110000];
        let mut cursor = BitCursor::new(data);

        let _ = u7::bit_read(&mut cursor);
        assert!(u3::bit_read(&mut cursor).is_err());
    }

    #[test]
    fn seek_from_start() {
        let data: Vec<u8> = vec![
            0b00001111,
            0b11110000,
            0b00001111,
            0b11110000,
            0b00001111,
        ];
        let mut cursor = BitCursor::new(data);
        // A valid seek
        assert_eq!((2, 2), cursor.seek(BitSeekFrom::Start(2, 2)).unwrap());
        assert_eq!(u4::new(0b0011), u4::bit_read(&mut cursor).unwrap());
        // A valid seek with bits > 1 byte
        assert_eq!((2, 2), cursor.seek(BitSeekFrom::Start(0, 18)).unwrap());
        assert_eq!(u4::new(0b0011), u4::bit_read(&mut cursor).unwrap());
        // A valid seek with bytes + overflowing bits
        assert_eq!((2, 2), cursor.seek(BitSeekFrom::Start(1, 10)).unwrap());
        assert_eq!(u4::new(0b0011), u4::bit_read(&mut cursor).unwrap());
        
        // Seek past the end
        assert!(cursor.seek(BitSeekFrom::Start(6, 0)).is_err());
        // Seek past the end using just bits
        assert!(cursor.seek(BitSeekFrom::Start(0, 48)).is_err());
        // Seek past the end with bytes + overflowing bits
        assert!(cursor.seek(BitSeekFrom::Start(2, 32)).is_err());
    }

    #[test]
    fn seek_from_current() {
        let data: Vec<u8> = vec![
            0b00001111,
            0b11110000,
            0b00001111,
            0b11110000,
            0b00001111,
        ];
        let mut cursor = BitCursor::new(data);
        // Start at some position
        let reset = |c: &mut BitCursor| c.seek(BitSeekFrom::Start(1, 2)).unwrap();
        reset(&mut cursor);
        
        // A valid seek
        assert_eq!((2, 4), cursor.seek(BitSeekFrom::Current(1, 2)).unwrap());
        assert_eq!(u4::new(0b1111), u4::bit_read(&mut cursor).unwrap());
        reset(&mut cursor);
        // A valid seek with bits > 1 byte
        assert_eq!((2, 4), cursor.seek(BitSeekFrom::Current(0, 10)).unwrap());
        assert_eq!(u4::new(0b1111), u4::bit_read(&mut cursor).unwrap());
        reset(&mut cursor);
        // A valid seek with bytes + overflowing bits
        assert_eq!((3, 4), cursor.seek(BitSeekFrom::Current(1, 10)).unwrap());
        assert_eq!(u4::new(0b0000), u4::bit_read(&mut cursor).unwrap());
        reset(&mut cursor);
        // A valid seek with bytes + bits which overflow with the current position
        assert_eq!((3, 1), cursor.seek(BitSeekFrom::Current(1, 7)).unwrap());
        assert_eq!(u4::new(0b1110), u4::bit_read(&mut cursor).unwrap());
        reset(&mut cursor);

        // A valid seek backwards
        assert_eq!((0, 2), cursor.seek(BitSeekFrom::Current(-1, 0)).unwrap());
        assert_eq!(u4::new(0b0011), u4::bit_read(&mut cursor).unwrap());
        reset(&mut cursor);

        // A valid seek backwards with bits > 1 byte
        assert_eq!((0, 0), cursor.seek(BitSeekFrom::Current(0, -10)).unwrap());
        assert_eq!(u4::new(0b0000), u4::bit_read(&mut cursor).unwrap());
        reset(&mut cursor);

        // A valid seek backwards with bytes + bits which overflow with the current position
        assert_eq!((0, 7), cursor.seek(BitSeekFrom::Current(0, -3)).unwrap());
        assert_eq!(u4::new(0b1111), u4::bit_read(&mut cursor).unwrap());

        // Seek past the end
        assert!(cursor.seek(BitSeekFrom::Current(6, 0)).is_err());
        reset(&mut cursor);
        // Seek past the end using just bits
        assert!(cursor.seek(BitSeekFrom::Current(0, 48)).is_err());
        reset(&mut cursor);
        // Seek past the end with bytes + overflowing bits
        assert!(cursor.seek(BitSeekFrom::Current(2, 32)).is_err());
        reset(&mut cursor);
        // Seek past the end with bytes + bits which overflow with the current position
        assert!(cursor.seek(BitSeekFrom::Current(3, 7)).is_err());

        // Seek past the beginning
        assert!(cursor.seek(BitSeekFrom::Current(0, -15)).is_err());
        reset(&mut cursor);
        assert!(cursor.seek(BitSeekFrom::Current(-1, -3)).is_err());
        reset(&mut cursor);
    }

    #[test]
    fn test_seek_from_end() {
        let data: Vec<u8> = vec![
            0b00001111,
            0b11110000,
            0b00001111,
            0b11110000,
            0b00001111,
        ];
        let mut cursor = BitCursor::new(data);

        // A valid seek
        assert_eq!((5, 0), cursor.seek(BitSeekFrom::End(0, 0)).unwrap());
        assert!(bool::bit_read(&mut cursor).is_err());

        // Seek back with bytes + bits
        assert_eq!((3, 6), cursor.seek(BitSeekFrom::End(-1, -2)).unwrap());
        assert_eq!(u4::new(0b0000), u4::bit_read(&mut cursor).unwrap());
        // Seek back with bytes + bits > 1 byte
        assert_eq!((2, 6), cursor.seek(BitSeekFrom::End(-1, -10)).unwrap());
        assert_eq!(u4::new(0b1111), u4::bit_read(&mut cursor).unwrap());

        // Seek back past the front
        assert!(cursor.seek(BitSeekFrom::End(-6, 0)).is_err());
    }
}
