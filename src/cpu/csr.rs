use crate::exception::Exception;

mod address;
mod mstatus;

use address::*;
use mstatus::*;

pub struct CSRs {
    mstatus: u32,
}

impl CSRs {
    pub fn new() -> Self {
        CSRs { mstatus: 0 }
    }

    pub fn csrrd(&self, src: u16) -> Result<u32, Exception> {
        match src {
            MSTATUS => {
                return Ok(self.mstatus);
            }
            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
    }

    pub fn csrwr(&mut self, dst: u16, imm: u32) -> Result<(), Exception> {
        match dst {
            MSTATUS => {
                // TODO: Check imm
                self.mstatus = imm;
                return Ok(());
            }
            _ => {
                return Err(Exception::IllegalInstruction);
            }
        }
    }
}
