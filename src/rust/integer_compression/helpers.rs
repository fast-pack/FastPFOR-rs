/// Finds the greatest multiple of `factor` that is less than or equal to `value`.
pub fn greatest_multiple(value: u32, factor: u32) -> u32 {
    value - value % factor
}

/// Returns the number of bits needed to represent `i`.
/// Returns 0 for input 0.
pub fn bits(i: u32) -> usize {
    32 - i.leading_zeros() as usize
}

/// Extracts a byte from an i32 array treated as packed bytes in big-endian order.
#[expect(dead_code)]
pub fn grab_byte(input: &[i32], index: u32) -> u8 {
    (input[(index / 4) as usize] >> (24 - (index % 4) * 8)) as u8
}

/// Returns the position of the most significant bit in `x` (1-indexed).
/// Returns 0 for input 0.
#[expect(dead_code)]
pub fn leading_bit_position(x: u32) -> i32 {
    bitlen(u64::from(x))
}

/// Counts the number of leading zeros in `x`.
fn clz(x: u64) -> u64 {
    u64::from(x.leading_zeros())
}

/// Returns the bit length of `x` (number of bits needed to represent it).
/// Returns 0 for input 0.
fn bitlen(x: u64) -> i32 {
    if x == 0 {
        return 0;
    }
    64 - clz(x) as i32
}
