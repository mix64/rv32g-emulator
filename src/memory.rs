use crate::exception::Exception;

pub const MEMORY_SIZE: u32 = 1024 * 1024 * 128;

pub struct Memory {
    pub ram: Vec<u8>,
}

pub enum MemOps {
    Load,
    Store,
    Fetch,
}

fn chk_address(start: u32, size: u32, ops: MemOps) -> Result<(), Exception> {
    if start + size >= MEMORY_SIZE {
        match ops {
            MemOps::Load => return Err(Exception::LoadAccessFault),
            MemOps::Store => return Err(Exception::StoreAMOAccessFault),
            MemOps::Fetch => return Err(Exception::InstructionAccessFault),
        }
    }

    if start % size != 0 {
        match ops {
            MemOps::Load => return Err(Exception::LoadAddressMisaligned),
            MemOps::Store => return Err(Exception::StoreAMOAddressMisaligned),
            MemOps::Fetch => return Err(Exception::InstructionAddressMisaligned),
        }
    }
    Ok(())
}

impl Memory {
    pub fn new() -> Memory {
        Self {
            ram: vec![0; MEMORY_SIZE as usize],
        }
    }

    pub fn set(&mut self, binary: &Vec<u8>) {
        self.ram.splice(..binary.len(), binary.iter().cloned());
    }

    pub fn fetch(&self, addr: u32) -> Result<u32, Exception> {
        chk_address(addr, 4, MemOps::Fetch)?;
        return self.read32(addr);
    }

    pub fn read8(&self, addr: u32) -> Result<u32, Exception> {
        chk_address(addr, 1, MemOps::Load)?;
        Ok(self.ram[addr as usize] as u32)
    }

    pub fn read16(&self, addr: u32) -> Result<u32, Exception> {
        chk_address(addr, 2, MemOps::Load)?;
        let index = addr as usize;
        Ok((self.ram[index] as u32) | ((self.ram[index + 1] as u32) << 8))
    }

    pub fn read32(&self, addr: u32) -> Result<u32, Exception> {
        chk_address(addr, 4, MemOps::Load)?;
        let index = addr as usize;
        Ok((self.ram[index] as u32)
            | ((self.ram[index + 1] as u32) << 8)
            | ((self.ram[index + 2] as u32) << 16)
            | ((self.ram[index + 3] as u32) << 24))
    }

    pub fn read64(&self, addr: u32) -> Result<u64, Exception> {
        chk_address(addr, 8, MemOps::Load)?;
        let index = addr as usize;
        Ok((self.ram[index] as u64)
            | ((self.ram[index + 1] as u64) << 8)
            | ((self.ram[index + 2] as u64) << 16)
            | ((self.ram[index + 3] as u64) << 24)
            | ((self.ram[index + 4] as u64) << 32)
            | ((self.ram[index + 5] as u64) << 40)
            | ((self.ram[index + 6] as u64) << 48)
            | ((self.ram[index + 7] as u64) << 56))
    }

    pub fn write8(&mut self, addr: u32, val: u8) -> Result<(), Exception> {
        chk_address(addr, 1, MemOps::Store)?;
        let index = addr as usize;
        self.ram[index] = val;
        Ok(())
    }

    pub fn write16(&mut self, addr: u32, val: u16) -> Result<(), Exception> {
        chk_address(addr, 2, MemOps::Store)?;
        let index = addr as usize;
        self.ram[index] = (val & 0xff) as u8;
        self.ram[index + 1] = ((val >> 8) & 0xff) as u8;
        Ok(())
    }

    pub fn write32(&mut self, addr: u32, val: u32) -> Result<(), Exception> {
        chk_address(addr, 4, MemOps::Store)?;
        let index = addr as usize;
        self.ram[index] = (val & 0xff) as u8;
        self.ram[index + 1] = ((val >> 8) & 0xff) as u8;
        self.ram[index + 2] = ((val >> 16) & 0xff) as u8;
        self.ram[index + 3] = ((val >> 24) & 0xff) as u8;
        Ok(())
    }

    pub fn write64(&mut self, addr: u32, val: u64) -> Result<(), Exception> {
        chk_address(addr, 8, MemOps::Store)?;
        let index = addr as usize;
        self.ram[index] = (val & 0xff) as u8;
        self.ram[index + 1] = ((val >> 8) & 0xff) as u8;
        self.ram[index + 2] = ((val >> 16) & 0xff) as u8;
        self.ram[index + 3] = ((val >> 24) & 0xff) as u8;
        self.ram[index + 4] = ((val >> 32) & 0xff) as u8;
        self.ram[index + 5] = ((val >> 40) & 0xff) as u8;
        self.ram[index + 6] = ((val >> 48) & 0xff) as u8;
        self.ram[index + 7] = ((val >> 56) & 0xff) as u8;
        Ok(())
    }
}
