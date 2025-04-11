# bits-io

bits-io provides types which mimic those in `std::io` except which operate on
the bit level instead of the byte level.

## BitCursor

Mimics `std::io::Cursor` but tracks a bit-level position instead of a
byte-level position.  In addition to the standard `Seek` implementation which
allows seeking by a number of bytes, it also provides `BitSeek` which allows
seeking by a number of bits.

## `BitRead`

`BitRead` mimics the
[`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait, but
its API is defined in terms of reading from "bit slices" instead of `u8` slices
(`&[u8]`) like `std::io::Read`.  It leverages the `BitSlice` type defined in
the [bitvec](https://docs.rs/bitvec/latest/bitvec/) crate.

## `BitWrite`

`BitWrite` mimics the
[`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait,
but its API is defined in terms of reading from "bit slices" instead of `u8`
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
