// Note: Changing the round mode using fesetround() in Rust.
// https://github.com/rust-lang/rust/issues/41753
use crate::exception::Exception;
extern crate num_traits;
use softfloat_wrapper::{ExceptionFlags, Float, RoundingMode, F32, F64};

pub const FP32: u32 = 0b00;
pub const FP64: u32 = 0b01;
// pub const FP16: u32 = 0b10;
// pub const FP128: u32 = 0b11;

const FFLAGS_NX: u8 = 1; // Inexact
const FFLAGS_UF: u8 = 1 << 1; // Underflow
const FFLAGS_OF: u8 = 1 << 2; // Overflow
const FFLAGS_DZ: u8 = 1 << 3; // Divide by Zero
const FFLAGS_NV: u8 = 1 << 4; // Invalid Operation

static mut FCSR: u32 = 0;

fn rnd_from_u32(rnd: u32) -> Result<RoundingMode, Exception> {
    match rnd {
        0b000 => Ok(RoundingMode::TiesToEven),
        0b001 => Ok(RoundingMode::TowardZero),
        0b010 => Ok(RoundingMode::TowardNegative),
        0b011 => Ok(RoundingMode::TowardPositive),
        0b100 => Ok(RoundingMode::TiesToAway),
        0b111 => read_frm(),
        _ => Err(Exception::IllegalInstruction),
    }
}

// NOTE: num_traits::Float doesn't have a fn to_bits()
fn soft_float<U: num_traits::Unsigned, F: Float>(a: U, f: fn(U) -> F) -> F {
    f(a)
}

fn write_fflags(f: ExceptionFlags) {
    let flags = if f.is_inexact() {
        FFLAGS_NX
    } else if f.is_underflow() {
        FFLAGS_UF
    } else if f.is_overflow() {
        FFLAGS_OF
    } else if f.is_invalid() {
        FFLAGS_NV
    } else {
        0
    };
    unsafe {
        FCSR |= flags as u32;
    }
}

fn read_frm() -> Result<RoundingMode, Exception> {
    Ok(RoundingMode::TiesToEven)
}

fn f_arithmetic<F: Float>(a: F, b: F, rnd: RoundingMode, f: fn(&F, F, RoundingMode) -> F) -> F {
    let mut flag = ExceptionFlags::default();
    flag.set();
    let c = f(&a, b, rnd);
    flag.get();
    write_fflags(flag);
    return c;
}

pub fn fadd_32(fa: f32, fb: f32, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let b = soft_float(fb.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = f_arithmetic(a, b, rnd, F32::add);
    Ok(f32::from_bits(c.bits()))
}

pub fn fadd_64(fa: f64, fb: f64, funct3: u32) -> Result<f64, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let b = soft_float(fb.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = f_arithmetic(a, b, rnd, F64::add);
    Ok(f64::from_bits(c.bits()))
}

pub fn fsub_32(fa: f32, fb: f32, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let b = soft_float(fb.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = f_arithmetic(a, b, rnd, F32::sub);
    Ok(f32::from_bits(c.bits()))
}

pub fn fsub_64(fa: f64, fb: f64, funct3: u32) -> Result<f64, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let b = soft_float(fb.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = f_arithmetic(a, b, rnd, F64::sub);
    Ok(f64::from_bits(c.bits()))
}

pub fn fmul_32(fa: f32, fb: f32, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let b = soft_float(fb.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = f_arithmetic(a, b, rnd, F32::mul);
    Ok(f32::from_bits(c.bits()))
}

pub fn fmul_64(fa: f64, fb: f64, funct3: u32) -> Result<f64, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let b = soft_float(fb.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = f_arithmetic(a, b, rnd, F64::mul);
    Ok(f64::from_bits(c.bits()))
}

pub fn fdiv_32(fa: f32, fb: f32, funct3: u32) -> Result<f32, Exception> {
    if fb != 0.0 {
        let a = soft_float(fa.to_bits(), F32::from_bits);
        let b = soft_float(fb.to_bits(), F32::from_bits);
        let rnd = rnd_from_u32(funct3)?;
        let c = f_arithmetic(a, b, rnd, F32::div);
        Ok(f32::from_bits(c.bits()))
    } else {
        unsafe { FCSR |= FFLAGS_DZ as u32 };
        Ok(f32::INFINITY)
    }
}

pub fn fdiv_64(fa: f64, fb: f64, funct3: u32) -> Result<f64, Exception> {
    if fb != 0.0 {
        let a = soft_float(fa.to_bits(), F64::from_bits);
        let b = soft_float(fb.to_bits(), F64::from_bits);
        let rnd = rnd_from_u32(funct3)?;
        let c = f_arithmetic(a, b, rnd, F64::div);
        Ok(f64::from_bits(c.bits()))
    } else {
        unsafe { FCSR |= FFLAGS_DZ as u32 };
        Ok(f64::INFINITY)
    }
}

pub fn fsqrt_32(fa: f32, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = a.sqrt(rnd);
    Ok(f32::from_bits(c.bits()))
}

pub fn fsqrt_64(fa: f64, funct3: u32) -> Result<f64, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = a.sqrt(rnd);
    Ok(f64::from_bits(c.bits()))
}

pub fn fsgnj_32(fa: f32, fb: f32, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let b = soft_float(fb.to_bits(), F32::from_bits);
    let c = fsgnj(a, b, funct3)?;
    Ok(f32::from_bits(c.bits()))
}

pub fn fsgnj_64(fa: f64, fb: f64, funct3: u32) -> Result<f64, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let b = soft_float(fb.to_bits(), F64::from_bits);
    let c = fsgnj(a, b, funct3)?;
    Ok(f64::from_bits(c.bits()))
}

fn fsgnj<F: Float>(mut a: F, b: F, funct3: u32) -> Result<F, Exception> {
    let a_sign = a.sign();
    let b_sign = b.sign();

    match funct3 {
        0b000 => {
            // fsgnj
            a.set_sign(b_sign);
        }
        0b001 => {
            // fsgnjn
            a.set_sign(!b_sign);
        }
        0b010 => {
            // fsgnjx
            a.set_sign(a_sign ^ b_sign);
        }
        _ => return Err(Exception::IllegalInstruction),
    }
    return Ok(a);
}

pub fn fclass_32(fa: f32) -> u32 {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    return fclass(a);
}

pub fn fclass_64(fa: f64) -> u32 {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    return fclass(a);
}

fn fclass<F: Float>(a: F) -> u32 {
    if a.is_positive_infinity() {
        1
    } else if a.is_negative_normal() {
        1 << 1
    } else if a.is_negative_subnormal() {
        1 << 2
    } else if a.is_negative_zero() {
        1 << 3
    } else if a.is_positive_zero() {
        1 << 4
    } else if a.is_positive_subnormal() {
        1 << 5
    } else if a.is_positive_normal() {
        1 << 6
    } else if a.is_positive_infinity() {
        1 << 7
    } else if a.is_nan() {
        1 << 8
    } else {
        0
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

// f64 -> f32
pub fn fcvt_s_d(fa: f64, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let c = a.to_f32(rnd);
    Ok(f32::from_bits(c.bits()))
}

// f32 -> i32
pub fn fcvt_w_s(fa: f32, funct3: u32) -> Result<i32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    Ok(a.to_i32(rnd))
}

// f64 -> i32
pub fn fcvt_w_d(fa: f64, funct3: u32) -> Result<i32, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    Ok(a.to_i32(rnd))
}

// f32 -> u32
pub fn fcvt_wu_s(fa: f32, funct3: u32) -> Result<u32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    Ok(a.to_u32(rnd))
}

// f64 -> u32
pub fn fcvt_wu_d(fa: f64, funct3: u32) -> Result<u32, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    Ok(a.to_u32(rnd))
}

// i32 -> f32
pub fn fcvt_s_w(i: i32, funct3: u32) -> Result<f32, Exception> {
    let rnd = rnd_from_u32(funct3)?;
    let c = F32::from_i32(i, rnd);
    Ok(f32::from_bits(c.bits()))
}

// i32 -> f64
pub fn fcvt_d_w(i: i32, funct3: u32) -> Result<f64, Exception> {
    let rnd = rnd_from_u32(funct3)?;
    let c = F64::from_i32(i, rnd);
    Ok(f64::from_bits(c.bits()))
}

// u32 -> f32
pub fn fcvt_s_wu(u: u32, funct3: u32) -> Result<f32, Exception> {
    let rnd = rnd_from_u32(funct3)?;
    let c = F32::from_u32(u, rnd);
    Ok(f32::from_bits(c.bits()))
}

// u32 -> f64
pub fn fcvt_d_wu(u: u32, funct3: u32) -> Result<f64, Exception> {
    let rnd = rnd_from_u32(funct3)?;
    let c = F64::from_u32(u, rnd);
    Ok(f64::from_bits(c.bits()))
}

pub fn fmadd_32(fa: f32, fb: f32, fc: f32, funct3: u32) -> Result<f32, Exception> {
    let a = soft_float(fa.to_bits(), F32::from_bits);
    let b = soft_float(fb.to_bits(), F32::from_bits);
    let c = soft_float(fc.to_bits(), F32::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let d = a.fused_mul_add(b, c, rnd);
    Ok(f32::from_bits(d.bits()))
}

pub fn fmadd_64(fa: f64, fb: f64, fc: f64, funct3: u32) -> Result<f64, Exception> {
    let a = soft_float(fa.to_bits(), F64::from_bits);
    let b = soft_float(fb.to_bits(), F64::from_bits);
    let c = soft_float(fc.to_bits(), F64::from_bits);
    let rnd = rnd_from_u32(funct3)?;
    let d = a.fused_mul_add(b, c, rnd);
    Ok(f64::from_bits(d.bits()))
}
