use super::csr::*;
use crate::bits::*;
use crate::cpu::{Cpu, Mode};
use crate::exception::*;

impl Cpu {
    pub fn trap(&mut self, e: Exception) {
        let ecode = e.exception_code();

        let mode = if self.mode == Mode::User {
            if self.csrs[MEDELEG] & (1 << ecode) != 0 {
                if self.csrs[SEDELEG] & (1 << ecode) != 0 {
                    Mode::User
                } else {
                    Mode::Supervisor
                }
            } else {
                Mode::Machine
            }
        } else if self.mode == Mode::Supervisor {
            if self.csrs[MEDELEG] & (1 << ecode) != 0 {
                Mode::Supervisor
            } else {
                Mode::Machine
            }
        } else {
            Mode::Machine
        };

        match mode {
            Mode::Machine => {
                self.csrs[MEPC] = self.pc - 4;
                self.pc = match self.csrs[MTVEC] & 0b11 {
                    0 => self.csrs[MTVEC] & !0b11,
                    1 => (self.csrs[MTVEC] & !0b11) + 4 * ecode,
                    _ => panic!("unknown CSR.mtvec MODE"),
                };
                self.csrs[MCAUSE] = ecode;
                // TODO: Set a correct value to mtval
                self.csrs[MTVAL] = 0;
                let mpie = read_bit(self.csrs[MSTATUS], MSTATUS_MIE);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_MIE, 0);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_MPIE, mpie);
                write_bits(
                    &mut self.csrs[MSTATUS],
                    MSTATUS_MPP..MSTATUS_MPP + 1,
                    self.mode as u32,
                );
            }

            Mode::Supervisor => {
                self.csrs[SEPC] = self.pc - 4;
                self.pc = match self.csrs[STVEC] & 0b11 {
                    0 => self.csrs[STVEC] & !0b11,
                    1 => (self.csrs[STVEC] & !0b11) + 4 * ecode,
                    _ => panic!("unknown CSR.stvec MODE"),
                };
                self.csrs[SCAUSE] = ecode;
                // TODO: Set a correct value to stval
                self.csrs[STVAL] = 0;
                let spie = read_bit(self.csrs[MSTATUS], MSTATUS_SIE);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_SIE, 0);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_SPIE, spie);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_SPP, self.mode as u32);
            }

            Mode::User => {
                self.csrs[UEPC] = self.pc - 4;
                self.pc = match self.csrs[UTVEC] & 0b11 {
                    0 => self.csrs[UTVEC] & !0b11,
                    1 => (self.csrs[UTVEC] & !0b11) + 4 * ecode,
                    _ => panic!("unknown CSR.utvec MODE"),
                };
                self.csrs[UCAUSE] = ecode;
                // TODO: Set a correct value to utval
                self.csrs[UTVAL] = 0;
                let upie = read_bit(self.csrs[MSTATUS], MSTATUS_UIE);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_UIE, 0);
                write_bit(&mut self.csrs[MSTATUS], MSTATUS_UPIE, upie);
            }
        }

        self.mode = mode;

        // println!(
        //     "Trap at [{:08x}] {:08x} (excode: {:})",
        //     self.csrs.mepc, self.csrs.mtval, ecode
        // );
        // println!("Jump to [{:08x}]\n", self.pc);
    }
}
