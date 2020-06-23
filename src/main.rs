extern crate elf;

use std::env;

mod cpu;
mod exception;
mod memory;

use cpu::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: rv32g-emulator <filename>");
    }

    let file = elf::File::open_path(&args[1]).expect("No such file.");
    let text = file
        .get_section(".text.init")
        .expect("Failed to look up .text section");
    let symtab = file
        .get_section(".symtab")
        .expect("Failed to look up .symtab section");

    let end_address =
        get_function_address(file.get_symbols(symtab).unwrap(), "write_tohost") - file.ehdr.entry;

    let mut cpu = Cpu::new();
    cpu.ram.set(&text.data);

    loop {
        let result = cpu.run(end_address as u32);
        match result {
            Ok(_) => break, // reach to end point
            Err(e) => cpu.trap(e),
        }
    }
    cpu.dump_registers();
}

fn get_function_address(symbols: Vec<elf::types::Symbol>, target: &str) -> u64 {
    for s in symbols {
        if s.name == target {
            return s.value;
        }
    }
    return 0;
}
