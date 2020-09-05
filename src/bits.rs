pub fn read_bit(reg: u32, bit: u32) -> u32 {
    read_bits(reg, bit..bit)
}

pub fn read_bits(reg: u32, range: std::ops::Range<u32>) -> u32 {
    let mask = if range.end < 31 {
        std::u32::MAX.wrapping_shl(range.end + 1)
    } else {
        0
    };
    (reg & !mask) >> range.start
}

pub fn write_bit(reg: &mut u32, bit: u32, val: u32) {
    write_bits(reg, bit..bit, val)
}

pub fn write_bits(reg: &mut u32, range: std::ops::Range<u32>, val: u32) {
    let t = if range.end < 31 {
        std::u32::MAX.wrapping_shl(range.end + 1)
    } else {
        0
    };
    let mask = t | !(std::u32::MAX.wrapping_shl(range.start));
    *reg = *reg & mask | val << range.start;
}
