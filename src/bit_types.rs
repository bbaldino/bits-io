pub trait BitStore: bitvec::store::BitStore {}

impl BitStore for u8 {}
impl BitStore for bitvec::access::BitSafeU8 {}

pub type BitSlice<O = u8> = bitvec::slice::BitSlice<O, bitvec::order::Msb0>;

pub type BitVec = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;

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
