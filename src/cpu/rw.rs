use crate::{cpu::CPU, ram::RAM};

impl CPU {
    fn read_instr(&mut self, ram: &mut RAM) {
        let ip = self.get_mem_addr();
        if let Some((data, cache)) = ram.read(Some(self.ir_cache), ip) {
            self.ir = data;
            self.ir_cache = cache;

            self.increment_ip();
        } else {
            unreachable!()
        }
    }

    pub fn read_u8(&mut self, ram: &mut RAM) -> u8 {
        self.read_instr(ram);

        self.ir
    }

    pub fn read_u16(&mut self, ram: &mut RAM) -> u16 {
        let val: u16 = self.read_u8(ram).into();

        (u16::from(self.read_u8(ram)) << 8) | val
    }

    pub fn read_u32(&mut self, ram: &mut RAM) -> u32 {
        let val: u32 = self.read_u16(ram).into();

        (u32::from(self.read_u16(ram)) << 16) | val
    }

    pub fn read_u64(&mut self, ram: &mut RAM) -> u64 {
        let val: u64 = self.read_u32(ram).into();

        (u64::from(self.read_u32(ram)) << 32) | val
    }

    pub fn get_u8(&mut self, ram: &mut RAM, virt_addr: u64) -> u8 {
        if let Some((data, cache)) = ram.read(self.read_cache, virt_addr) {
            self.read_cache = Some(cache);

            data
        } else {
            unreachable!()
        }
    }

    pub fn get_u16(&mut self, ram: &mut RAM, virt_addr: u64) -> u16 {
        let val: u16 = self.get_u8(ram, virt_addr).into();

        (u16::from(self.get_u8(ram, virt_addr + 1)) << 8) + val
    }

    pub fn get_u32(&mut self, ram: &mut RAM, virt_addr: u64) -> u32 {
        let val: u32 = self.get_u16(ram, virt_addr).into();

        (u32::from(self.get_u16(ram, virt_addr + 2)) << 16) + val
    }

    pub fn get_u64(&mut self, ram: &mut RAM, virt_addr: u64) -> u64 {
        let val: u64 = self.get_u32(ram, virt_addr).into();

        (u64::from(self.get_u32(ram, virt_addr + 4)) << 32) + val
    }

    pub fn setup_cache(&mut self, ram: &mut RAM) {
        let ip = self.get_mem_addr();
        if let Some((_, cache)) = ram.read(None, ip) {
            self.ir_cache = cache;
        } else {
            unreachable!()
        }
    }
}
