use std::io::SeekFrom;

pub trait BitSeek {
    fn bit_seek(&mut self, pos: SeekFrom) -> std::io::Result<u64>;
}
