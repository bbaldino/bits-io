use std::ops::RangeBounds;

use ux::u1;

/// Get the `bit_index` bit of `byte` as a u1
pub(crate) fn get_bit(byte: u8, bit_index: u64) -> u1 {
    let mask = match bit_index {
        0 => 0b10000000,
        1 => 0b01000000,
        2 => 0b00100000,
        3 => 0b00010000,
        4 => 0b00001000,
        5 => 0b00000100,
        6 => 0b00000010,
        7 => 0b00000001,
        _ => unreachable!(),
    };
    let result = byte & mask;
    u1::new(result >> (7 - bit_index))
}

/// Set the `bit_index` bit of `byte` to `value`
pub(crate) fn set_bit(byte: &mut u8, bit_index: u64, value: u1) {
    // Mask out bit_index
    // assign value to u8, shift it to the index
    // or value with byte
    let mask = match bit_index {
        0 => 0b01111111,
        1 => 0b10111111,
        2 => 0b11011111,
        3 => 0b11101111,
        4 => 0b11110111,
        5 => 0b11111011,
        6 => 0b11111101,
        7 => 0b11111110,
        _ => unreachable!(),
    };
    *byte &= mask;
    let mut value: u8 = value.into();
    value <<= 7 - bit_index;
    *byte |= value;
}

/// Get the start and end bit indices from the given |range|, where |len| represents the length of
/// the item being indexed.  The returned start_bit_index is inclusive and end_bit_index is
/// exclusive.
pub(crate) fn get_start_end_bit_index_from_range<T: RangeBounds<u64>>(
    range: &T,
    len: usize,
) -> (u64, u64) {
    let start_bit_index = match range.start_bound() {
        std::ops::Bound::Included(&s) => s,
        std::ops::Bound::Excluded(s) => s + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end_bit_index = match range.end_bound() {
        std::ops::Bound::Included(s) => s + 1,
        std::ops::Bound::Excluded(&s) => s,
        // The end bit index is exclusive, so to handle the case where the length is 0 we make sure
        // it's always at least '1'.
        std::ops::Bound::Unbounded => std::cmp::max(len, 1) as u64,
    };
    (start_bit_index, end_bit_index)
}
