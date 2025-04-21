/// Returns the number of bytes needed to accommodate the given number of bits
pub(crate) fn bytes_needed(num_bits: usize) -> usize {
    num_bits.div_ceil(8)
}
