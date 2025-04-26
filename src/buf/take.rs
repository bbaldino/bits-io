use crate::prelude::*;

/// A `BitBuf` adaptor which limits the bits read from an underlying buffer.
pub struct Take<T> {
    inner: T,
    limit: usize,
}

impl<T> Take<T> {
    pub fn new(inner: T, limit: usize) -> Take<T> {
        Self { inner, limit }
    }
}

impl<T> Take<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }
}

impl<T: BitBuf> BitBuf for Take<T> {
    fn advance_bits(&mut self, count: usize) {
        assert!(count <= self.limit);
        self.inner.advance_bits(count);
        self.limit -= count;
    }

    fn remaining_bits(&self) -> usize {
        std::cmp::min(self.inner.remaining_bits(), self.limit)
    }

    fn chunk_bits(&self) -> &BitSlice {
        let chunk = self.inner.chunk_bits();
        let end = std::cmp::min(chunk.len(), self.limit);
        &chunk[..end]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());
        let chunk = self.inner.chunk_bytes();
        let byte_limit = self.limit / 8;
        let end = std::cmp::min(chunk.len(), byte_limit);
        &chunk[..end]
    }

    fn byte_aligned(&self) -> bool {
        // TODO: need to verify that this is right/it's possible to reliably implement this for
        // BitTake
        self.inner.byte_aligned() && self.limit % 8 == 0
    }
}
