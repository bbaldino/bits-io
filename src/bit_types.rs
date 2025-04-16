use bitvec::{
    order::Msb0,
    ptr::{BitPtr, BitSpanError, Mut},
};

pub trait BitStore: bitvec::store::BitStore {}

impl BitStore for u8 {}
impl BitStore for bitvec::access::BitSafeU8 {}

pub type BitSlice<O = u8> = bitvec::slice::BitSlice<O, bitvec::order::Msb0>;

pub type BitVec = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;

/// Create a mutable BitSlice from raw parts.  This is a wrapper of a bitvec function and just
/// hardcodes the storage to u8 and order to Msb0
///
/// # Errors
///
/// This function will return an error if .
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

#[macro_export]
macro_rules! bits {
    (mut $($bit:expr),* $(,)?) => {
        $crate::internal::bitvec::bits![mut u8, $crate::internal::bitvec::order::Msb0; $($bit),*]
    };
    ($($bit:expr),* $(,)?) => {
        $crate::internal::bitvec::bits![u8, $crate::internal::bitvec::order::Msb0; $($bit),*]
    };
}

#[macro_export]
macro_rules! bitvec {
    // Repeat value form: bitvec_u8![value; len]
    ($value:expr; $len:expr) => {
        $crate::internal::bitvec::bitvec!(u8, $crate::internal::bitvec::order::Msb0; $value; $len)
    };
    // List of explicit bits: bitvec_u8![1, 0, 1, 1]
    ($($bit:expr),* $(,)?) => {
        $crate::internal::bitvec::bitvec!(u8, $crate::internal::bitvec::order::Msb0; $($bit),*)
    };

}
