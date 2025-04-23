use bytes::BufMut;

use super::{bit_buf_mut::BitBufMut, bits_mut::BitsMut};
use crate::{buf::util::bytes_needed, prelude::*};

impl BitBufMut for BitsMut {
    // TODO: I think it's wrong to use the remaining bytes as the source of truth and base bits off
    // of that: calls like reserve() are in bits and, even though we need to round up wen reserving
    // the underlying bytes, the caller may not have requested all of that final byte as reserved.
    // we should change remaining_mut_bytes to be based off reamining_mut (in bits)
    fn remaining_mut_bytes(&self) -> usize {
        self.inner.remaining_mut()
    }

    fn chunk_mut(&mut self) -> &mut BitSlice {
        if self.capacity == self.bit_len {
            self.reserve(64);
        }
        self.spare_capacity_mut()
    }

    fn chunk_mut_bytes(&mut self) -> &mut bytes::buf::UninitSlice {
        assert!(self.byte_aligned_mut());
        if self.capacity == self.bit_len {
            self.reserve(64);
        }
        self.inner.chunk_mut()
    }

    fn advance_mut(&mut self, cnt: usize) {
        assert!(cnt <= self.remaining_mut(), "advance_mut past end");
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
