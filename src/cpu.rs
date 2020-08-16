#[allow(dead_code)]
mod csr;
mod execute;
#[allow(unused_variables)]
mod fpu;

use crate::exception::Exception;
use crate::memory::*;
use csr::CSRs;

const SP: usize = 2;

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

pub struct Cpu {
    pub xregs: [u32; 32],
    pub fregs32: [f32; 32],
    pub fregs64: [f64; 32],
    pub csrs: CSRs,
    pub pc: u32,
    pub mode: Mode,
    pub ram: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        let mut xregs: [u32; 32] = [0; 32];
        xregs[SP] = MEMORY_SIZE;
        Cpu {
            xregs,
            fregs32: [0.0f32; 32],
            fregs64: [0.0f64; 32],
            csrs: CSRs::new(),
            pc: 0,
            ram: Memory::new(),
            mode: Mode::Machine,
        }
    }

    pub fn run(&mut self, end: u32) -> Result<(), Exception> {
        loop {
            let inst = self.fetch()?;

            // println!("[{:08x}] {:08x}", self.pc - 4, inst);
            self.execute(inst)?;
            if self.pc == end {
                return Ok(());
            }
        }
    }

    pub fn fetch(&mut self) -> Result<u32, Exception> {
        // TODO: if self.pc & 1 == 1 {raise exception of Instruction address misaligned}
        if self.pc & 1 == 1 {
            return Err(Exception::InstructionAddressMisaligned);
        }
        let inst = self.ram.read32(self.pc)?;
        self.pc += 4;
        Ok(inst)
    }

    pub fn dump_registers(&self) {
        for i in (0..32).step_by(4) {
            println!(
                "x{:02}={:#010x} ({:11})  x{:02}={:#010x} ({:11})  x{:02}={:#010x} ({:11})  x{:02}={:#010x} ({:11})",
                i,
                self.xregs[i],
                self.xregs[i] as i32,
                i + 1,
                self.xregs[i + 1],
                self.xregs[i + 1] as i32,
                i + 2,
                self.xregs[i + 2],
                self.xregs[i + 2] as i32,
                i + 3,
                self.xregs[i + 3],
                self.xregs[i + 3] as i32,
            );
        }
        for i in (0..32).step_by(4) {
            println!(
                "f{:02}={:}  f{:02}={:}  f{:02}={:}  f{:02}={:}",
                i,
                self.fregs32[i],
                i + 1,
                self.fregs32[i + 1],
                i + 2,
                self.fregs32[i + 2],
                i + 3,
                self.fregs32[i + 3],
            );
        }

        for i in (0..32).step_by(4) {
            println!(
                "f{:02}={:}  f{:02}={:}  f{:02}={:}  f{:02}={:}",
                i,
                self.fregs64[i],
                i + 1,
                self.fregs64[i + 1],
                i + 2,
                self.fregs64[i + 2],
                i + 3,
                self.fregs64[i + 3],
            );
        }
    }
}
