use bitvec::view::BitView;

use crate::prelude::*;

impl BitBuf for Bits {
    fn advance(&mut self, count: usize) {
        assert!(count <= self.remaining(), "advance past end of Bits");
        self.inc_start(count);
    }

    fn remaining(&self) -> usize {
        self.bit_len
    }

    fn chunk(&self) -> &BitSlice {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.bit_start % 8 == 0);
        assert!(self.bit_len % 8 == 0);

        let byte_start = self.bit_start / 8;

        &self.inner[byte_start..]
    }

    fn try_copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) -> std::io::Result<()> {
        if !self.byte_aligned() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buf beginning and end must both be byte-aligned",
            ));
        }
        if self.remaining_bytes() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }

        Ok(())
    }

    fn byte_aligned(&self) -> bool {
        self.bit_start % 8 == 0 && self.bit_len % 8 == 0
    }
}

impl BitBuf for BitsMut {
    fn advance(&mut self, count: usize) {
        assert!(count <= self.remaining(), "advance past end of BitsMut");
        self.bit_start += count;
        self.bit_len -= count;
        self.capacity -= count;
    }

    fn remaining(&self) -> usize {
        self.len()
    }

    fn chunk(&self) -> &BitSlice {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());

        let byte_start = self.bit_start / 8;

        &self.inner[byte_start..]
    }

    fn try_copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) -> std::io::Result<()> {
        if !self.byte_aligned() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buf beginning and end must both be byte-aligned",
            ));
        }
        if self.remaining_bytes() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }

        Ok(())
    }

    fn byte_aligned(&self) -> bool {
        self.bit_start % 8 == 0 && self.bit_len % 8 == 0
    }
}

impl BitBuf for &[u8] {
    fn advance(&mut self, count: usize) {
        if self.len() < count {
            panic!("Can't advance past the end of slice");
        }
        *self = &self[count..];
    }

    fn remaining(&self) -> usize {
        self.len() * 8
    }

    fn chunk(&self) -> &BitSlice {
        self[..].view_bits()
    }

    fn chunk_bytes(&self) -> &[u8] {
        self
    }

    fn try_copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) -> std::io::Result<()> {
        if self.len() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }

        Ok(())
    }

    fn byte_aligned(&self) -> bool {
        true
    }
}

// TODO: I think we're gonna get bit by not supporting BitSlice<O> here, but come back to that
// later--hopefully we don't need a generic on the trait
// impl BitBuf for &BitSlice {
impl BitBuf for &BitSlice {
    fn advance(&mut self, count: usize) {
        if self.len() < count {
            panic!("Can't advance past end of BitSlice");
        }
        *self = &self[count..];
    }

    fn remaining(&self) -> usize {
        self.len()
    }

    fn chunk(&self) -> &BitSlice {
        self
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());
        let bitvec::domain::Domain::Region { body, .. } = self.domain() else {
            unreachable!("Verified by the assert above");
        };

        body
    }

    fn try_copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) -> std::io::Result<()> {
        if !self.byte_aligned() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buf beginning and end must both be byte-aligned",
            ));
        }
        if self.remaining_bytes() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }

        Ok(())
    }

    fn byte_aligned(&self) -> bool {
        matches!(
            self.domain(),
            bitvec::domain::Domain::Region {
                head: None,
                tail: None,
                ..
            }
        )
    }
}

impl<T: AsRef<BitSlice>> BitBuf for BitCursor<T> {
    fn advance(&mut self, count: usize) {
        let len = self.get_ref().as_ref().len();
        let pos = self.position();

        let max_count = len.saturating_sub(pos as usize);
        if count > max_count {
            panic!("Can't advance beyond end of buffer");
        }
        self.set_position(pos + count as u64);
    }

    fn remaining(&self) -> usize {
        self.get_ref()
            .as_ref()
            .len()
            .saturating_sub(self.position() as usize)
    }

    fn chunk(&self) -> &BitSlice {
        let slice = self.get_ref().as_ref();
        let start = slice.len().min(self.position() as usize);
        &slice[start..]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());
        let bitslice = self.get_ref().as_ref();
        let bitvec::domain::Domain::Region { body, .. } = bitslice.domain() else {
            unreachable!("Verified by the assert above");
        };

        body
    }

    fn try_copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) -> std::io::Result<()> {
        if !self.byte_aligned() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buf beginning and end must both be byte-aligned",
            ));
        }
        if self.remaining_bytes() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }

        Ok(())
    }

    fn byte_aligned(&self) -> bool {
        // TODO: helper func on BitSlice?
        // TODO: would a slice of a single by be represented by `Region` or `Enclave`? If Enclave,
        // we need to support that as well
        matches!(
            self.get_ref().as_ref().domain(),
            bitvec::domain::Domain::Region {
                head: None,
                tail: None,
                ..
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_buf_bits_advance() {
        let mut bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        bits.advance(4);
        assert_eq!(bits.len(), 4);
        assert_eq!(bits.chunk(), bits![0, 0, 0, 0]);
    }

    #[test]
    fn test_bit_buf_bits_mut_advance() {
        let mut bits_mut = BitsMut::zeroed(16);
        bits_mut.advance(8);
        assert_eq!(bits_mut.len(), 8);
    }

    #[test]
    fn test_bits_copy_to_slice() {
        let mut bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        let dest = bits![mut 0; 4];
        bits.copy_to_slice(dest);
        assert_eq!(dest, bits![1, 1, 1, 1,]);

        bits.copy_to_slice(dest);
        assert_eq!(dest, bits![0, 0, 0, 0]);
    }

    #[test]
    fn test_chunk_bytes() {
        {
            let bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

            let chunk_bytes = bits.chunk_bytes();
            assert_eq!(chunk_bytes.len(), 1);
            assert_eq!(chunk_bytes[0], 0b11110000);
        }
        {
            let mut bits = Bits::copy_from_slice(bits![
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0
            ]);
            bits.advance(8);
            let chunk_bytes = bits.chunk_bytes();
            assert_eq!(chunk_bytes.len(), 2);
            assert_eq!(chunk_bytes, [0b11111111, 0b10101010]);
        }
    }

    #[test]
    fn test_copy_to_slice_bytes() {
        let mut dest = [0; 4];

        let mut bits = Bits::from_owner_bytes([42, 43, 44, 45]);

        bits.copy_to_slice_bytes(&mut dest);
        assert_eq!(dest, [42, 43, 44, 45]);
    }

    #[test]
    fn test_bitslice_bitbuf() {
        let mut bits = bits![1, 0, 1, 0, 1, 0];
        assert_eq!(6, bits.remaining());
        bits.advance(3);
        assert_eq!(3, bits.remaining());
    }
}
