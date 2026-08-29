use crate::{cpu::CPU, ram::RAM, reg::{Mem, Register}};

pub enum ModRMType {
    MEMREG,
    REGREG,
}

pub struct ModRM<'a> {
    addr: Mem,
    reg: &'a Register,
    order: ModRMType,
}

impl CPU {
    pub fn decode_modrm(&mut self, ram: &mut RAM) -> ModRM<'_> {
        self.read_instr(ram);
        self.read_instr(ram);
        let modrm = self.ir;
        println!("{}", modrm);

        ModRM { addr: Mem { r: 0 }, reg: &Register::GPR(crate::reg::GPR { sr: 0 }), order: ModRMType::MEMREG }
    }
}
