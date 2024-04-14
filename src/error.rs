use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Tried to take a slice from array of length {len} from {slice_start} to {slice_end}")]
    SliceOutOfRange {
        len: usize,
        slice_start: u64,
        slice_end: u64,
    },
}
