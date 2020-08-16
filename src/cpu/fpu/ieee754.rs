/*
    IEEE Standard for Floating-Point Arithmetic, in IEEE Std 754-2008,
    pp.1-70, 29 Aug. 2008, doi: 10.1109/IEEESTD.2008.4610935.
*/

use crate::bits::*;

#[derive(PartialOrd, PartialEq)]
pub enum FloatClass {
    Zero,
    Float32,
    Float64,
    Inf,
    QNaN, /* all NaNs from here */
    SNaN,
}

pub enum RoundMode {
    RNE, // Round to Nearest, ties to Even
    RTZ, // Round towards Zero
    RDN, // Round Down (towards -Inf)
    RUP, // Round Up (towards +Inf)
    RMM, // Round to Nearest, ties to Max Magnitude
    DYN, // In instruction’s rm field, selects dynamic rounding mode; In frm, Invalid.
}

impl RoundMode {
    pub fn from_u32(n: u32) -> Option<RoundMode> {
        match n {
            0b000 => Some(RoundMode::RNE),
            0b001 => Some(RoundMode::RTZ),
            0b010 => Some(RoundMode::RDN),
            0b011 => Some(RoundMode::RUP),
            0b100 => Some(RoundMode::RMM),
            0b111 => Some(RoundMode::DYN),
            _ => None,
        }
    }
}

pub struct FloatParts {
    pub sign: bool, // 1 bit
    pub exp: i16,   // (8, 11) bits
    pub frac: u64,  // (23, 52) bits
    pub class: FloatClass,
}

impl FloatParts {
    pub fn new32(f: f32) -> Self {
        let n: u32 = f.to_bits();
        let sign: bool = read_bit(n, 31) != 0;

        // exp32 - (-126~127)+127 = 1~254
        let mut exp: i16 = read_bits(n, 23..30) as i16;

        // flac32 [63] - overflow flag
        // flac32 [62] - implicit bit
        // flac32 [61:39] - frac
        // flac32 [38:0] - pad
        let mut frac: u64 = (read_bits(n, 0..22) as u64) << 39;
        let class: FloatClass = match (exp, frac) {
            (0xFF, 0) => FloatClass::Inf,
            (0xFF, _) => FloatClass::QNaN,
            (0x00, 0) => FloatClass::Zero,
            (0x00, _) => {
                // Denormalized
                exp = -126;
                FloatClass::Float32
            }
            (_, _) => {
                exp -= 127;
                frac |= 1 << 62; // implicit bit
                FloatClass::Float32
            }
        };

        FloatParts {
            sign,
            exp,
            frac,
            class,
        }
    }

    pub fn new64(f: f64) -> Self {
        let n: u64 = f.to_bits();
        let sign: bool = read_bit64(n, 63) != 0;

        // exp64 - (-1022~1023)+1023 = 1~2046
        let mut exp: i16 = read_bits64(n, 52..62) as i16;

        // flac32 [63] - overflow flag
        // flac32 [62] - implicit bit
        // flac32 [61:10] - frac
        // flac32 [9:0] - pad
        let mut frac: u64 = read_bits64(n, 0..51) << 11;
        let class = match (exp, frac) {
            (0x7FF, 0) => FloatClass::Inf,
            (0x7FF, _) => FloatClass::QNaN,
            (0x000, 0) => FloatClass::Zero,
            (0x000, _) => {
                // Denormalized
                exp = -1022;
                FloatClass::Float64
            }
            (_, _) => {
                exp -= 1023;
                frac |= 1 << 62; // implicit bit
                FloatClass::Float64
            }
        };
        FloatParts {
            sign,
            exp,
            frac,
            class,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self.class {
            FloatClass::Inf => f32::from_bits(0x7F80_0000),
            FloatClass::QNaN => f32::from_bits(0xFFFF_FFFF),
            FloatClass::SNaN => f32::from_bits(0xFFFF_FFFF),
            FloatClass::Zero => f32::from_bits(0),
            FloatClass::Float32 => {
                if (self.exp == -126) & (read_bit64(self.frac, 62) == 0) {
                    // Denormalized
                    f32::from_bits(
                        ((self.sign as u32) << 31) | read_bits64(self.frac, 39..61) as u32,
                    )
                } else {
                    let _exp = ((self.exp + 127) as u32) << 23;
                    f32::from_bits(
                        ((self.sign as u32) << 31) | _exp | read_bits64(self.frac, 39..61) as u32,
                    )
                }
            }
            FloatClass::Float64 => self.to_f64() as f32,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self.class {
            FloatClass::Inf => f64::from_bits(0x7FF0_0000_0000_0000),
            FloatClass::QNaN => f64::from_bits(0xFFFF_FFFF_FFFF_FFFF),
            FloatClass::SNaN => f64::from_bits(0xFFFF_FFFF_FFFF_FFFF),
            FloatClass::Zero => f64::from_bits(0),
            FloatClass::Float32 => self.to_f32() as f64,
            FloatClass::Float64 => {
                if (self.exp == -1022) & (read_bit64(self.frac, 62) == 0) {
                    // Denormalized
                    f64::from_bits(((self.sign as u64) << 63) | read_bits64(self.frac, 10..61))
                } else {
                    let _exp = ((self.exp + 1023) as u64) << 52;
                    f64::from_bits(
                        ((self.sign as u64) << 63) | _exp | read_bits64(self.frac, 10..61),
                    )
                }
            }
        }
    }
}
