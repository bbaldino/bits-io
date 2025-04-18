use bitvec::{
    order::Msb0,
    ptr::{BitPtr, BitSpanError, Mut},
};

use crate::buf::byte_order::{CopyFromBitSlice, CopyToBitSlice};

mod sealed {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for bitvec::access::BitSafeU8 {}
}

// We use sealed above to ensure that these are the only two possible impls of our `BitStore`
// wrapper.  This makes the logic in `byte_order` more straightforward because the compiler knows
// that the two specialized implementations of `WriteU32ToBits` are the _only_ possible values.
// TODO:: I'm not sure if this trait needs to be public or not.  If it does, then it might be good
// to look at removing the write_u32_to_bits method to another internal trait, can't decide if it
// feels good or bad as part of the public API
pub trait BitStore:
    bitvec::store::BitStore + sealed::Sealed + CopyFromBitSlice + CopyToBitSlice
{
}

impl BitStore for u8 {}
impl BitStore for bitvec::access::BitSafeU8 {}

pub type BitSlice<O = u8> = bitvec::slice::BitSlice<O, bitvec::order::Msb0>;

pub type BitVec = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;

/// Create a mutable BitSlice from raw parts.  This is a wrapper of a bitvec function and just
/// hardcodes the storage to u8 and order to Msb0
///
/// # Errors
///
/// This function will return an error if len is too large, or if data is ill-formed. This fails if
/// it has an error while encoding the &mut BitSlice, and succeeds if it is able to produce a
/// correctly encoded value.
///
/// # Safety
/// See bitvec::slice::from_raw_parts_mut
///
/// .
#[inline]
pub unsafe fn from_raw_parts_mut<'a>(
    data: BitPtr<Mut, u8, Msb0>,
    len: usize,
) -> Result<&'a mut BitSlice, BitSpanError<u8>> {
    bitvec::slice::from_raw_parts_mut::<u8, Msb0>(data, len)
}

// Having to include the 'use' statements in the nested calls below is pretty annoying, but the
// bitvec macros generate some annoying linter complaints when a fully-qualified path is used.

#[macro_export]
macro_rules! bits {
    (mut $($bit:expr),* $(,)?) => {{
        use $crate::internal::bitvec::order::Msb0;
        ($crate::internal::bitvec::bits![mut u8, Msb0; $($bit),*])
    }};
    ($($bit:expr),* $(,)?) => {{
        use $crate::internal::bitvec::order::Msb0;
        ($crate::internal::bitvec::bits![u8, Msb0; $($bit),*])
    }};
}

#[macro_export]
macro_rules! bitvec {
    // Repeat value form: bitvec_u8![value; len]
    ($value:expr; $len:expr) => {{
        use $crate::internal::bitvec::order::Msb0;
        ($crate::internal::bitvec::bitvec!(u8, Msb0; $value; $len))
    }};
    // List of explicit bits: bitvec_u8![1, 0, 1, 1]
    ($($bit:expr),* $(,)?) => {
        use $crate::internal::bitvec::order::Msb0;
        ($crate::internal::bitvec::bitvec!(u8, Msb0; $($bit),*))
    };
}
