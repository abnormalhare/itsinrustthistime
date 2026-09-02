use crate::{cpu::CPU, ram::RAM, reg::{GPR, Mem, Register}};

pub enum ModRMType {
    MemReg,
    RegReg,
}

pub struct ModRM<'a> {
    addr: Mem,
    reg1: &'a Register,
    reg2: &'a Register,
    order: ModRMType,
}

impl CPU {
    // fn instr_is_64bit(&self) -> bool {
    //     // self.ir_data
    // }

    pub fn decode_modrm(&mut self, ram: &mut RAM) -> ModRM<'_> {
        self.read_instr(ram);
        let modrm = self.ir;

        let md  = (self.ir & 0b1100_0000) >> 6;
        let rm  = (self.ir & 0b0011_1000) >> 3;
        let reg = (self.ir & 0b0000_0111) >> 0;

        let order = if md == 3 { ModRMType::RegReg } else { ModRMType::MemReg };

        // if self.

        ModRM {
            addr: Mem { r: 0 },
            reg1: &Register::GPR(GPR { sr: 0 }),
            reg2: &Register::GPR(GPR { sr: 0 }),
            order: ModRMType::MemReg
        }
    }
}
