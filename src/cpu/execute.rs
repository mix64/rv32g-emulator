use crate::cpu::{Cpu, Mode};
use crate::exception::Exception;

fn read_bits(reg: u32, range: std::ops::Range<u32>) -> u32 {
    let mask = if range.end != 31 {
        std::u32::MAX.wrapping_shl(range.end + 1)
    } else {
        0
    };
    (reg & !mask) >> range.start
}

impl Cpu {
    pub fn execute(&mut self, inst: u32) -> Result<(), Exception> {
        let opcode = read_bits(inst, 0..6);

        match opcode {
            0b011_0011 => {
                // R-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let funct7 = read_bits(inst, 25..31);

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

                    // RM32M
                    /*
                    Information on zero division. (1.7.2)
                        We considered raising exceptions on integer divide by zero,
                        with these exceptions causing a trap in most execution environments.
                        However, this would be the only arithmetic trap in the standard ISA
                        (floating-point exceptions set flags and write default values, but do not cause traps)
                        and would require language implementors to interact
                        with the execution environment’s trap handlers for this case.
                        Further, where language standards mandate that a divide-by-zero exception
                        must cause an immediate control flow change,
                        only a single branch instruction needs to be added to each divide operation,
                        and this branch instruction can be inserted after the divide
                        and should normally be very predictably not taken, adding little runtime overhead.

                        The value of all bits set is returned for both unsigned and signed divide by zero
                        to simplify the divider circuitry.
                        The value of all 1s is both the natural value to return for unsigned divide,
                        representing the largest unsigned number,
                        and also the natural result for simple unsigned divider implementations.
                        Signed division is often implemented using an unsigned division circuit
                        and specifying the same overflow result simplifies the hardware.
                    */
                    (0x0, 0x01) => {
                        // mul
                        self.regs[rd] = self.regs[rs1].wrapping_mul(self.regs[rs2]);
                    }
                    (0x1, 0x01) => {
                        // mulh (signed * signed)
                        self.regs[rd] = ((self.regs[rs1] as i32 as i64)
                            .wrapping_mul(self.regs[rs2] as i32 as i64)
                            >> 32) as u32
                    }
                    (0x2, 0x01) => {
                        // mulhsu (signed rs1 * unsigned rs2)
                        self.regs[rd] = ((self.regs[rs1] as i32 as i64)
                            .wrapping_mul(self.regs[rs2] as u64 as i64)
                            >> 32) as u32;
                    }
                    (0x3, 0x01) => {
                        // mulhu (unsigned * unsigned)
                        self.regs[rd] = ((self.regs[rs1] as u64)
                            .wrapping_mul(self.regs[rs2] as u64)
                            >> 32) as u32;
                    }
                    (0x4, 0x01) => {
                        // div
                        self.regs[rd] = if self.regs[rs2] == 0 {
                            // Divide by Zero
                            0xFFFF_FFFF // -1
                        } else {
                            // By using wrapping_*, Overflow case (dividend == -2^31 && divisor == -1) is included.
                            (self.regs[rs1] as i32).wrapping_div(self.regs[rs2] as i32) as u32
                        }
                    }
                    (0x5, 0x01) => {
                        // divu
                        self.regs[rd] = if self.regs[rs2] == 0 {
                            // Divide by Zero
                            0xFFFF_FFFF // 2^32-1
                        } else {
                            self.regs[rs1].wrapping_div(self.regs[rs2])
                        }
                    }
                    (0x6, 0x01) => {
                        // rem
                        self.regs[rd] = if self.regs[rs2] == 0 {
                            // Divide by Zero
                            self.regs[rs1]
                        } else {
                            // By using wrapping_*, Overflow case (dividend == -2^31 && divisor == -1) is included.
                            (self.regs[rs1] as i32).wrapping_rem(self.regs[rs2] as i32) as u32
                        }
                    }
                    (0x7, 0x01) => {
                        // remu
                        self.regs[rd] = if self.regs[rs2] == 0 {
                            // Divide by Zero
                            self.regs[rs1]
                        } else {
                            self.regs[rs1].wrapping_rem(self.regs[rs2])
                        }
                    }
                    _ => {}
                }
            }

            0b001_0011 => {
                // I-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let imm = ((inst as i32) >> 20) as u32;
                /*
                The operand to be shifted is in rs1, and the shift amount is
                encoded in the lower 5 bits of the I-immediate field. (1.2.4)
                */
                let shamt = read_bits(imm, 0..4);
                let funct7 = read_bits(imm, 5..11);

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
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let imm = ((inst as i32) >> 20) as u32;
                let addr = self.regs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.ram.read8(addr)?;
                        self.regs[rd] = val as i8 as i32 as u32;
                    }
                    0x1 => {
                        // lh
                        let val = self.ram.read16(addr)?;
                        self.regs[rd] = val as i16 as i32 as u32;
                    }
                    0x2 => {
                        // lw
                        let val = self.ram.read32(addr)?;
                        self.regs[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.ram.read8(addr)?;
                        self.regs[rd] = val;
                    }
                    0x5 => {
                        // lhu
                        let val = self.ram.read16(addr)?;
                        self.regs[rd] = val;
                    }
                    _ => {}
                }
            }

            0b010_0011 => {
                // S-type
                let imm1 = read_bits(inst, 7..11);
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let imm2 = ((inst & 0xfe00_0000) as i32 >> 25) as u32;
                let imm = (imm2 << 5) | imm1;
                let addr = self.regs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => {
                        // sb
                        self.ram.write8(addr, self.regs[rs2])?;
                    }
                    0x1 => {
                        // sh
                        self.ram.write16(addr, self.regs[rs2])?;
                    }
                    0x2 => {
                        // sw
                        self.ram.write32(addr, self.regs[rs2])?;
                    }
                    _ => {}
                }
            }

            0b110_0011 => {
                // B-type
                let imm11 = read_bits(inst, 7..7);
                let imm4 = read_bits(inst, 8..11);
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let imm10 = read_bits(inst, 25..30);
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
                let rd = read_bits(inst, 7..11) as usize;
                let imm19 = read_bits(inst, 12..19);
                let imm11 = read_bits(inst, 20..20);
                let imm10 = read_bits(inst, 21..30);
                let imm20 = ((inst & 0x8000_0000) as i32 >> 31) as u32;
                // Jumps can therefore target a ±1 MiB range (1.2.5)
                let imm = imm20 << 20 | imm19 << 12 | imm11 << 11 | imm10 << 1;

                self.regs[rd] = self.pc;
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            0b110_0111 => {
                // I-type
                // jalr
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
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
                let rd = read_bits(inst, 7..11) as usize;
                self.regs[rd] = inst & 0xFFFF_F000;
            }

            0b001_0111 => {
                // U-type
                // auipc
                let rd = read_bits(inst, 7..11) as usize;
                let imm = inst & 0xFFFF_F000;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            // RV32 Zicsr + ecall/ebreak
            0b111_0011 => {
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let imm = read_bits(inst, 15..19);
                let rs1 = imm as usize;
                let funct12 = read_bits(inst, 20..31);
                let csr = funct12 as u16;

                match funct3 {
                    0b000 => {
                        match funct12 {
                            0x0 => {
                                // ecall
                                match self.mode {
                                    Mode::Machine => {
                                        return Err(Exception::EnvironmentCallFromMMode)
                                    }
                                    Mode::Supervisor => {
                                        return Err(Exception::EnvironmentCallFromSMode)
                                    }
                                    Mode::User => return Err(Exception::EnvironmentCallFromUMode),
                                }
                            }
                            0x1 => {
                                // ebreak
                                return Err(Exception::Breakpoint);
                            }
                            _ => {}
                        }
                    }
                    0b001 => {
                        // csrrw
                        if rd != 0 {
                            self.regs[rd] = self.csrr(csr)?;
                        }
                        self.csrw(csr, self.regs[rs1])?;
                    }
                    0b010 => {
                        // csrrs
                        self.regs[rd] = self.csrr(csr)?;
                        if rs1 != 0 {
                            self.csrw(csr, self.regs[rd] | self.regs[rs1])?;
                        }
                    }
                    0b011 => {
                        // csrrc
                        self.regs[rd] = self.csrr(csr)?;
                        if rs1 != 0 {
                            self.csrw(csr, self.regs[rd] & !self.regs[rs1])?;
                        }
                    }
                    0b101 => {
                        // csrrwi
                        if rd != 0 {
                            self.regs[rd] = self.csrr(csr)?;
                        }
                        self.csrw(csr, imm)?;
                    }
                    0b110 => {
                        // csrrsi
                        self.regs[rd] = self.csrr(csr)?;
                        if imm != 0 {
                            self.csrw(csr, self.regs[rd] | imm)?;
                        }
                    }
                    0b111 => {
                        // csrrci
                        self.regs[rd] = self.csrr(csr)?;
                        if imm != 0 {
                            self.csrw(csr, self.regs[rd] & !imm)?;
                        }
                    }
                    _ => {}
                }
            }

            // RV32A
            /*
                The A extension requires that the address held in rs1 be naturally aligned to the size of the operand.
                (i.e., eight-byte aligned for 64-bit words and four-byte aligned for 32-bit words)
                If the address is not naturally aligned, an 'address-misaligned exception' or an 'access-fault exception'
                will be generated. (1.8.2, 1.8.4)
            */
            0b010_1111 => {
                // TODO: if address is not aligned, raise address-misaligned exception
                // R-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let _rl = read_bits(inst, 25..25); // release
                let _aq = read_bits(inst, 26..26); // acquire
                let funct5 = read_bits(inst, 27..31);

                match (funct3, funct5) {
                    (0x2, 0x02) => {
                        // TODO: implement set reserve
                        // lr.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                    }
                    (0x2, 0x03) => {
                        // TODO: implement check reserve
                        // sc.w
                        self.ram.write32(self.regs[rs1], self.regs[rs2])?;
                        self.regs[rd] = 0;
                    }
                    (0x2, 0x01) => {
                        // amoswap.w
                        /*
                            These AMO in-structions atomically load a data value from the address in rs1,
                            place the value into register rd, apply a binary operator to the loaded value
                            and the original value in rs2, then store the result back to the address in rs1. (1.8.4)
                        */
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram.write32(self.regs[rs1], self.regs[rs2])?;

                        /*
                            atomically load a 32-bit signed data value from the address in rs1,
                            place the value into register rd,
                            swap the loaded value and the original 32-bit signed value in rs2,
                            then store the result back to the address in rs1.
                            (https://msyksphinz-self.github.io/riscv-isadoc/html/rva.html#amoswap-w)
                        */
                        // self.regs.swap(rd, rs2);
                    }
                    (0x2, 0x00) => {
                        // amoadd.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram
                            .write32(self.regs[rs1], self.regs[rd].wrapping_add(self.regs[rs2]))?;
                    }
                    (0x2, 0x04) => {
                        // amoxor.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram
                            .write32(self.regs[rs1], self.regs[rd] ^ self.regs[rs2])?;
                    }
                    (0x2, 0x0C) => {
                        // amoand.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram
                            .write32(self.regs[rs1], self.regs[rd] & self.regs[rs2])?;
                    }
                    (0x2, 0x0A) => {
                        // amoor.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram
                            .write32(self.regs[rs1], self.regs[rd] | self.regs[rs2])?;
                    }
                    (0x2, 0x10) => {
                        // amomin.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram.write32(
                            self.regs[rs1],
                            std::cmp::min(self.regs[rd] as i32, self.regs[rs2] as i32) as u32,
                        )?;
                    }
                    (0x2, 0x14) => {
                        // amomax.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram.write32(
                            self.regs[rs1],
                            std::cmp::max(self.regs[rd] as i32, self.regs[rs2] as i32) as u32,
                        )?;
                    }
                    (0x2, 0x18) => {
                        // amominu.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram.write32(
                            self.regs[rs1],
                            std::cmp::min(self.regs[rd], self.regs[rs2]),
                        )?;
                    }
                    (0x2, 0x1C) => {
                        // amomaxu.w
                        self.regs[rd] = self.ram.read32(self.regs[rs1])?;
                        self.ram.write32(
                            self.regs[rs1],
                            std::cmp::max(self.regs[rd], self.regs[rs2]),
                        )?;
                    }
                    _ => {}
                }
            }

            0b000_1111 => {
                // TODO: implement fence if emulator supports a multi-core processor.
                // I-type
                let funct3 = read_bits(inst, 12..14);
                match funct3 {
                    0x0 => {
                        // fence
                    }
                    0x7 => {
                        // fence.i
                    }
                    _ => {}
                }
            }

            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
        // Register x0 is hardwired with all bits equal to 0. (1.2.1)
        self.regs[0] = 0;
        Ok(())
    }
}
