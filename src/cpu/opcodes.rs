use crate::{cpu::{CPU, decode::GPRSize}, ram::RAM, reg::GPRs};


impl CPU {
    fn op_unimplemented(&mut self, _: &mut RAM) {
        panic!("unimplemented opcode: {}", self.ir);
    }

    fn op_03(&mut self, ram: &mut RAM) {
        let modrm = self.decode_modrm(ram);
        let val = self.decode_rm_gpr(ram, &modrm);
        let gpr: GPRs = modrm.reg1.try_into().unwrap();

        unsafe {
            match val {
                GPRSize::X16(val) => self[gpr].uw += val,
                GPRSize::X32(val) => self[gpr].ud += val,
                GPRSize::X64(val) => self[gpr].ur += val,
            }
        }
    }
}

type OpcodeFunc = fn(&mut CPU, &mut RAM);
const fn build_op_table() -> [OpcodeFunc; 0x100] {
    let mut table: [OpcodeFunc; 0x100] = [CPU::op_unimplemented; 0x100];

    table[0x03] = CPU::op_03;

    table
}

impl CPU {
    pub const OPCODES: [OpcodeFunc; 0x100] = build_op_table();
}
