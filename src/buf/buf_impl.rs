use bytes::{Buf, BufMut};

use crate::prelude::{BitBufMut, BitsMut};

use super::{bit_buf::BitBuf, bits::Bits};

impl Buf for Bits {
    fn remaining(&self) -> usize {
        self.remaining_bytes()
    }

    fn chunk(&self) -> &[u8] {
        self.chunk_bytes()
    }

    fn advance(&mut self, cnt: usize) {
        self.advance_bytes(cnt);
    }
}

impl Buf for BitsMut {
    fn remaining(&self) -> usize {
        self.remaining_bytes()
    }

    fn chunk(&self) -> &[u8] {
        self.chunk_bytes()
    }

    fn advance(&mut self, cnt: usize) {
        self.advance_bytes(cnt);
    }
}

unsafe impl BufMut for BitsMut {
    fn remaining_mut(&self) -> usize {
        self.remaining_mut_bytes()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.advance_mut_bytes(cnt);
    }

    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        self.chunk_mut_bytes()
    }
}
