use crate::{cpu::CPU, ram::RAM};

impl CPU {
    pub fn read_instr(&mut self, ram: &mut RAM) {
        let ip = self.get_mem_addr() as u32;
        (self.ir, self.cache_addr) = ram.read(self.cache_addr, ip);
        self.increment_ip();
    }
}
