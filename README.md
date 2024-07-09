# BitCursor

BitCursor is similar to std::io::Cursor, but allows reading various amounts of bits from a given buffer in addition
to byte-sized chunks.  It's built on top of the [nsw_types](https://crates.io/crates/nsw-types) crate for types and leverages 
[bitvec](https://docs.rs/bitvec/latest/bitvec/) to provide a more complete implementation.

# Examples

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

# Design

## Traits

### `BitRead`
`BitRead` is analogus to the [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait, but its API is defined in terms of reading from "bit slices" instead of `u8` slices (`&[u8]`) like `std::io::Read`.


### `BitWrite`
`BitWrite` is analogus to the [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait, but its API is defined in terms of writing to "bit slices" instead of `u8` slices (`&[u8]`) like `std::io::Write`.


## Types

### `BitCursor`
`BitCursor` is analogous to the [`std::io::Cursor`](https://doc.rust-lang.org/std/io/struct.Cursor.html) type, but its API is defined in terms of bits instead of bytes.
