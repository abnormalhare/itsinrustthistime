use crate::{cpu::{CPU, decode::RM::{Addr, Reg}}, ram::RAM, reg::{GPRs, Mem, REX}};

#[derive(Clone, Copy)]
pub struct MemoryInfo {
    pub addr: Mem,
    pub base: Option<u8>,
    pub idx: Option<u8>,
    pub off: Option<u32>,
}

#[derive(Clone, Copy)]
pub enum RM {
    Addr(MemoryInfo),
    Reg(u8),
}

pub struct ModRM {
    pub reg1: u8,
    pub reg2: RM,
}

#[derive(Clone, Copy)]
pub enum GPRSize {
    X16(u16),
    X32(u32),
    X64(u64),
}

#[derive(PartialEq, Eq)]
pub enum Location {
    Address,
    Operand,
}

impl CPU {
    // fn instr_is_64bit(&self) -> bool {
    //     // self.ir_data
    // }

    #[allow(clippy::identity_op)]
    fn decode_sib_64(&mut self, ram: &mut RAM) -> MemoryInfo {
        let sib = self.read_u8(ram);

        let ss   = (sib & 0b1100_0000) >> 6;
        let idx  = (sib & 0b0011_1000) >> 3;
        let base = (sib & 0b0000_0111) >> 0;

        if base == 5 && ss == 0 {
            let off = self.read_u32(ram);
            return MemoryInfo {
                addr: Mem { d: off },
                base: None,
                idx: None,
                off: Some(off),
            };
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

        let mut addr = unsafe { if self.ir_data.rex.contains(REX::W) { self[gpr1].ur } else { self[gpr1].ud.into() } };
        if idx == 5 && !self.ir_data.rex.contains(REX::X) {
            MemoryInfo {
                addr: Mem{ r: addr },
                base: Some(reg1),
                idx: None,
                off: None,
            }
        } else {
            addr += unsafe { if self.ir_data.rex.contains(REX::W) { self[gpr2].ur } else { self[gpr2].ud.into() } } * mul;

            MemoryInfo {
                addr: Mem{ r: addr },
                base: Some(reg1),
                idx: Some(reg2),
                off: None,
            }
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
                reg1,
                reg2: Reg(reg2),
            };
        }

        let gpr: GPRs = reg2.try_into().unwrap();
        let mut addr = MemoryInfo {
            addr: unsafe {
                if self.ir_data.rex.contains(REX::B) {
                    Mem { r: self[gpr].ur }
                } else {
                    Mem { d: self[gpr].ud }
                }
            },
            base: Some(reg2),
            idx: None,
            off: None,
        };

        if rm == 4 {
            addr = self.decode_sib_64(ram);
        } else if rm == 5 && md == 1 {
            unsafe {
                if self.ir_data.rex.contains(REX::B) {
                    addr.addr.r = self.ip.r;
                } else {
                    addr.addr.d = self.ip.d;
                }
            }
        }

        if md == 1 {
            let disp = self.read_u8(ram).into();
            addr.off = Some(disp);
            unsafe { addr.addr.r += u64::from(disp) };
        } else if md == 2 {
            let disp = self.read_u32(ram);
            addr.off = Some(disp);
            unsafe { addr.addr.r += u64::from(disp) };
        }

        ModRM {
            reg1,
            reg2: Addr(addr),
        }
    }

    pub fn decode_modrm(&mut self, ram: &mut RAM) -> ModRM {
        self.decode_modrm_64(ram)
    }

// --- Size Decoding --- //

    fn decode_size_64(&self, location: &Location) -> GPRSize {
        if location == &Location::Address && self.ir_data.addr || location == &Location::Operand && self.ir_data.oper {
            GPRSize::X16(0)
        } else if self.ir_data.rex.contains(REX::W) {
            GPRSize::X64(0)
        } else {
            GPRSize::X32(0)
        }
    }

    // TODO: detect real/protected/etc
    pub fn decode_size(&self, location: &Location) -> GPRSize {
        self.decode_size_64(location)
    }

    pub fn decode_gpr_size(&mut self, gpr_idx: u8) -> GPRSize {
        let gpr: GPRs = gpr_idx.try_into().unwrap();
        unsafe {
            match self.decode_size(&Location::Operand) {
                GPRSize::X16(_) => GPRSize::X16(self[gpr].uw),
                GPRSize::X32(_) => GPRSize::X32(self[gpr].ud),
                GPRSize::X64(_) => GPRSize::X64(self[gpr].ur),
            }
        }
    }

    pub fn decode_mem_size(&mut self, ram: &mut RAM, addr: Mem) -> GPRSize {
        let virt_addr: u64 = unsafe {
            match self.decode_size(&Location::Address) {
                GPRSize::X16(_) => addr.w.into(),
                GPRSize::X32(_) => addr.r,
                GPRSize::X64(_) => addr.d.into(),
            }
        };

        match self.decode_size(&Location::Operand) {
            GPRSize::X16(_) => GPRSize::X16(self.get_u16(ram, virt_addr)),
            GPRSize::X32(_) => GPRSize::X32(self.get_u32(ram, virt_addr)),
            GPRSize::X64(_) => GPRSize::X64(self.get_u64(ram, virt_addr)),
        }
    }

    pub fn decode_rm_size(&mut self, ram: &mut RAM, modrm: &ModRM) -> GPRSize {
        match modrm.reg2 {
            Addr(mem) => self.decode_mem_size(ram, mem.addr),
            Reg(reg) => self.decode_gpr_size(reg),
        }
    }
}
