use crate::bitcursor::BitCursorResult;

pub enum BitSeekFrom {
    /// Seek a given number of (bytes, bits) forwards from the start of the buffer.
    Start(u64, u64),
    /// Seek a given number of (bytes, bits) in either direction from the current position.
    Current(i64, i64),
    /// Seek a given number of (bytes, bits) backwards from the end of the buffer.
    End(i64, i64),
}

pub trait BitSeek {
    fn seek(&mut self, pos: BitSeekFrom) -> BitCursorResult<(u64, u64)>;

    fn rewind(&mut self) -> BitCursorResult<()> {
        self.seek(BitSeekFrom::Start(0, 0)).map(|_| ())
    }
}
