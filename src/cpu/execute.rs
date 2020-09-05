use super::csr;
use crate::bits::*;
use crate::cpu::{Cpu, Mode};
use crate::exception::Exception;
use crate::fpu;

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
                        self.xregs[rd] = self.xregs[rs1].wrapping_add(self.xregs[rs2]);
                    }
                    (0x0, 0x20) => {
                        // sub
                        self.xregs[rd] = self.xregs[rs1].wrapping_sub(self.xregs[rs2]);
                    }
                    (0x4, 0x00) => {
                        // xor
                        self.xregs[rd] = self.xregs[rs1] ^ self.xregs[rs2];
                    }
                    (0x6, 0x00) => {
                        // or
                        self.xregs[rd] = self.xregs[rs1] | self.xregs[rs2];
                    }
                    (0x7, 0x00) => {
                        // and
                        self.xregs[rd] = self.xregs[rs1] & self.xregs[rs2];
                    }
                    /*
                    SLL, SRL, and SRA perform logical left, logical right, and arithmetic right shifts
                    on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2. (1.2.4)
                    */
                    (0x1, 0x00) => {
                        // sll
                        let shamt = self.xregs[rs2] & 0b1_1111;
                        self.xregs[rd] = self.xregs[rs1].wrapping_shl(shamt);
                    }
                    (0x5, 0x00) => {
                        // srl
                        let shamt = self.xregs[rs2] & 0b1_1111;
                        self.xregs[rd] = self.xregs[rs1].wrapping_shr(shamt);
                    }
                    (0x5, 0x20) => {
                        // sra
                        let shamt = self.xregs[rs2] & 0b1_1111;
                        self.xregs[rd] = (self.xregs[rs1] as i32).wrapping_shr(shamt) as u32;
                    }
                    (0x2, 0x00) => {
                        // slt
                        self.xregs[rd] = if (self.xregs[rs1] as i32) < (self.xregs[rs2] as i32) {
                            1
                        } else {
                            0
                        };
                    }
                    (0x3, 0x00) => {
                        // sltu
                        self.xregs[rd] = if self.xregs[rs1] < self.xregs[rs2] {
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
                        self.xregs[rd] = self.xregs[rs1].wrapping_mul(self.xregs[rs2]);
                    }
                    (0x1, 0x01) => {
                        // mulh (signed * signed)
                        self.xregs[rd] = ((self.xregs[rs1] as i32 as i64)
                            .wrapping_mul(self.xregs[rs2] as i32 as i64)
                            >> 32) as u32
                    }
                    (0x2, 0x01) => {
                        // mulhsu (signed rs1 * unsigned rs2)
                        self.xregs[rd] = ((self.xregs[rs1] as i32 as i64)
                            .wrapping_mul(self.xregs[rs2] as u64 as i64)
                            >> 32) as u32;
                    }
                    (0x3, 0x01) => {
                        // mulhu (unsigned * unsigned)
                        self.xregs[rd] = ((self.xregs[rs1] as u64)
                            .wrapping_mul(self.xregs[rs2] as u64)
                            >> 32) as u32;
                    }
                    (0x4, 0x01) => {
                        // div
                        self.xregs[rd] = if self.xregs[rs2] == 0 {
                            // Divide by Zero
                            0xFFFF_FFFF // -1
                        } else {
                            // By using wrapping_*, Overflow case (dividend == -2^31 && divisor == -1) is included.
                            (self.xregs[rs1] as i32).wrapping_div(self.xregs[rs2] as i32) as u32
                        }
                    }
                    (0x5, 0x01) => {
                        // divu
                        self.xregs[rd] = if self.xregs[rs2] == 0 {
                            // Divide by Zero
                            0xFFFF_FFFF // 2^32-1
                        } else {
                            self.xregs[rs1].wrapping_div(self.xregs[rs2])
                        }
                    }
                    (0x6, 0x01) => {
                        // rem
                        self.xregs[rd] = if self.xregs[rs2] == 0 {
                            // Divide by Zero
                            self.xregs[rs1]
                        } else {
                            // By using wrapping_*, Overflow case (dividend == -2^31 && divisor == -1) is included.
                            (self.xregs[rs1] as i32).wrapping_rem(self.xregs[rs2] as i32) as u32
                        }
                    }
                    (0x7, 0x01) => {
                        // remu
                        self.xregs[rd] = if self.xregs[rs2] == 0 {
                            // Divide by Zero
                            self.xregs[rs1]
                        } else {
                            self.xregs[rs1].wrapping_rem(self.xregs[rs2])
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
                        self.xregs[rd] = self.xregs[rs1].wrapping_add(imm);
                    }
                    0x4 => {
                        // xori
                        self.xregs[rd] = self.xregs[rs1] ^ imm;
                    }
                    0x6 => {
                        // ori
                        self.xregs[rd] = self.xregs[rs1] | imm;
                    }
                    0x7 => {
                        // andi
                        self.xregs[rd] = self.xregs[rs1] & imm;
                    }
                    0x1 => {
                        // slli
                        self.xregs[rd] = self.xregs[rs1].wrapping_shl(shamt);
                    }
                    0x2 => {
                        // slti
                        self.xregs[rd] = if (self.xregs[rs1] as i32) < (imm as i32) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // sltiu
                        self.xregs[rd] = if self.xregs[rs1] < imm { 1 } else { 0 };
                    }
                    0x5 => {
                        match funct7 {
                            0x00 => {
                                // srli
                                self.xregs[rd] = self.xregs[rs1].wrapping_shr(shamt);
                            }
                            0x20 => {
                                // srai
                                self.xregs[rd] =
                                    (self.xregs[rs1] as i32).wrapping_shr(shamt) as u32;
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
                let addr = self.xregs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => {
                        // lb
                        let val = self.ram.read8(addr)?;
                        self.xregs[rd] = val as i8 as i32 as u32;
                    }
                    0x1 => {
                        // lh
                        let val = self.ram.read16(addr)?;
                        self.xregs[rd] = val as i16 as i32 as u32;
                    }
                    0x2 => {
                        // lw
                        let val = self.ram.read32(addr)?;
                        self.xregs[rd] = val;
                    }
                    0x4 => {
                        // lbu
                        let val = self.ram.read8(addr)?;
                        self.xregs[rd] = val;
                    }
                    0x5 => {
                        // lhu
                        let val = self.ram.read16(addr)?;
                        self.xregs[rd] = val;
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
                let addr = self.xregs[rs1].wrapping_add(imm);

                match funct3 {
                    0x0 => {
                        // sb
                        self.ram.write8(addr, self.xregs[rs2])?;
                    }
                    0x1 => {
                        // sh
                        self.ram.write16(addr, self.xregs[rs2])?;
                    }
                    0x2 => {
                        // sw
                        self.ram.write32(addr, self.xregs[rs2])?;
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
                        if self.xregs[rs1] == self.xregs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x1 => {
                        // bne
                        if self.xregs[rs1] != self.xregs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x4 => {
                        // blt
                        if (self.xregs[rs1] as i32) < (self.xregs[rs2] as i32) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x5 => {
                        // bge
                        if (self.xregs[rs1] as i32) >= (self.xregs[rs2] as i32) {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x6 => {
                        // bltu
                        if self.xregs[rs1] < self.xregs[rs2] {
                            self.pc = self.pc.wrapping_add(imm).wrapping_sub(4)
                        };
                    }
                    0x7 => {
                        // bgeu
                        if self.xregs[rs1] >= self.xregs[rs2] {
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

                self.xregs[rd] = self.pc;
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }

            0b110_0111 => {
                // I-type
                // jalr
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let imm = ((inst as i32) >> 20) as u32;
                let addr = self.xregs[rs1].wrapping_add(imm);
                if funct3 == 0 {
                    self.xregs[rd] = self.pc;
                    self.pc = addr;
                }
            }

            0b011_0111 => {
                // U-type
                // lui
                let rd = read_bits(inst, 7..11) as usize;
                self.xregs[rd] = inst & 0xFFFF_F000;
            }

            0b001_0111 => {
                // U-type
                // auipc
                let rd = read_bits(inst, 7..11) as usize;
                let imm = inst & 0xFFFF_F000;
                self.xregs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
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
                            0x002 => {
                                // uret
                            }
                            0x102 => {
                                // sret
                                // (3.1.6.4) SRET should also raise an illegal instruction exception when TSR=1 in mstatus
                                let mut mstatus = self.csrr(csr::MSTATUS)?;
                                if read_bit(mstatus, csr::MSTATUS_TSR) != 0 {
                                    return Err(Exception::IllegalInstruction);
                                }
                                if self.mode == Mode::User {
                                    return Err(Exception::IllegalInstruction);
                                }

                                self.pc = self.csrr(csr::SEPC)?;
                                self.mode = match read_bit(mstatus, csr::MSTATUS_SPP) {
                                    0x0 => Mode::User,
                                    0x1 => Mode::Supervisor,
                                    _ => panic!("unknown sstatus.SPP"),
                                };

                                let spie = read_bit(mstatus, csr::MSTATUS_SPIE);
                                write_bit(&mut mstatus, csr::MSTATUS_SIE, spie);
                                write_bit(&mut mstatus, csr::MSTATUS_SPIE, 1);
                                write_bit(&mut mstatus, csr::MSTATUS_SPP, Mode::User as u32);
                            }
                            0x302 => {
                                // mret
                                if self.mode != Mode::Machine {
                                    return Err(Exception::IllegalInstruction);
                                }

                                self.pc = self.csrr(csr::MEPC)?;
                                let mut mstatus = self.csrr(csr::MSTATUS)?;
                                self.mode = match read_bits(
                                    mstatus,
                                    csr::MSTATUS_MPP..csr::MSTATUS_MPP + 1,
                                ) {
                                    0x0 => Mode::User,
                                    0x1 => Mode::Supervisor,
                                    0x3 => Mode::Machine,
                                    _ => panic!("unknown mstatus.MPP"),
                                };

                                let mpie = read_bit(mstatus, csr::MSTATUS_MPIE);
                                write_bit(&mut mstatus, csr::MSTATUS_MIE, mpie);
                                write_bit(&mut mstatus, csr::MSTATUS_MPIE, 1);
                                write_bits(
                                    &mut mstatus,
                                    csr::MSTATUS_MPP..csr::MSTATUS_MPP + 1,
                                    Mode::User as u32,
                                );
                                self.csrw(csr::MSTATUS, mstatus)?;
                            }
                            0x105 => {
                                // wfi
                            }
                            _ => {}
                        }
                    }
                    0b001 => {
                        // csrrw
                        if rd != 0 {
                            self.xregs[rd] = self.csrr(csr)?;
                        }
                        self.csrw(csr, self.xregs[rs1])?;
                    }
                    0b010 => {
                        // csrrs
                        self.xregs[rd] = self.csrr(csr)?;
                        if rs1 != 0 {
                            self.csrw(csr, self.xregs[rd] | self.xregs[rs1])?;
                        }
                    }
                    0b011 => {
                        // csrrc
                        self.xregs[rd] = self.csrr(csr)?;
                        if rs1 != 0 {
                            self.csrw(csr, self.xregs[rd] & !self.xregs[rs1])?;
                        }
                    }
                    0b101 => {
                        // csrrwi
                        if rd != 0 {
                            self.xregs[rd] = self.csrr(csr)?;
                        }
                        self.csrw(csr, imm)?;
                    }
                    0b110 => {
                        // csrrsi
                        self.xregs[rd] = self.csrr(csr)?;
                        if imm != 0 {
                            self.csrw(csr, self.xregs[rd] | imm)?;
                        }
                    }
                    0b111 => {
                        // csrrci
                        self.xregs[rd] = self.csrr(csr)?;
                        if imm != 0 {
                            self.csrw(csr, self.xregs[rd] & !imm)?;
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
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                    }
                    (0x2, 0x03) => {
                        // TODO: implement check reserve
                        // sc.w
                        self.ram.write32(self.xregs[rs1], self.xregs[rs2])?;
                        self.xregs[rd] = 0;
                    }
                    (0x2, 0x01) => {
                        // amoswap.w
                        /*
                            These AMO in-structions atomically load a data value from the address in rs1,
                            place the value into register rd, apply a binary operator to the loaded value
                            and the original value in rs2, then store the result back to the address in rs1. (1.8.4)
                        */
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram.write32(self.xregs[rs1], self.xregs[rs2])?;

                        /*
                            atomically load a 32-bit signed data value from the address in rs1,
                            place the value into register rd,
                            swap the loaded value and the original 32-bit signed value in rs2,
                            then store the result back to the address in rs1.
                            (https://msyksphinz-self.github.io/riscv-isadoc/html/rva.html#amoswap-w)
                        */
                        // self.xregs.swap(rd, rs2);
                    }
                    (0x2, 0x00) => {
                        // amoadd.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram.write32(
                            self.xregs[rs1],
                            self.xregs[rd].wrapping_add(self.xregs[rs2]),
                        )?;
                    }
                    (0x2, 0x04) => {
                        // amoxor.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram
                            .write32(self.xregs[rs1], self.xregs[rd] ^ self.xregs[rs2])?;
                    }
                    (0x2, 0x0C) => {
                        // amoand.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram
                            .write32(self.xregs[rs1], self.xregs[rd] & self.xregs[rs2])?;
                    }
                    (0x2, 0x0A) => {
                        // amoor.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram
                            .write32(self.xregs[rs1], self.xregs[rd] | self.xregs[rs2])?;
                    }
                    (0x2, 0x10) => {
                        // amomin.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram.write32(
                            self.xregs[rs1],
                            std::cmp::min(self.xregs[rd] as i32, self.xregs[rs2] as i32) as u32,
                        )?;
                    }
                    (0x2, 0x14) => {
                        // amomax.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram.write32(
                            self.xregs[rs1],
                            std::cmp::max(self.xregs[rd] as i32, self.xregs[rs2] as i32) as u32,
                        )?;
                    }
                    (0x2, 0x18) => {
                        // amominu.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram.write32(
                            self.xregs[rs1],
                            std::cmp::min(self.xregs[rd], self.xregs[rs2]),
                        )?;
                    }
                    (0x2, 0x1C) => {
                        // amomaxu.w
                        self.xregs[rd] = self.ram.read32(self.xregs[rs1])?;
                        self.ram.write32(
                            self.xregs[rs1],
                            std::cmp::max(self.xregs[rd], self.xregs[rs2]),
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

            // RV32FD
            0b101_0011 => {
                // R-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let fmt = read_bits(inst, 25..26);
                let funct5 = read_bits(inst, 27..31);

                /*
                    A value of 111 in the instruction’s rm field selects the dynamic rounding mode held in frm.
                    If frm is set to an invalid value (101–111), any subsequent attempt to execute a floating-point operation
                    with a dynamic rounding mode will raise an illegal instruction exception. (1.11.2)
                */
                match funct5 {
                    0x00 => {
                        if fmt == fpu::FP32 {
                            // fadd.s
                            self.fregs[rd] = fpu::fadd_32(
                                self.fregs[rs1] as f32,
                                self.fregs[rs2] as f32,
                                funct3,
                            )? as f64;
                        } else if fmt == fpu::FP64 {
                            // fadd.d
                            self.fregs[rd] =
                                fpu::fadd_64(self.fregs[rs1], self.fregs[rs2], funct3)?;
                        }
                    }
                    0x01 => {
                        if fmt == fpu::FP32 {
                            // fsub.s
                            self.fregs[rd] = fpu::fsub_32(
                                self.fregs[rs1] as f32,
                                self.fregs[rs2] as f32,
                                funct3,
                            )? as f64;
                        } else if fmt == fpu::FP64 {
                            // fsub.d
                            self.fregs[rd] =
                                fpu::fsub_64(self.fregs[rs1], self.fregs[rs2], funct3)?;
                        }
                    }
                    0x02 => {
                        if fmt == fpu::FP32 {
                            // fmul.s
                            self.fregs[rd] = fpu::fmul_32(
                                self.fregs[rs1] as f32,
                                self.fregs[rs2] as f32,
                                funct3,
                            )? as f64;
                        } else if fmt == fpu::FP64 {
                            // fmul.d
                            self.fregs[rd] =
                                fpu::fmul_64(self.fregs[rs1], self.fregs[rs2], funct3)?;
                        }
                    }
                    0x03 => {
                        if fmt == fpu::FP32 {
                            // fdiv.s
                            self.fregs[rd] = fpu::fdiv_32(
                                self.fregs[rs1] as f32,
                                self.fregs[rs2] as f32,
                                funct3,
                            )? as f64;
                        } else if fmt == fpu::FP64 {
                            // fdiv.d
                            self.fregs[rd] =
                                fpu::fdiv_64(self.fregs[rs1], self.fregs[rs2], funct3)?;
                        }
                    }
                    0x04 => {
                        // fsgnj
                        // fsgnjn
                        // fsgnjx
                        if fmt == fpu::FP32 {
                            self.fregs[rd] = fpu::fsgnj_32(
                                self.fregs[rs1] as f32,
                                self.fregs[rs2] as f32,
                                funct3,
                            )? as f64;
                        } else if fmt == fpu::FP64 {
                            self.fregs[rd] =
                                fpu::fsgnj_64(self.fregs[rs1], self.fregs[rs2], funct3)?;
                        }
                    }
                    0x05 => {
                        match funct3 {
                            0b000 => {
                                // fmin
                                if fmt == fpu::FP32 {
                                    self.fregs[rd] =
                                        fpu::fmin(self.fregs[rs1] as f32, self.fregs[rs2] as f32)
                                            as f64;
                                } else if fmt == fpu::FP64 {
                                    self.fregs[rd] = fpu::fmin(self.fregs[rs1], self.fregs[rs2]);
                                }
                            }
                            0b001 => {
                                // fmax
                                if fmt == fpu::FP32 {
                                    self.fregs[rd] =
                                        fpu::fmax(self.fregs[rs1] as f32, self.fregs[rs2] as f32)
                                            as f64;
                                } else if fmt == fpu::FP64 {
                                    self.fregs[rd] = fpu::fmax(self.fregs[rs1], self.fregs[rs2]);
                                }
                            }
                            _ => {}
                        }
                    }
                    0x0B => {
                        if fmt == fpu::FP32 {
                            // fsqrt.s
                            self.fregs[rd] = fpu::fsqrt_32(self.fregs[rs1] as f32, funct3)? as f64;
                        } else if fmt == fpu::FP64 {
                            // fsqrt.d
                            self.fregs[rd] = fpu::fsqrt_64(self.fregs[rs1], funct3)?;
                        }
                    }
                    0x08 => {
                        match rs2 {
                            0x0 => {
                                // fcvt.d.s
                                self.fregs[rd] = self.fregs[rs1];
                            }
                            0x1 => {
                                // fcvt.s.d
                                self.fregs[rd] = fpu::fcvt_s_d(self.fregs[rs1], funct3)? as f64;
                            }
                            _ => {}
                        }
                    }
                    0x18 => {
                        match rs2 {
                            0x0 => {
                                if fmt == fpu::FP32 {
                                    // fcvt.w.s
                                    self.xregs[rd] =
                                        fpu::fcvt_w_s(self.fregs[rs1] as f32, funct3)? as u32;
                                } else if fmt == fpu::FP64 {
                                    // fcvt.w.d
                                    self.xregs[rd] = fpu::fcvt_w_d(self.fregs[rs1], funct3)? as u32;
                                }
                            }
                            0x1 => {
                                if fmt == fpu::FP32 {
                                    // fcvt.wu.s
                                    self.xregs[rd] =
                                        fpu::fcvt_wu_s(self.fregs[rs1] as f32, funct3)?;
                                } else if fmt == fpu::FP64 {
                                    // fcvt.wu.d
                                    self.xregs[rd] = fpu::fcvt_wu_d(self.fregs[rs1], funct3)?;
                                }
                            }
                            _ => {}
                        }
                    }
                    0x1A => {
                        match rs2 {
                            0x0 => {
                                if fmt == fpu::FP32 {
                                    // fcvt.s.w
                                    self.fregs[rd] =
                                        fpu::fcvt_s_w(self.xregs[rs1] as i32, funct3)? as f64;
                                } else if fmt == fpu::FP64 {
                                    // fcvt.d.w
                                    self.fregs[rd] = fpu::fcvt_d_w(self.xregs[rs1] as i32, funct3)?;
                                }
                            }
                            0x1 => {
                                if fmt == fpu::FP32 {
                                    // fcvt.s.wu
                                    self.fregs[rd] =
                                        fpu::fcvt_s_wu(self.xregs[rs1], funct3)? as f64;
                                } else if fmt == fpu::FP64 {
                                    // fcvt.d.wu
                                    self.fregs[rd] = fpu::fcvt_d_wu(self.xregs[rs1], funct3)?;
                                }
                            }
                            _ => {}
                        }
                    }
                    0x14 => {
                        match funct3 {
                            0b000 => {
                                // fle
                                if fmt == fpu::FP32 {
                                    self.xregs[rd] =
                                        fpu::fle(self.fregs[rs1] as f32, self.fregs[rs2] as f32);
                                } else if fmt == fpu::FP64 {
                                    self.xregs[rd] = fpu::fle(self.fregs[rs1], self.fregs[rs2]);
                                }
                            }
                            0b001 => {
                                // flt
                                if fmt == fpu::FP32 {
                                    self.xregs[rd] =
                                        fpu::flt(self.fregs[rs1] as f32, self.fregs[rs2] as f32);
                                } else if fmt == fpu::FP64 {
                                    self.xregs[rd] = fpu::flt(self.fregs[rs1], self.fregs[rs2]);
                                }
                            }
                            0b010 => {
                                // feq
                                if fmt == fpu::FP32 {
                                    self.xregs[rd] =
                                        fpu::feq(self.fregs[rs1] as f32, self.fregs[rs2] as f32);
                                } else if fmt == fpu::FP64 {
                                    self.xregs[rd] = fpu::feq(self.fregs[rs1], self.fregs[rs2]);
                                }
                            }
                            _ => {}
                        }
                    }
                    0x1C => {
                        match funct3 {
                            0b000 => {
                                // fmv.x.w
                                self.xregs[rd] = (self.fregs[rs1] as f32).to_bits();
                            }
                            0b001 => {
                                // fclass
                                if fmt == fpu::FP32 {
                                    self.xregs[rd] = fpu::fclass_32(self.fregs[rs1] as f32);
                                } else if fmt == fpu::FP64 {
                                    self.xregs[rd] = fpu::fclass_64(self.fregs[rs1]);
                                }
                            }
                            _ => {}
                        }
                    }
                    0x1E => {
                        // fmv.w.x
                        self.fregs[rd] = f32::from_bits(self.xregs[rs1]) as f64;
                    }

                    _ => {}
                }
            }

            0b100_0011 => {
                // R4-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let fmt = read_bits(inst, 25..26);
                let rs3 = read_bits(inst, 27..31) as usize;

                if fmt == fpu::FP32 {
                    // fmadd.s
                    self.fregs[rd] = fpu::fmadd_32(
                        self.fregs[rs1] as f32,
                        self.fregs[rs2] as f32,
                        self.fregs[rs3] as f32,
                        funct3,
                    )? as f64;
                } else if fmt == fpu::FP64 {
                    // fmadd.d
                    self.fregs[rd] =
                        fpu::fmadd_64(self.fregs[rs1], self.fregs[rs2], self.fregs[rs3], funct3)?;
                }
            }

            0b100_0111 => {
                // R4-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let fmt = read_bits(inst, 25..26);
                let rs3 = read_bits(inst, 27..31) as usize;

                if fmt == fpu::FP32 {
                    // fmsub.s
                    self.fregs[rd] = fpu::fmadd_32(
                        self.fregs[rs1] as f32,
                        self.fregs[rs2] as f32,
                        -self.fregs[rs3] as f32,
                        funct3,
                    )? as f64;
                } else if fmt == fpu::FP64 {
                    // fmsub.d
                    self.fregs[rd] =
                        fpu::fmadd_64(self.fregs[rs1], self.fregs[rs2], -self.fregs[rs3], funct3)?;
                }
            }

            0b100_1011 => {
                // R4-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let fmt = read_bits(inst, 25..26);
                let rs3 = read_bits(inst, 27..31) as usize;

                if fmt == fpu::FP32 {
                    // fnmsub.s
                    self.fregs[rd] = fpu::fmadd_32(
                        -self.fregs[rs1] as f32,
                        self.fregs[rs2] as f32,
                        self.fregs[rs3] as f32,
                        funct3,
                    )? as f64;
                } else if fmt == fpu::FP64 {
                    // fnmsub.d
                    self.fregs[rd] =
                        fpu::fmadd_64(-self.fregs[rs1], self.fregs[rs2], self.fregs[rs3], funct3)?;
                }
            }

            0b100_1111 => {
                // R4-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let fmt = read_bits(inst, 25..26);
                let rs3 = read_bits(inst, 27..31) as usize;

                if fmt == fpu::FP32 {
                    // fnmadd.s
                    self.fregs[rd] = fpu::fmadd_32(
                        -self.fregs[rs1] as f32,
                        self.fregs[rs2] as f32,
                        -self.fregs[rs3] as f32,
                        funct3,
                    )? as f64;
                } else if fmt == fpu::FP64 {
                    // fnmadd.d
                    self.fregs[rd] =
                        fpu::fmadd_64(-self.fregs[rs1], self.fregs[rs2], -self.fregs[rs3], funct3)?;
                }
            }

            0b000_0111 => {
                // I-type
                let rd = read_bits(inst, 7..11) as usize;
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let offset = read_bits(inst, 20..31);
                match funct3 {
                    0b010 => {
                        // flw
                        self.fregs[rd] =
                            f32::from_bits(self.ram.read32(self.xregs[rs1].wrapping_add(offset))?)
                                as f64;
                    }
                    0b011 => {
                        // fld
                        self.fregs[rd] =
                            f64::from_bits(self.ram.read64(self.xregs[rs1].wrapping_add(offset))?);
                    }
                    _ => {}
                }
            }

            0b010_0111 => {
                // fsw
                // S-type
                let imm1 = read_bits(inst, 7..11);
                let funct3 = read_bits(inst, 12..14);
                let rs1 = read_bits(inst, 15..19) as usize;
                let rs2 = read_bits(inst, 20..24) as usize;
                let imm2 = read_bits(inst, 25..31);
                let imm = imm2 << 5 | imm1;

                match funct3 {
                    0b010 => {
                        // fsw
                        self.ram
                            .write32(self.xregs[rs1] + imm, (self.fregs[rs2] as f32).to_bits())?;
                    }
                    0b011 => {
                        // fsd
                        self.ram
                            .write64(self.xregs[rs1] + imm, self.fregs[rs2].to_bits())?;
                    }
                    _ => {}
                }
            }

            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
        // Register x0 is hardwired with all bits equal to 0. (1.2.1)
        self.xregs[0] = 0;
        Ok(())
    }
}
