use bitvec::view::BitView;
use bytes::BufMut;

use super::{bit_buf_mut::BitBufMut, bits_mut::BitsMut};
use crate::{buf::util::bytes_needed, prelude::*};

impl BitBufMut for BitsMut {
    fn remaining_mut_bits(&self) -> usize {
        usize::MAX - self.bit_len
    }

    fn chunk_mut_bits(&mut self) -> &mut BitSlice {
        if self.capacity == self.bit_len {
            self.reserve_bits(64);
        }
        self.spare_capacity_mut()
    }

    fn chunk_mut_bytes(&mut self) -> &mut bytes::buf::UninitSlice {
        assert!(self.byte_aligned_mut());
        if self.capacity == self.bit_len {
            self.reserve_bits(64);
        }
        self.inner.chunk_mut()
    }

    fn advance_mut_bits(&mut self, cnt: usize) {
        assert!(cnt <= self.remaining_mut_bits(), "advance_mut past end");
        let current_byte_len = bytes_needed(self.bit_len);
        self.bit_len += cnt;
        let new_byte_len = bytes_needed(self.bit_len);
        // Every time we cross into a new byte, we need to advance the underlying instance's
        // position as well.
        if new_byte_len > current_byte_len {
            unsafe {
                self.inner.advance_mut(new_byte_len - current_byte_len);
            }
        }
    }

    fn byte_aligned_mut(&self) -> bool {
        self.bit_start % 8 == 0 && self.bit_len % 8 == 0
    }
}

impl BitBufMut for &mut [u8] {
    fn advance_mut_bits(&mut self, count: usize) {
        assert!(
            count <= self.remaining_mut_bits(),
            "advance_mut_bits past end"
        );
        let byte_count = bytes_needed(count);

        let (_, b) = std::mem::take(self).split_at_mut(byte_count);
        *self = b;
    }

    fn chunk_mut_bits(&mut self) -> &mut BitSlice {
        self.view_bits_mut()
    }

    fn chunk_mut_bytes(&mut self) -> &mut bytes::buf::UninitSlice {
        bytes::buf::UninitSlice::new(self)
    }

    fn remaining_mut_bits(&self) -> usize {
        self.len() * 8
    }

    fn byte_aligned_mut(&self) -> bool {
        true
    }
}

impl BitBufMut for &mut BitSlice {
    fn advance_mut_bits(&mut self, count: usize) {
        assert!(count <= self.len(), "advance_mut_bits past end");
        *self = &mut std::mem::take(self)[count..];
    }

    fn chunk_mut_bits(&mut self) -> &mut BitSlice {
        self
    }

    fn chunk_mut_bytes(&mut self) -> &mut bytes::buf::UninitSlice {
        assert!(self.byte_aligned_mut());
        let bitvec::domain::Domain::Region { body, .. } = self.domain_mut() else {
            unreachable!("Verified by the assert above");
        };
        bytes::buf::UninitSlice::new(body)
    }

    fn remaining_mut_bits(&self) -> usize {
        self.len()
    }

    fn byte_aligned_mut(&self) -> bool {
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
