use bits_io::prelude::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(sample_size = 10000)]
fn get_ux_byte_aligned() {
    let mut bits = Bits::from_static_bytes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let _ = bits.get_u8();
    let _ = bits.get_u16::<NetworkOrder>();
    let _ = bits.get_u24::<NetworkOrder>();
    let _ = bits.get_u32::<NetworkOrder>();
}

#[divan::bench(sample_size = 10000)]
fn get_ux_unaligned() {
    let mut bits = Bits::from_static_bytes(&[0, 1, 2, 3, 4]);

    let _ = bits.get_u3();
    let _ = bits.get_u6();
    let _ = bits.get_u9::<NetworkOrder>();
    let _ = bits.get_u12::<NetworkOrder>();
}

#[divan::bench(sample_size = 10000)]
fn put_ux_byte_aligned() {
    let mut bits_mut = BitsMut::new();

    let _ = bits_mut.put_u8(2);
    let _ = bits_mut.put_u16::<NetworkOrder>(4);
    let _ = bits_mut.put_u24::<NetworkOrder>(u24::new(6));
    let _ = bits_mut.put_u32::<NetworkOrder>(8);
}

#[divan::bench(sample_size = 10000)]
fn put_ux_unaligned() {
    let mut bits_mut = BitsMut::new();

    let _ = bits_mut.put_u3(u3::new(2));
    let _ = bits_mut.put_u6(u6::new(4));
    let _ = bits_mut.put_u9::<NetworkOrder>(u9::new(6));
    let _ = bits_mut.put_u12::<NetworkOrder>(u12::new(8));
}
