use crate::{cpu::CPU, ram::RAM, reg::{GPRs, Mem, REX}};

#[derive(PartialEq, Eq)]
pub enum ModRMType {
    MemReg,
    RegReg,
}

pub struct ModRM {
    pub addr: Mem,
    pub reg1: u8,
    pub reg2: Option<u8>,
    pub rmtype: ModRMType,
}

pub enum GPRSize {
    X16(u16),
    X32(u32),
    X64(u64),
}

impl CPU {
    // fn instr_is_64bit(&self) -> bool {
    //     // self.ir_data
    // }

    #[allow(clippy::identity_op)]
    fn decode_sib_64(&mut self, ram: &mut RAM) -> u64 {
        let sib = self.read_u8(ram);

        let ss   = (sib & 0b1100_0000) >> 6;
        let idx  = (sib & 0b0011_1000) >> 3;
        let base = (sib & 0b0000_0111) >> 0;

        if base == 5 && ss == 0 {
            return self.read_u32(ram).into();
        }

        let highbyte1: u8 = self.ir_data.rex.contains(REX::B).into();
        let highbyte2: u8 = self.ir_data.rex.contains(REX::X).into();

        let reg1 = (highbyte1 << 3) | base;
        let reg2 = (highbyte2 << 3) | idx;

        let gpr1: GPRs = reg1.try_into().unwrap();
        let gpr2: GPRs = reg2.try_into().unwrap();

        let mul = match ss {
            0 => 0,
            1 => 2,
            2 => 4,
            3 => 8,
            _ => unreachable!(),
        };

        let addr = unsafe { if self.ir_data.rex.contains(REX::B) { self[gpr1].ur } else { self[gpr1].ud.into() } };
        if idx == 5 {
            addr
        } else {
            addr + unsafe { if self.ir_data.rex.contains(REX::X) { self[gpr2].ur } else { self[gpr2].ud.into() } } * mul
        }
    }

    #[allow(clippy::identity_op)]
    fn decode_modrm_64(&mut self, ram: &mut RAM) -> ModRM {
        let modrm = self.read_u8(ram);

        let md  = (modrm & 0b1100_0000) >> 6;
        let reg = (modrm & 0b0011_1000) >> 3;
        let rm  = (modrm & 0b0000_0111) >> 0;

        let highbyte1: u8 = self.ir_data.rex.contains(REX::R).into();
        let highbyte2: u8 = self.ir_data.rex.contains(REX::B).into();

        let reg1 = (highbyte1 << 3) | reg;
        let reg2 = (highbyte2 << 3) | rm;

        if md == 3 {
            return ModRM {
                addr: Mem { r: 0 },
                reg1,
                reg2: Some(reg2),
                rmtype: ModRMType::RegReg,
            };
        }

        let gpr: GPRs = reg2.try_into().unwrap();
        let mut addr = unsafe { if self.ir_data.rex.contains(REX::B) { self[gpr].ur } else { self[gpr].ud.into() } };

        if rm == 4 {
            addr = self.decode_sib_64(ram);
        } else if rm == 5 && md == 1 {
            addr = unsafe { if self.ir_data.rex.contains(REX::B) { self.ip.0.r } else { self.ip.0.d.into() } };
        }

        if md == 1 {
            let disp: u64 = self.read_u8(ram).into();
            addr += disp;
        } else if md == 2 {
            let disp: u64 = self.read_u32(ram).into();
            addr += disp;
        }

        ModRM {
            addr: Mem { r: addr },
            reg1,
            reg2: None,
            rmtype: ModRMType::MemReg,
        }
    }

    pub fn decode_modrm(&mut self, ram: &mut RAM) -> ModRM {
        self.decode_modrm_64(ram)
    }

    fn decode_gpr_size_64(&mut self, gpr_idx: u8) -> GPRSize {
        let gpr: GPRs = gpr_idx.try_into().unwrap();
        if self.ir_data.oper {
            return unsafe { GPRSize::X16(self[gpr].uw) };
        }
        if self.ir_data.rex.contains(REX::W) {
            return unsafe { GPRSize::X64(self[gpr].ur) };
        }

        unsafe { GPRSize::X32(self[gpr].ud) }
    }

    pub fn decode_gpr_size(&mut self, gpr_idx: u8) -> GPRSize {
        self.decode_gpr_size_64(gpr_idx)
    }

    fn decode_mem_size_64(&mut self, ram: &mut RAM, addr: Mem) -> GPRSize {
        let virt_addr: u64 = if self.ir_data.addr {
            unsafe { addr.w.into() }
        } else if self.ir_data.rex.contains(REX::W) {
            unsafe { addr.r }
        } else {
            unsafe { addr.d.into() }
        };

        if self.ir_data.oper {
            GPRSize::X16(self.get_u16(ram, virt_addr))
        } else if self.ir_data.rex.contains(REX::W) {
            GPRSize::X64(self.get_u64(ram, virt_addr))
        } else {
            GPRSize::X32(self.get_u32(ram, virt_addr))
        }
    }

    pub fn decode_mem_size(&mut self, ram: &mut RAM, addr: Mem) -> GPRSize {
        self.decode_mem_size_64(ram, addr)
    }

    pub fn decode_rm_gpr(&mut self, ram: &mut RAM, modrm: &ModRM) -> GPRSize {
        if modrm.rmtype == ModRMType::RegReg {
            modrm.reg2.map_or_else(|| unreachable!(), |reg| {
                self.decode_gpr_size(reg)
            })
        } else {
            self.decode_mem_size(ram, modrm.addr)
        }
    }
}
