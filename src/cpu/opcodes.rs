use crate::{cpu::{CPU, decode::{GPRSize, Location}}, ram::RAM, reg::GPRs};


impl CPU {
    fn op_unimplemented(&mut self, _: &mut RAM) {
        panic!("unimplemented opcode: {}", self.ir);
    }

    fn op_03(&mut self, ram: &mut RAM) {
        let modrm = self.decode_modrm(ram);
        let reg = modrm.reg1;
        let rm = modrm.reg2;

        let val = self.decode_rm_size(ram, &modrm);
        let gpr: GPRs = reg.try_into().unwrap();
        unsafe {
            match val {
                GPRSize::X16(val) => self[gpr].uw += val,
                GPRSize::X32(val) => self[gpr].ud += val,
                GPRSize::X64(val) => self[gpr].ur += val,
            }
        }

        let reg_name = Self::get_gpr_name(reg, &val);
        let rm_name = Self::get_rm_name(&rm, &val, &self.decode_size(&Location::Address));
        println!("ADD {reg_name}, {rm_name}");

        unsafe {
            println!("FINAL VAL: {:#x}", self[gpr].ur);
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
