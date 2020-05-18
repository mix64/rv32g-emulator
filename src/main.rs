mod cpu;
mod memory;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cpu::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: rv32g-emulator <filename>");
    }

    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let size = binary.len();
    let mut cpu = Cpu::new();
    cpu.ram.set(binary);

    while cpu.pc < size as u32 {
        let inst = cpu.fetch();
        cpu.execute(inst);
        if cpu.pc == 0 {
            // ra(x1) initialized 0
            break;
        }
    }
    cpu.dump_registers();

    Ok(())
}
