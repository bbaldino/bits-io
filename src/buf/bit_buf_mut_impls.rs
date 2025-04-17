use bytes::BufMut;

use super::{bit_buf_mut::BitBufMut, bits_mut::BitsMut};
use crate::prelude::*;

impl BitBufMut for BitsMut {
    fn remaining_mut_bytes(&self) -> usize {
        self.inner.remaining_mut()
    }

    fn chunk_mut(&mut self) -> &mut BitSlice {
        if self.capacity == self.bit_len {
            self.reserve(64);
        }
        self.spare_capacity_mut()
    }

    // TODO: chunk_mut_bytes might be nice, but I think it could only work if we were on a byte
    // boundary

    fn advance_mut(&mut self, cnt: usize) {
        assert!(cnt <= self.remaining_mut(), "advance_mut past end");
        self.bit_len += cnt;
    }
}
