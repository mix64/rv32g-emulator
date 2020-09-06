use crate::cpu::Cpu;
use crate::exception::Exception;
use crate::fpu;

mod address;
mod mstatus;

pub use address::*;
pub use mstatus::*;

impl Cpu {
    pub fn csrr(&self, src: usize) -> Result<u32, Exception> {
        // TODO: mask registers
        match src {
            SSTATUS | USTATUS => Ok(self.csrs[MSTATUS]),
            SIP | UIP => Ok(self.csrs[MIP]),
            SIE | UIE => Ok(self.csrs[MIE]),
            FCSR => unsafe { Ok(fpu::FCSR) },
            FFLAGS => unsafe { Ok(fpu::FCSR & 0x1F) },
            FRM => unsafe { Ok(fpu::FCSR & 0xE0) },
            _ => Ok(self.csrs[src]),
        }
    }

    pub fn csrw(&mut self, dst: usize, imm: u32) -> Result<(), Exception> {
        // TODO: Check imm
        match dst {
            SSTATUS | USTATUS => {
                self.csrs[MSTATUS] = imm;
            }
            SIP | UIP => {
                self.csrs[MIP] = imm;
            }
            SIE | UIE => {
                self.csrs[MIE] = imm;
            }
            FCSR => unsafe {
                fpu::FCSR = imm;
            },
            FFLAGS => unsafe {
                fpu::FCSR &= !0x1F;
                fpu::FCSR |= imm & 0x1F;
            },
            FRM => unsafe {
                fpu::FCSR &= !0xE0;
                fpu::FCSR |= imm & 0xE0;
            },
            _ => self.csrs[dst] = imm,
        }
        Ok(())
    }
}
