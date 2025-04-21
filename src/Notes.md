`std::io::Cursor`:

```rust
struct Cursor<T> {
  inner: T,
  pos: u64 // 'byte' level position
}
```

`BitCursor`:

```rust
struct BitCursor<T> {
  inner: T,
  pos: u64 // 'bit' level position
}
```

The APIs will be exactly the same, just with `BitCursor`'s applying at a bit-level where it makes sense:

`get_mut`, `get_ref`, `into_inner`, `new` all work exactly the same

`position` and `set_position` will use bit-indices as opposed to byte for `BitCursor`

`split` and `split_mut` will return `(&BitSlice<u8, Msb0>, &BitSlice<u8, Msb0>)` for `BitCursor` instead of `(&[u8], &[u8])`.

Implementing split/split_mut:

At first I did blanket implementations of `BitCursor<T>` for any T that was
`AsRef<BitSlice<u8, Msb0>>`.  For the most part this worked well, but it meant
that `BitCursor<&[u8]>` no longer worked since it didn't meet that requirement.
Then I discovered `bitvec` defines an `AsBits` trait that it claims is the
equivalent for `AsRef`, so I tried that instead, but that results in another
problem: for some reason `BitVec`` doesn't implement`AsBits`, it just provides
its own`as_bitslice` method.  From what I can tell, this means there's no
single blanket trait boundary I can use to provide an implementation that works
for everything.

For now I think I'll stick with AsRef, but hopefully can figure something
better out long term.
--> Actually, it looks like there isn't an impl for AsRef<BitSlice<..>> for
Vec<u8>, so maybe AsBits is better for now and I can manually do some
implementations for BitCursor<BitVec<..>>? Will give that a try.
--> **Actually** this is flawed: it looks like AsRef _is_ what I want, and, in
order to make things like Vec<u8> work I'd just need to do some extra stuff
with a helper trait.  But, maybe this makes sense since Vec<u8> and &[u8] are
explicitly byte-level abstractions and need to be 'adapted'?

----> The way forward here ended up being adding a separate helper trait that
can bridge all the gaps and using that

## BitBuf::advance vs BitBufMut::advance_mut

'advance' on a Buf/BitBuf is "advancing" the start of the view.  'advance_mut' on BitBufMut/BufMut is advancing the _write position_ of the view.

## Managing BitStore compatibility

Originally the read_uXX and write_uXX methods all copied the data over
bit-by-bit.  This felt a bit wasteful and inefficient and it made that code
very "special" compared to the other APIs.  `bytes` does this by converting all
types to slices and then putting a slice, so I wanted to do something similar
but with BitSlices.  This means we'd have a trait like this:

```rust
pub trait ByteOrder {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32);
    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32;
}
```

Which would then get implemented for `BigEndian` and `LittleEndian`.  Ideally we'd have something like:

```rust
impl ByteOrder for BigEndian {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32) {
        assert!(bits.len() <= 32, "cannot write more than 32 bits");
        let mut value_be_bytes = value.to_be_bytes();
        let value_slice = &mut value_be_bytes.view_bits_mut::<Msb0>()[(32 - bits.len())..];
        bits.copy_from_slice(value_slice);
    }

    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32 {
        assert!(bits.len() <= 32, "cannot read more than 32 bits into a u32");

        let mut buf = [0u8; 4];
        let dest = &mut BitSlice::from_slice_mut(&mut buf)[..bits.len()];
        dest.copy_from_slice(bits);
        u32::from_be_bytes(buf)
    }
}
```

The problem is that `BitSlice::copy_from_slice` requires that both slices have
the exact same `BitStore` type.  Even though bits-io _always_ uses `u8`, bitvec
defines 'safe aliases' for each storage type that can be used when two mutable
slices overlap the same bytes in their back storage.  In `u8`'s case that alias
is `BitSafeU8`.  So that means if one of `src` or `dest` is `u8` and the other
is `BitSafeU8` then we can't copy directly between them.  In each of the
read/write scenarios, we have flexibility over one of the slices, but not the
other:

When writing, we control the source's bitslice: we're converting a u32 into
bytes and then can always create a `BitSlice<u8>` from that, but we don't
control the slice we're writing _to_: that comes from the caller and could be
either `BitSlice<u8>` or `BitSlice<BitSafeU8`>.

When reading, we control the destination's bitslice: we create a buffer to read
the value into and can always create a `BitSlice<u8>` from that, but we don't
control the source we're reading _from_: that comes from the caller and could
be either `BitSlice<u8>` or `BitSlice<BitSafeU8>`.

If, in both cases, the source and destination are `BitSlice<u8>`, then we can
use `BitSlice::copy_from_slice`. But, if they differ, we need some special
logic.  This means we need to know one case from the other, so we define some
traits to enable this:

For writing:

```rust
pub trait CopyFromBitSlice {
    fn copy_from_slice(dest: &mut BitSlice<Self>, src: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized;
}
```

For reading:

```rust
pub trait CopyToBitSlice {
    fn copy_to_slice(src: &BitSlice<Self>, dest: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized;
}
```

Then we can have specific implementations for the `BitSafeU8` and `u8` cases.
When writing from `BitSlice<u8>` to `BitSlice<BitSafeU8>`, we can use
`BitSlice::split_at_unchecked_mut` to transform the source from a
`BitSlice<u8>` into a `BitSlice<BitSafeU8>`.  When reading from a
`BitSlice<BitSafeU8>` to a `BitSlice<u8>`, we can use `split_at_unchecked_mut`
again to transform the _dest_ from a `BitSlice<u8>` to a `BitSlice<BitSafeU8>`.
This subtle difference in the source vs dest transformation is why we need two
traits.

Another required piece for this to work is to seal our wrapper of the
`BitStore` trait so that the compiler knows there are only two possible
implementations (`u8` and `BitSafeU8`), that way we can take a generic bounded
by the `BitStore` trait (`<O: BitStore>`) and call `O::copy_to_slice` and
`O::copy_from_slice` after providing implementations of `CopyFromBitSlice` and
`CopyToBitSlice` for both `u8` and `BitSafeU8` because that then covers all
possible values of `O`.

--> I later found out that bitvec has store_{be|le} and load_{be|le} methods
that take care of all this logic.  They do have the caveat that little-endian
operations don't work well when the slice isn't aligned with the underlying
storage.  I _think_ this might be a bug and filed [this
issue](https://github.com/ferrilab/bitvec/issues/294).  For now will deal with
it as I don't think it'll be a common case for stuff I'm working on, but would
be nice if it worked as expected (though for all I know it could be by-design).

I ran into another case where I was going to need to handle this again and I
finally stumbled across `clone_from_bitslice` which works when BitStore's
differ: when they're the same it uses `copy_from_bitslice` and when it can't it
goes through and copies on its on (what i was doing manually).

Yet another case this came up was in the BitWrite impl for &mut BitSlice.  I
noticed that the std::io::Write impl for &mut [T] had the reference update
itself to reflect the written bytes, but my impl  for BitWrite didn't.  I
looked at the std::io::Write impl and it leveraged split_at_mut so I tried to
do the same, but then I ran into the aliasing problem again:

when the type was &mut BitSlice<u8>, calling split_a_mut gave me a &mut
BitSlice<BitSafeU8>, so I couldn't re-assign that to self.  bitvec has quite a
few ways to unalias a type's storage when you know it's safe (there's
BitSlice::unalias_mut and BitSlice::split_at_unchecked_mut_noalias which really
would've been ideal) but none are public.  I also tried a more manual casting
approach:

```rust
/// SAFETY:
/// - `bits` must have originated from a `BitSlice<u8, Msb0>`
/// - `bits` must be disjoint from any other active reference
// Note: this almost worked, but BitSpan isn't public at all.
pub unsafe fn strip_aliasing_u8<'a>(bits: &mut BitSlice<BitSafeU8>) -> &'a mut BitSlice<u8> {
    let bitptr = bits.as_mut_bitptr(); // BitPtr<Mut, BitSafeU8, Msb0>
    let len = bits.len();

    // Get the underlying storage pointer and bit offset
    let (raw, bit_offset) = bitptr.raw_parts();

    // Cast the raw element pointer from BitSafeU8 to u8
    let unaliased_raw = raw.cast::<u8>();

    // SAFETY: caller guarantees this was originally BitSlice<u8, Msb0>
    let ptr_u8 = BitPtr::<_, u8, Msb0>::new_unchecked(unaliased_raw, bit_offset);

    // SAFETY: BitSpan::new_unchecked is equivalent to reconstructing a bitslice view
    let span = BitSpan::new_unchecked(ptr_u8, len);

    BitSlice::from_bitptr_mut(ptr_u8, len)
}
```

but `BitSpan` isn't public at all.  Finally I just tried doing the impl without
`split_at_mut` and just writing to self directly and that seems to work, so
will go with that for now.
