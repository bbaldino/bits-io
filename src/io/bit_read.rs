use crate::prelude::*;

/// The BitRead trait allows for reading bits from a source.
pub trait BitRead: std::io::Read {
    /// Pull some bits from this source into the specified buffer, returning how many bits were
    /// read.
    fn read_bits<O: BitStore>(&mut self, dest: &mut BitSlice<O>) -> std::io::Result<usize>;

    /// Read the exact number of bits required to fill buf.
    fn read_bits_exact<O: BitStore>(&mut self, dest: &mut BitSlice<O>) -> std::io::Result<()> {
        // TODO: double-check this impl
        read_bits_exact_helper(self, dest)
    }
}

// TODO: do we need this helper?
fn read_bits_exact_helper<R: BitRead + ?Sized, O: BitStore>(
    this: &mut R,
    mut dest: &mut BitSlice<O>,
) -> std::io::Result<()> {
    while !dest.is_empty() {
        // Note: unlike std::io::Read, we don't have a special case for an 'interrupted'
        // error, since we don't have access to all the error data it uses.
        // TODO: look into if we can replicate the is_interrupted logic here somehow.
        match this.read_bits(dest) {
            Ok(0) => break,
            Ok(n) => dest = &mut dest[n..],
            Err(e) => return Err(e),
        }
    }
    if !dest.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "failed to fill whole buffer",
        ))
    } else {
        Ok(())
    }
}
