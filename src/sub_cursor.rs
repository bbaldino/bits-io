use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use bitvec::{order::Msb0, slice::BitSlice, vec::BitVec};

use crate::bit_cursor::BitCursor;

pub trait SubCursor<R> {
    type Output<'a>
    where
        Self: 'a;
    fn sub_cursor(&self, range: R) -> Self::Output<'_>;
}

macro_rules! impl_sub_cursor {
    ($range_type:ty,$type:ty) => {
        impl SubCursor<$range_type> for $type {
            type Output<'a>
                = BitCursor<&'a BitSlice<u8, Msb0>>
            where
                Self: 'a;

            fn sub_cursor(&self, range: $range_type) -> Self::Output<'_> {
                let slice = &self.remaining_slice()[range];
                BitCursor::new(slice)
            }
        }
    };
}

impl_sub_cursor!(Range<usize>, BitCursor<BitVec<u8, Msb0>>);
impl_sub_cursor!(RangeFrom<usize>, BitCursor<BitVec<u8, Msb0>>);
impl_sub_cursor!(RangeFull, BitCursor<BitVec<u8, Msb0>>);
impl_sub_cursor!(RangeInclusive<usize>, BitCursor<BitVec<u8, Msb0>>);
impl_sub_cursor!(RangeTo<usize>, BitCursor<BitVec<u8, Msb0>>);
impl_sub_cursor!(RangeToInclusive<usize>, BitCursor<BitVec<u8, Msb0>>);

impl_sub_cursor!(Range<usize>, BitCursor<&BitSlice<u8, Msb0>>);
impl_sub_cursor!(RangeFrom<usize>, BitCursor<&BitSlice<u8, Msb0>>);
impl_sub_cursor!(RangeFull, BitCursor<&BitSlice<u8, Msb0>>);
impl_sub_cursor!(RangeInclusive<usize>, BitCursor<&BitSlice<u8, Msb0>>);
impl_sub_cursor!(RangeTo<usize>, BitCursor<&BitSlice<u8, Msb0>>);
impl_sub_cursor!(RangeToInclusive<usize>, BitCursor<&BitSlice<u8, Msb0>>);
