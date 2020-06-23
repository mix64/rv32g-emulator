use crate::cpu::{Cpu, Mode};
use crate::exception::Exception;

mod address;
mod mstatus;

use address::*;
use mstatus::*;

fn read_bit(reg: u32, bit: u32) -> u32 {
    read_bits(reg, bit..bit)
}

fn read_bits(reg: u32, range: std::ops::Range<u32>) -> u32 {
    let mask = if range.end != 31 {
        std::u32::MAX.wrapping_shl(range.end + 1)
    } else {
        0
    };
    (reg & !mask) >> range.start
}

fn write_bit(reg: &mut u32, bit: u32, val: u32) {
    write_bits(reg, bit..bit, val)
}

fn write_bits(reg: &mut u32, range: std::ops::Range<u32>, val: u32) {
    let mask = (!0 << (range.end + 1)) | !(!0 << range.start);
    *reg = *reg & mask | val << range.start;
}

pub struct CSRs {
    mstatus: u32,
    mtvec: u32,
    mip: u32,
    mie: u32,
    mscratch: u32,
    mepc: u32,
    mcause: u32,
    mtval: u32,
}

impl CSRs {
    pub fn new() -> Self {
        CSRs {
            mstatus: 0,
            mtvec: 0,
            mip: 0,
            mie: 0,
            mscratch: 0,
            mepc: 0,
            mcause: 0,
            mtval: 0,
        }
    }
}

impl Cpu {
    pub fn trap(&mut self, e: Exception) {
        let ecode = e.exception_code();

        self.csrs.mepc = self.pc - 4;
        self.pc = match self.csrs.mtvec & 0b11 {
            0 => self.csrs.mtvec & !0b11,
            1 => (self.csrs.mtvec & !0b11) + 4 * ecode,
            _ => panic!("unknown CSR.mtvec MODE"),
        };

        self.csrs.mcause = ecode;
        // TODO: Set a correct value to mtval.
        self.csrs.mtval = self.ram.read32(self.csrs.mepc).unwrap();

        let mpie = read_bit(self.csrs.mstatus, MSTATUS_MIE);
        write_bit(&mut self.csrs.mstatus, MSTATUS_MIE, 0);
        write_bit(&mut self.csrs.mstatus, MSTATUS_MPIE, mpie);

        write_bits(
            &mut self.csrs.mstatus,
            MSTATUS_MPP..MSTATUS_MPP + 1,
            self.mode as u32,
        );
        self.mode = Mode::Machine;

        // println!(
        //     "Trap at [{:08x}] {:08x} (excode: {:})",
        //     self.csrs.mepc, self.csrs.mtval, ecode
        // );
        // println!("Jump to [{:08x}]\n", self.pc);
    }

    pub fn csrr(&self, src: u16) -> Result<u32, Exception> {
        match src {
            MSTATUS => Ok(self.csrs.mstatus),
            MTVEC => Ok(self.csrs.mtvec),
            MIP => Ok(self.csrs.mip),
            MIE => Ok(self.csrs.mie),
            MSCRATCH => Ok(self.csrs.mscratch),
            MEPC => Ok(self.csrs.mepc),
            MCAUSE => Ok(self.csrs.mcause),
            MTVAL => Ok(self.csrs.mtval),
            MHARTID => Ok(0),
            _ => Err(Exception::IllegalInstruction),
        }
    }

    pub fn csrw(&mut self, dst: u16, imm: u32) -> Result<(), Exception> {
        // TODO: Check imm
        match dst {
            MSTATUS => {
                self.csrs.mstatus = imm;
                Ok(())
            }
            MTVEC => {
                let mode = imm & 0b11;
                match mode {
                    0 | 1 => {
                        self.csrs.mtvec = imm;
                    }
                    _ => self.csrs.mtvec = (self.csrs.mtvec & 0b11) | (imm & !0b11),
                }
                Ok(())
            }
            MIP => {
                self.csrs.mip = imm;
                Ok(())
            }
            MIE => {
                self.csrs.mie = imm;
                Ok(())
            }
            MSCRATCH => {
                self.csrs.mscratch = imm;
                Ok(())
            }
            MEPC => {
                self.csrs.mepc = imm;
                Ok(())
            }
            MCAUSE => {
                self.csrs.mcause = imm;
                Ok(())
            }
            MTVAL => {
                self.csrs.mtval = imm;
                Ok(())
            }
            _ => Err(Exception::IllegalInstruction),
        }
    }
}
