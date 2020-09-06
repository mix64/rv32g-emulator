use super::csr::*;
use crate::bits::*;
use crate::cpu::{Cpu, Mode};
use crate::exception::Exception;
use crate::memory::MemOps;

const SATP_SV32: u32 = 0x8000_0000;
const SATP_PPN: u32 = 0x003F_FFFF;

const PTE_V: u32 = 0x0000_00001;
const PTE_R: u32 = 0x0000_00002;
const PTE_W: u32 = 0x0000_00004;
const PTE_X: u32 = 0x0000_00008;
const PTE_U: u32 = 0x0000_00010;
// const PTE_G: u32 = 0x0000_00020;
// const PTE_A: u32 = 0x0000_00040;
// const PTE_D: u32 = 0x0000_00080;
const PTE_PPN: u32 = 0xFFFF_FC00;

fn page_fault(ops: MemOps) -> Exception {
    match ops {
        MemOps::Load => Exception::LoadPageFault,
        MemOps::Store => Exception::StoreAMOPageFault,
        MemOps::Fetch => Exception::InstructionPageFault,
    }
}

impl Cpu {
    fn walkpgdir(&self, satp: u32, va: u32, ops: MemOps) -> Result<u32, Exception> {
        let pde_pa = (satp & SATP_PPN).wrapping_shr(12) + read_bits(va, 22..31).wrapping_shr(2);
        let pde = self.ram.read32(pde_pa)?;
        if pde & PTE_V == 0 {
            return Err(page_fault(ops));
        }
        // 4.3.1
        // When all R/W/X are zero, the PTE is a pointer to the next level of the page table.
        if pde & (PTE_R | PTE_W | PTE_X) != 0 {
            return Err(page_fault(ops));
        }

        let pte_pa = (pde & PTE_PPN).wrapping_shr(2) + read_bits(va, 12..21).wrapping_shr(2);
        let pte = self.ram.read32(pte_pa)?;

        if pte & PTE_V == 0 {
            return Err(page_fault(ops));
        }
        // 4.3.1
        // U-mode software may only access the page when U=1.
        // If the SUM bit in the sstatus register is set, supervisor mode software may also access pages with U=1.
        if pte & PTE_U != 0
            && self.mode == Mode::Supervisor
            && self.csrr(SSTATUS)? & MSTATUS_SUM == 0
        {
            return Err(page_fault(ops));
        }
        if pte & PTE_U == 0 && self.mode == Mode::User {
            return Err(page_fault(ops));
        }
        match ops {
            MemOps::Load => {
                if pte & PTE_R == 0 {
                    return Err(page_fault(ops));
                }
            }
            MemOps::Store => {
                if pte & PTE_W == 0 {
                    return Err(page_fault(ops));
                }
            }
            MemOps::Fetch => {
                if pte & PTE_R == 0 || pte & PTE_X == 0 {
                    return Err(page_fault(ops));
                }
            }
        }

        Ok((pte & PTE_PPN).wrapping_shr(2) + read_bits(va, 0..11).wrapping_shr(2))
    }

    pub fn vm_fetch(&self, addr: u32) -> Result<u32, Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Fetch)?;
            return self.ram.fetch(pa);
        } else {
            return self.ram.fetch(addr);
        }
    }

    pub fn vm_read8(&self, addr: u32) -> Result<u32, Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Load)?;
            return self.ram.read8(pa);
        } else {
            return self.ram.read8(addr);
        }
    }

    pub fn vm_read16(&self, addr: u32) -> Result<u32, Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Load)?;
            return self.ram.read16(pa);
        } else {
            return self.ram.read16(addr);
        }
    }

    pub fn vm_read32(&self, addr: u32) -> Result<u32, Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Load)?;
            return self.ram.read32(pa);
        } else {
            return self.ram.read32(addr);
        }
    }

    pub fn vm_read64(&self, addr: u32) -> Result<u64, Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Load)?;
            return self.ram.read64(pa);
        } else {
            return self.ram.read64(addr);
        }
    }

    pub fn vm_write8(&mut self, addr: u32, val: u8) -> Result<(), Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Store)?;
            return self.ram.write8(pa, val);
        } else {
            return self.ram.write8(addr, val);
        }
    }

    pub fn vm_write16(&mut self, addr: u32, val: u16) -> Result<(), Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Store)?;
            return self.ram.write16(pa, val);
        } else {
            return self.ram.write16(addr, val);
        }
    }

    pub fn vm_write32(&mut self, addr: u32, val: u32) -> Result<(), Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Store)?;
            return self.ram.write32(pa, val);
        } else {
            return self.ram.write32(addr, val);
        }
    }

    pub fn vm_write64(&mut self, addr: u32, val: u64) -> Result<(), Exception> {
        let satp = self.csrr(SATP)?;
        if satp & SATP_SV32 != 0 {
            // paging on
            let pa = self.walkpgdir(satp, addr, MemOps::Store)?;
            return self.ram.write64(pa, val);
        } else {
            return self.ram.write64(addr, val);
        }
    }
}
