# BitCursor

BitCursor is similar to std::io::Cursor, but allows reading various amounts of bits from a given buffer in addition
to byte-sized chunks.  It's built on top of the [ux](https://crates.io/crates/ux) crate for types.

### Examples
```rust
let data: Vec<u8> = vec![0b11110000, 0b00001111];
let mut cursor = BitCursor::new(data);

assert_eq!(cursor.bit_read::<u4>().unwrap(), 15);
assert_eq!(cursor.bit_read::<u4>().unwrap(), 0);
assert_eq!(cursor.bit_read::<u2>().unwrap(), 0);
assert_eq!(cursor.bit_read::<u6>().unwrap(), 15);
```

It also supports seeking via `BitSeek`, which is similar to `Seek`:
```rust
let data: Vec<u8> = vec![0b11110000, 0b00001111];
let mut cursor = BitCursor::new(data);

assert_eq!((1, 0), cursor.seek(BitSeekFrom::Start(1, 0)).unwrap());
assert_eq!(cursor.bit_read::<u4>().unwrap(), 0);

assert_eq!((0, 3), cursor.seek(BitSeekFrom::Current(0, -6)).unwrap());
assert_eq!(cursor.bit_read::<u4>().unwrap(), 0b1100);


assert_eq!((0, 3), cursor.seek(BitSeekFrom::End(0, -4)).unwrap());
assert_eq!(cursor.bit_read::<u4>().unwrap(), 0b1111);
```


