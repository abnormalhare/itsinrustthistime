use crate::{cpu::CPU, ram::RAM};


impl CPU {
    fn op_unimplemented(&mut self, _: &mut RAM) {
        panic!("unimplemented opcode: {}", self.ir);
    }

    fn op_00(&mut self, _: &mut RAM) {
        panic!("unimplemented opcode: {}", self.ir);
    }
}

type OpcodeFunc = fn(&mut CPU, &mut RAM);
const fn build_op_table() -> [OpcodeFunc; 0x100] {
    let mut table: [OpcodeFunc; 0x100] = [CPU::op_unimplemented; 0x100];

    table[0x00] = CPU::op_00;

    table
}

impl CPU {
    pub const OPCODES: [OpcodeFunc; 0x100] = build_op_table();
}
