// Note: Changing the round mode using fesetround() in Rust.
// https://github.com/rust-lang/rust/issues/41753

mod ieee754;
use crate::exception::Exception;
extern crate num_traits;
use ieee754::*;

pub const FP32: u32 = 0b00;
pub const FP64: u32 = 0b01;
// pub const FP16: u32 = 0b10;
// pub const FP128: u32 = 0b11;

// TODO: Change round mode
pub fn fadd_32(fa: f32, fb: f32, rm: u32) -> Result<f32, Exception> {
    // let a = FloatParts::new32(fa);
    // let b = FloatParts::new32(fb);
    // match RoundMode::from_u32(rm) {
    //     Some(mode) => {
    //         let c = fadd(a, b, mode)?;
    //         return Ok(c.to_f32());
    //     }
    //     None => Err(Exception::IllegalInstruction),
    // }
    Ok(fa + fb)
}

pub fn fadd_64(fa: f64, fb: f64, rm: u32) -> Result<f64, Exception> {
    Ok(fa + fb)
}

// pub fn fadd<T: num_traits::Float + num_traits::FromPrimitive>(fa: T, fb: T, rm: u32) {}
fn fadd(a: FloatParts, b: FloatParts, rm: RoundMode) -> Result<FloatParts, Exception> {
    // TODO: implement
    Ok(a)
}

pub fn fsub_32(fa: f32, fb: f32, rm: u32) -> Result<f32, Exception> {
    Ok(fa - fb)
}

pub fn fsub_64(fa: f64, fb: f64, rm: u32) -> Result<f64, Exception> {
    Ok(fa - fb)
}

pub fn fmul_32(fa: f32, fb: f32, rm: u32) -> Result<f32, Exception> {
    Ok(fa * fb)
}

pub fn fmul_64(fa: f64, fb: f64, rm: u32) -> Result<f64, Exception> {
    Ok(fa * fb)
}

pub fn fdiv_32(fa: f32, fb: f32, rm: u32) -> Result<f32, Exception> {
    if fb != 0.0 {
        Ok(fa / fb)
    } else {
        // set fcsr error flags
        Ok(f32::INFINITY)
    }
}

pub fn fdiv_64(fa: f64, fb: f64, rm: u32) -> Result<f64, Exception> {
    if fb != 0.0 {
        Ok(fa / fb)
    } else {
        // set fcsr error flags
        Ok(f64::INFINITY)
    }
}

pub fn fsqrt_32(fa: f32, rm: u32) -> Result<f32, Exception> {
    Ok(fa.sqrt())
}

pub fn fsqrt_64(fa: f64, rm: u32) -> Result<f64, Exception> {
    Ok(fa.sqrt())
}

pub fn fsgnj_32(fa: f32, fb: f32, funct3: u32) -> Result<f32, Exception> {
    let a = FloatParts::new32(fa);
    let b = FloatParts::new32(fb);

    let c = fsgnj(a, b, funct3)?;
    return Ok(c.to_f32());
}

pub fn fsgnj_64(fa: f64, fb: f64, funct3: u32) -> Result<f64, Exception> {
    let a = FloatParts::new64(fa);
    let b = FloatParts::new64(fb);

    let c = fsgnj(a, b, funct3)?;
    return Ok(c.to_f64());
}

fn fsgnj(mut a: FloatParts, b: FloatParts, funct3: u32) -> Result<FloatParts, Exception> {
    match funct3 {
        0b000 => {
            // fsgnj
            a.sign = b.sign;
        }
        0b001 => {
            // fsgnjn
            a.sign = !b.sign;
        }
        0b010 => {
            // fsgnjx
            a.sign = a.sign ^ b.sign;
        }
        _ => return Err(Exception::IllegalInstruction),
    }
    return Ok(a);
}

pub fn fclass_32(fa: f32) -> u32 {
    let a = FloatParts::new32(fa);
    return fclass(a);
}

pub fn fclass_64(fa: f64) -> u32 {
    let a = FloatParts::new64(fa);
    return fclass(a);
}

fn fclass(a: FloatParts) -> u32 {
    match a.class {
        FloatClass::Zero => {
            if a.sign {
                1 << 3
            } else {
                1 << 4
            }
        }
        FloatClass::Inf => {
            if a.sign {
                1
            } else {
                1 << 7
            }
        }
        FloatClass::QNaN => 1 << 9,
        FloatClass::SNaN => 1 << 8,
        FloatClass::Float32 | FloatClass::Float64 => match (a.sign, a.frac & (1 << 62) == 0) {
            (true, true) => 1 << 2,
            (true, false) => 1 << 1,
            (false, true) => 1 << 5,
            (false, false) => 1 << 6,
        },
    }
}

pub fn fmin<T: num_traits::Float>(fa: T, fb: T) -> T {
    fa.min(fb)
}

pub fn fmax<T: num_traits::Float>(fa: T, fb: T) -> T {
    fa.max(fb)
}

pub fn fle<T: num_traits::Float>(fa: T, fb: T) -> u32 {
    (fa <= fb) as u32
}

pub fn flt<T: num_traits::Float>(fa: T, fb: T) -> u32 {
    (fa < fb) as u32
}

pub fn feq<T: num_traits::Float>(fa: T, fb: T) -> u32 {
    (fa == fb) as u32
}
