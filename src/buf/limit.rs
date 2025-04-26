use crate::prelude::BitBufMut;

pub struct Limit<T> {
    inner: T,
    limit: usize,
}

impl<T> Limit<T> {
    pub fn new(inner: T, limit: usize) -> Self {
        Self { inner, limit }
    }

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

impl<T: BitBufMut> BitBufMut for Limit<T> {
    fn advance_mut_bits(&mut self, count: usize) {
        assert!(count <= self.limit);
        self.inner.advance_mut_bits(count);
        self.limit -= count;
    }

    fn chunk_mut_bits(&mut self) -> &mut crate::prelude::BitSlice {
        let chunk = self.inner.chunk_mut_bits();
        let end = std::cmp::min(chunk.len(), self.limit);
        &mut chunk[..end]
    }

    fn chunk_mut_bytes(&mut self) -> &mut bytes::buf::UninitSlice {
        assert!(self.byte_aligned_mut());
        let chunk = self.inner.chunk_mut_bytes();
        let byte_limit = self.limit / 8;
        let end = std::cmp::min(chunk.len(), byte_limit);
        &mut chunk[..end]
    }

    fn remaining_mut_bits(&self) -> usize {
        std::cmp::min(self.inner.remaining_mut_bits(), self.limit)
    }

    fn byte_aligned_mut(&self) -> bool {
        self.inner.byte_aligned_mut() && self.limit % 8 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit() {
        let data = &mut [0u8; 10];

        let lim = data.limit_bytes(2);
        assert_eq!(lim.remaining_mut_bytes(), 2);
        let lim = data.limit_bits(16);
        assert_eq!(lim.remaining_mut_bits(), 16);
    }
}
