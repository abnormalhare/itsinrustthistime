use crate::{cpu::{CPU, decode::{GPRSize, Location, MemoryInfo, RM}}, reg::GPRs};

const LO8_NAMES: [&str; 16] = [
    "AL", "CL", "DL", "BL", "SPL", "BPL", "SIL", "DIL",
    "R8B", "R9B", "R10B", "R11B", "R12B", "R13B", "R14B", "R15B"
];

const HI8_NAMES: [&str; 4] = [
    "AH", "CH", "DH", "BH",
];

const X16_NAMES: [&str; 16] = [
    "AX", "CX", "DX", "BX", "SP", "BP", "SI", "DI",
    "R8W", "R9W", "R10W", "R11W", "R12W", "R13W", "R14W", "R15W"
];

const X32_NAMES: [&str; 16] = [
    "EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI",
    "R8D", "R9D", "R10D", "R11D", "R12D", "R13D", "R14D", "R15D"
];

const X64_NAMES: [&str; 16] = [
    "RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI",
    "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15"
];

impl CPU {
    pub fn get_lo8_name(reg: u8) -> String {
        let reg: usize = reg.into();
        String::from(LO8_NAMES[reg])
    }

    pub fn get_hi8_name(reg: u8) -> String {
        let reg: usize = reg.into();
        String::from(HI8_NAMES[reg])
    }

    pub fn get_gpr_name(reg: u8, size: &GPRSize) -> String {
        let reg: usize = reg.into();

        String::from(
            match size {
                GPRSize::X16(_) => X16_NAMES[reg],
                GPRSize::X32(_) => X32_NAMES[reg],
                GPRSize::X64(_) => X64_NAMES[reg],
            }
        )
    }

    pub fn get_mem_name(mem: &MemoryInfo, ad_size: &GPRSize) -> String {
        let mut name = String::from("[");

        if let Some(base) = mem.base {
            name.push_str(&Self::get_gpr_name(base, ad_size));
            if mem.idx.is_some() {
                name.push('+');
            }
        }

        if let Some(idx) = mem.idx {
            name.push_str(&Self::get_gpr_name(idx, ad_size));
            if mem.off.is_some() {
                name.push('+');
            }
        }

        if let Some(disp) = mem.off {
            let off = format!("{disp:#x}");
            name.push_str(&off);
        }

        name.push(']');

        name
    }

    pub fn get_rm_name(rm: &RM, op_size: &GPRSize, ad_size: &GPRSize) -> String {
        match rm {
            RM::Addr(addr) => Self::get_mem_name(addr, ad_size),
            RM::Reg(reg) => Self::get_gpr_name(*reg, op_size),
        }
    }
}
