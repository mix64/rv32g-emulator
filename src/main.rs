mod cpu;
mod exception;
mod memory;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cpu::Cpu;

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
        let result = cpu.run();
    }
    cpu.dump_registers();

    Ok(())
}
