#[allow(dead_code)]
mod csr;
mod execute;
mod trap;
mod vm;
#[allow(unused_variables)]
use crate::exception::Exception;
use crate::memory::*;

const SP: usize = 2;
const NCSR: usize = 0x1000;

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

pub struct Cpu {
    pub xregs: [u32; 32],
    pub fregs: [f64; 32],
    pub csrs: [u32; NCSR],
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
            fregs: [0.0f64; 32],
            csrs: [0; NCSR],
            pc: 0,
            ram: Memory::new(),
            mode: Mode::Machine,
        }
    }

    pub fn run(&mut self, end: u32) -> Result<(), Exception> {
        loop {
            let inst = self.vm_fetch(self.pc)?;
            self.pc += 4;

            // println!("[{:08x}] {:08x}", self.pc - 4, inst);
            self.execute(inst)?;
            if self.pc == end {
                return Ok(());
            }
        }
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
                "f{:02}={:#018x} ({:3})  f{:02}={:#018x} ({:3})  f{:02}={:#018x} ({:3})  f{:02}={:#018x} ({:3})",
                i,
                self.fregs[i].to_bits(),
                self.fregs[i],
                i + 1,
                self.fregs[i+1].to_bits(),
                self.fregs[i + 1],
                i + 2,
                self.fregs[i+2].to_bits(),
                self.fregs[i + 2],
                i + 3,
                self.fregs[i+3].to_bits(),
                self.fregs[i + 3],
            );
        }
    }
}
