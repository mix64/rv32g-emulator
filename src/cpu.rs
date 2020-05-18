use crate::memory::*;

const SP: usize = 2;

fn mask(val: u32, start: u32, end: u32) -> u32 {
    let bits = end - start + 1;
    (val >> start) & (0xFFFF_FFFF >> (32 - (bits)))
}

pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub ram: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        let mut regs: [u32; 32] = [0; 32];
        regs[SP] = MEMORY_SIZE;
        Cpu {
            regs,
            pc: 0,
            ram: Memory::new(),
        }
    }

    pub fn dump_registers(&self) {
        for i in (0..32).step_by(4) {
            println!(
                "x{:02}={:#08x} x{:02}={:#08x} x{:02}={:#08x} x{:02}={:#08x}",
                i,
                self.regs[i],
                i + 1,
                self.regs[i + 1],
                i + 2,
                self.regs[i + 2],
                i + 3,
                self.regs[i + 3],
            );
        }
    }

    pub fn fetch(&mut self) -> u32 {
        // TODO: if self.pc & 1 == 1 {raise exception of Instruction address misaligned}
        let inst = self.ram.read32(self.pc);
        self.pc += 4;
        return inst;
    }

    pub fn execute(&mut self, inst: u32) {
        let opcode = mask(inst, 0, 6);

        match opcode {
            0b011_0011 => {
                // R-type
                let rd = mask(inst, 7, 11) as usize;
                let funct3 = mask(inst, 12, 14);
                let rs1 = mask(inst, 15, 19) as usize;
                let rs2 = mask(inst, 20, 24) as usize;
                let funct7 = mask(inst, 25, 31);

                match (funct3, funct7) {
                    (0x0, 0x00) => {
                        // add
                        self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                    }
                    (0x0, 0x20) => {
                        // sub
                        self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                    }
                    (0x4, 0x00) => {
                        // xor
                        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
                    }
                    (0x6, 0x00) => {
                        // or
                        self.regs[rd] = self.regs[rs1] | self.regs[rs2];
                    }
                    (0x7, 0x00) => {
                        // and
                        self.regs[rd] = self.regs[rs1] & self.regs[rs2];
                    }
                    /*
                    SLL, SRL, and SRA perform logical left, logical right, and arithmetic right shifts
                    on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2. (1.2.4)
                    */
                    (0x1, 0x00) => {
                        // sll
                        let shamt = self.regs[rs2] & 0b1_1111;
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt);
                    }
                    (0x5, 0x00) => {
                        // srl
                        let shamt = self.regs[rs2] & 0b1_1111;
                        self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
                    }
                    (0x5, 0x20) => {
                        // sra
                        let shamt = self.regs[rs2] & 0b1_1111;
                        self.regs[rd] = (self.regs[rs1] as i32).wrapping_shr(shamt) as u32;
                    }
                    (0x2, 0x00) => {
                        // slt
                        self.regs[rd] = if (self.regs[rs1] as i32) < (self.regs[rs2] as i32) {
                            1
                        } else {
                            0
                        };
                    }
                    (0x3, 0x00) => {
                        // sltu
                        self.regs[rd] = if self.regs[rs1] < self.regs[rs2] {
                            1
                        } else {
                            0
                        };
                    }

                    _ => {}
                }
            }

            0b001_0011 => {
                // I-type
                let rd = mask(inst, 7, 11) as usize;
                let funct3 = mask(inst, 12, 14);
                let rs1 = mask(inst, 15, 19) as usize;
                let imm = ((inst as i32) >> 20) as u32;
                /*
                The operand to be shifted is in rs1, and the shift amount is
                encoded in the lower 5 bits of the I-immediate field. (1.2.4)
                */
                let shamt = mask(imm, 0, 4);
                let funct7 = mask(imm, 5, 11);

                match funct3 {
                    0x0 => {
                        // addi
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }
                    0x4 => {
                        // xori
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }
                    0x6 => {
                        // ori
                        self.regs[rd] = self.regs[rs1] | imm;
                    }
                    0x7 => {
                        // andi
                        self.regs[rd] = self.regs[rs1] & imm;
                    }
                    0x1 => {
                        // slli
                        self.regs[rd] = self.regs[rs1].wrapping_shl(shamt);
                    }
                    0x2 => {
                        // slti
                        self.regs[rd] = if (self.regs[rs1] as i32) < (imm as i32) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // sltiu
                        self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
                    }
                    0x5 => {
                        match funct7 {
                            0x00 => {
                                // srli
                                self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
                            }
                            0x20 => {
                                // srai
                                self.regs[rd] = (self.regs[rs1] as i32).wrapping_shr(shamt) as u32;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            0b000_0011 => {
                // I-type
                let rd = mask(inst, 7, 11) as usize;
                let funct3 = mask(inst, 12, 14);
                let rs1 = mask(inst, 15, 19) as usize;
                let imm = ((inst as i32) >> 20) as u32;
                let addr = self.regs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.ram.read8(addr);
                        self.regs[rd] = val as i8 as i32 as u32;
                    }
                    0x1 => {
                        // lh
                        let val = self.ram.read16(addr);
                        self.regs[rd] = val as i16 as i32 as u32;
                    }
                    0x2 => {
                        // lw
                        let val = self.ram.read32(addr);
                        self.regs[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.ram.read8(addr);
                        self.regs[rd] = val;
                    }
                    0x5 => {
                        // lhu
                        let val = self.ram.read16(addr);
                        self.regs[rd] = val;
                    }
                    _ => {}
                }
            }

            0b010_0011 => {
                // S-type
                let imm1 = mask(inst, 7, 11);
                let funct3 = mask(inst, 12, 14);
                let rs1 = mask(inst, 15, 19) as usize;
                let rs2 = mask(inst, 20, 24) as usize;
                let imm2 = ((inst & 0xfe00_0000) as i32 >> 25) as u32;
                let imm = (imm2 << 5) | imm1;
                let addr = self.regs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => {
                        // sb
                        self.ram.write8(addr, self.regs[rs2]);
                    }
                    0x1 => {
                        // sh
                        self.ram.write16(addr, self.regs[rs2]);
                    }
                    0x2 => {
                        // sw
                        self.ram.write32(addr, self.regs[rs2]);
                    }
                    _ => {}
                }
            }

            0b110_0011 => {
                // B-type
                let imm11 = mask(inst, 7, 7);
                let imm4 = mask(inst, 8, 11);
                let funct3 = mask(inst, 12, 14);
                let rs1 = mask(inst, 15, 19) as usize;
                let rs2 = mask(inst, 20, 24) as usize;
                let imm10 = mask(inst, 25, 30);
                let imm12 = ((inst & 0x8000_0000) as i32 >> 31) as u32;
                // The conditional branch range is ±4 KiB. (1.2.5)
                let imm = imm12 << 12 | imm11 << 11 | imm10 << 5 | imm4 << 1;

                match funct3 {
                    0x0 => {
                        // beq
                        if self.regs[rs1] == self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x1 => {
                        // bne
                        if self.regs[rs1] != self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x4 => {
                        // blt
                        if (self.regs[rs1] as i32) < (self.regs[rs2] as i32) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x5 => {
                        // bge
                        if (self.regs[rs1] as i32) >= (self.regs[rs2] as i32) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x6 => {
                        // bltu
                        if self.regs[rs1] < self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x7 => {
                        // bgeu
                        if self.regs[rs1] >= self.regs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    _ => {}
                }
            }

            0b110_1111 => {
                // J-type
                // jal
                let rd = mask(inst, 7, 11) as usize;
                let imm19 = mask(inst, 12, 19);
                let imm11 = mask(inst, 20, 20);
                let imm10 = mask(inst, 21, 30);
                let imm20 = ((inst & 0x8000_0000) as i32 >> 31) as u32;
                // Jumps can therefore target a ±1 MiB range (1.2.5)
                let imm = imm20 << 20 | imm19 << 12 | imm11 << 11 | imm10 << 1;

                self.regs[rd] = self.pc;
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            0b110_0111 => {
                // I-type
                // jalr
                let rd = mask(inst, 7, 11) as usize;
                let funct3 = mask(inst, 12, 14);
                let rs1 = mask(inst, 15, 19) as usize;
                let imm = ((inst as i32) >> 20) as u32;
                let addr = self.regs[rs1].wrapping_add(imm);
                if funct3 == 0 {
                    self.regs[rd] = self.pc;
                    self.pc = addr;
                }
            }

            0b011_0111 => {
                // U-type
                // lui
                let rd = mask(inst, 7, 11) as usize;
                self.regs[rd] = inst & 0xFFFF_F000;
            }

            0b001_0111 => {
                // U-type
                // auipc
                let rd = mask(inst, 7, 11) as usize;
                let imm = inst & 0xFFFF_F000;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            /* TODO: implement ecall and ebreak
            0b111_0011 => {
                // I-type
            }
            */
            _ => {
                // TODO: raise exception of Illegal instruction
                println!("not implemented yet: opcode {:#x}", opcode);
            }
        }
        // Register x0 is hardwired with all bits equal to 0. (1.2.1)
        self.regs[0] = 0;
    }
}
