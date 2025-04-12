# bits-io

Bit-level IO utilities for Rust, inspired by std::io patterns but designed for
working with bits instead of bytes.

Built on top of [`bitvec`](https://docs.rs/bitvec) for bit-level abstractions
and [nsw-types](https://github.com/bbaldino/nsw-types) for non-standard-width
types.

## Overview

`bits-io` provides:

- `BitCursor` - Like `std::io::Cursor`, but for bits.
- `BitRead` / `BitWrite` - Like `std::io::Read` and `Write`, but for bits.
- `BitReadExts` / `BitWriteExts` - Extensions for reading/writing `u1` through
`u32` values.
- `ByteOrder` - Support for BigEndian and LittleEndian bit ordering.
- `BitSlice` - A type alias for `&BitSlice<u8, Msb0>` (all APIs here use u8
storage and Msb0 ordering).
- Helpful macros for defining bits and bitvecs with u8 storage and Msb0 order.

## BitCursor

Mimics `std::io::Cursor` but tracks a bit-level position instead of a
byte-level position.  In addition to the standard `Seek` implementation which
allows seeking by a number of bytes, it also provides `BitSeek` which allows
seeking by a number of bits.

## `BitRead`

`BitRead` mimics the
[`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait, but
its API is defined in terms of reading into "bit slices" instead of `u8` slices
(`&[u8]`) like `std::io::Read`.  It leverages the `BitSlice` type defined in
the [bitvec](https://docs.rs/bitvec/latest/bitvec/) crate.

## `BitWrite`

`BitWrite` mimics the
[`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait,
but its API is defined in terms of writing from "bit slices" instead of `u8`
slices (`&[u8]`).  It leverages the `BitSlice` type defined in the
[bitvec](https://docs.rs/bitvec/latest/bitvec/) crate.

## Examples

```rust
let data: Vec<u8> = vec![0b11100000, 0b11101111];
let mut cursor = BitCursor::from_vec(data);

// Read any non-standard-width type from the cursor
let u3_val = cursor.read_u3().unwrap();
assert_eq!(u3_val, nsw_types::u3::new(0b111));
// Sizes larger than 8 bits require a byte order argument
let u13_val = cursor
    .read_u13::<crate::byte_order::NetworkOrder>()
    .unwrap();
assert_eq!(u13_val, nsw_types::u13::new(0b0000011101111));
```
