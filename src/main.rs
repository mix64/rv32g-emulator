extern crate elf;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod bits;
mod cpu;
mod exception;
mod fpu;
mod memory;

use cpu::Cpu;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: rv32g-emulator <filename>");
    }

    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.seek(io::SeekFrom::Start(0x1000))?; // skip ELF header
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new();
    cpu.ram.set(&binary);

    let end_address = get_write_tohost_address(&args[1]);

    loop {
        let result = cpu.run(end_address as u32);
        match result {
            Ok(_) => break, // reach to end point
            Err(e) => cpu.trap(e),
        }
    }
    cpu.dump_registers();
    Ok(())
}

fn get_write_tohost_address(filename: &str) -> u64 {
    let file = elf::File::open_path(filename).expect("No such file.");
    let symtab = file
        .get_section(".symtab")
        .expect("Failed to look up .symtab section");
    let symbols = file.get_symbols(symtab).unwrap();

    for s in symbols {
        if s.name == "write_tohost" {
            return s.value - file.ehdr.entry;
        }
    }
    return 0;
}
