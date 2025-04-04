pub fn greatest_multiple(value: u32, factor: u32) -> u32 {
    value - value % factor
}

pub fn bits(i: u32) -> usize {
    32 - i.leading_zeros() as usize
}

#[expect(dead_code)]
pub fn grap_byte(input: &[i32], index: u32) -> u8 {
    (input[(index / 4) as usize] >> (24 - (index % 4) * 8)) as u8
}

#[expect(dead_code)]
pub fn ceil_by(value: i32, factor: i32) -> i32 {
    value + factor - value % factor
}

#[expect(dead_code)]
pub fn leading_bit_position(x: u32) -> i32 {
    bitlen(u64::from(x))
}

fn clz(x: u64) -> u64 {
    u64::from(x.leading_zeros())
}

fn bitlen(x: u64) -> i32 {
    if x == 0 {
        return 0;
    }
    64 - clz(x) as i32
}

pub fn extract7bits(i: i32, val: i64) -> u8 {
    ((val >> (7 * i)) & ((1 << 7) - 1)) as u8
}

pub fn extract_7bits_maskless(i: i32, val: i64) -> u8 {
    (val >> (7 * i)) as u8
}
