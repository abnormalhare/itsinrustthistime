use crate::{cpu::CPU, ram::RAM};

impl CPU {
    pub fn read_instr(&mut self, ram: &mut RAM) {
        let ip = self.get_mem_addr();
        if let Some((data, cache)) = ram.read(Some(self.cache_addr), ip) {
            self.ir = data;
            self.cache_addr = cache;

            self.increment_ip();
        } else {
            unreachable!()
        }
    }

    pub fn setup_cache(&mut self, ram: &mut RAM) {
        let ip = self.get_mem_addr();
        if let Some((_, cache)) = ram.read(Some(self.cache_addr), ip) {
            self.cache_addr = cache;
        } else {
            unreachable!()
        }
    }
}
