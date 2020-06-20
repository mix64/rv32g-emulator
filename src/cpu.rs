mod execute;

#[allow(dead_code)]
mod csr;

use crate::exception::Exception;
use crate::memory::*;
use csr::CSRs;

const SP: usize = 2;
pub struct Cpu {
    pub regs: [u32; 32],
    pub fregs: [f32; 32],
    pub csrs: CSRs,
    pub pc: u32,
    pub ram: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        let mut regs: [u32; 32] = [0; 32];
        regs[SP] = MEMORY_SIZE;
        Cpu {
            regs,
            fregs: [0.0; 32],
            csrs: CSRs::new(),
            pc: 0,
            ram: Memory::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        loop {
            let inst = self.fetch()?;
            self.execute(inst)?;
            if self.pc == 0 {
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
                self.regs[i],
                self.regs[i] as i32,
                i + 1,
                self.regs[i + 1],
                self.regs[i + 1] as i32,
                i + 2,
                self.regs[i + 2],
                self.regs[i + 2] as i32,
                i + 3,
                self.regs[i + 3],
                self.regs[i + 3] as i32,
            );
        }
    }
}
