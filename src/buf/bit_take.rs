use crate::prelude::*;

// TODO: Probably just rename this to 'Take' since we'll use it for both?

/// A `BitBuf` adaptor which limits the bits read from an underlying buffer.
pub struct BitTake<T> {
    inner: T,
    limit: usize,
}

impl<T> BitTake<T> {
    pub fn new(inner: T, limit: usize) -> BitTake<T> {
        Self { inner, limit }
    }
}

impl<T> BitTake<T> {
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

impl<T: BitBuf> BitBuf for BitTake<T> {
    fn advance(&mut self, count: usize) {
        assert!(count <= self.limit);
        self.inner.advance(count);
        self.limit -= count;
    }

    fn remaining(&self) -> usize {
        std::cmp::min(self.inner.remaining(), self.limit)
    }

    fn chunk(&self) -> &BitSlice {
        let chunk = self.inner.chunk();
        &chunk[..std::cmp::min(chunk.len(), self.limit)]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());
        let chunk = self.inner.chunk_bytes();
        let byte_limit = self.limit / 8;
        &chunk[..std::cmp::min(chunk.len(), byte_limit)]
    }

    fn byte_aligned(&self) -> bool {
        // TODO: need to verify that this is right/it's possible to reliably implement this for
        // BitTake
        self.inner.byte_aligned() && self.limit % 8 == 0
    }
}
