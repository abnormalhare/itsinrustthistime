use crate::{cpu::msr::{IA32EFER, MSRs}, ram::RAM, reg::{CR0, CRVals, CRs, DR6, DR7, DRVals, DRs, DTR, FR, GPR, GPRs, IPR, InstrData, SR, SRs}};

mod decode;
mod reg;
mod rw;
mod msr;
mod opcodes;

pub struct CPU {
    gprs: [GPR; 16],
    ip: IPR,
    srs: [SR; 6],
    flags: FR,
    crs: [u64; 16],
    drs: [u64; 8],
    msrs: MSRs,

    gdtr: DTR,
    idtr: DTR,
    ldtr: SR,
    tr: SR,

    ir: u8,
    ir_data: InstrData,
    ir_cache: u64,

    read_cache: Option<u64>,
}

impl CPU {
    pub fn new(ram: &mut RAM) -> Self {
        let mut cpu = Self {
            gprs: std::array::from_fn(|_| GPR::default()),
            ip: IPR::default(),
            srs: std::array::from_fn(|_| SR::new(0)),
            flags: FR::from_bits_retain(1),
            crs: [0; 16],
            drs: [0; 8],
            msrs: MSRs::default(),
            gdtr: DTR::default(),
            idtr: DTR::default(),
            ldtr: SR::new(0),
            tr: SR::new(0),
            ir: 0,
            ir_data: InstrData::default(),
            ir_cache: 0,
            read_cache: None,
        };
        cpu.setup_cache(ram);

        cpu[GPRs::DX].ud = 0x00A7_0F41;

        cpu[SRs::CS].base = 0xFFFF_0000;


        cpu.ip.0.d = 0x0000_FFF0;
        let cr0 = CR0::from_bits_retain(0x6000_0010);
        cpu.set_cr(CRs::CR0, CRVals::CR0(cr0));

        let dr6 = DR6::from_bits_retain(0xFFFF_0FF0);
        cpu.set_dr(DRs::DR6, DRVals::DR6(dr6));

        let dr7 = DR7::from_bits_retain(0x0000_0400);
        cpu.set_dr(DRs::DR7, DRVals::DR7(dr7));

        cpu
    }

    fn is_real_mode(&self) -> bool {
        let cr0: CR0 = self.get_cr(&CRs::CR0).try_into().unwrap();

        let is_protected = cr0.contains(CR0::from_bits_retain(0x1));

        !is_protected
    }

    fn is_protected_mode(&self) -> bool {
        let cr0: CR0 = self.get_cr(&CRs::CR0).try_into().unwrap();

        let is_protected = cr0.contains(CR0::from_bits_retain(0x1));
        let is_in_vm = self.flags.contains(FR::from_bits_retain(0x20000));
        let long_mode_active = self.msrs.ia32_efer.contains(IA32EFER::from_bits_retain(0x400));

        is_protected && !is_in_vm && !long_mode_active
    }

    fn is_vm_mode(&self) -> bool {
        let cr0: CR0 = self.get_cr(&CRs::CR0).try_into().unwrap();

        let is_protected = cr0.contains(CR0::from_bits_retain(0x1));
        let is_in_vm = self.flags.contains(FR::from_bits_retain(0x20000));
        let long_mode_active = self.msrs.ia32_efer.contains(IA32EFER::from_bits_retain(0x400));

        is_protected && is_in_vm && !long_mode_active
    }

    // TODO: add descriptor bits
    fn is_long_mode(&self) -> bool {
        let cr0: CR0 = self.get_cr(&CRs::CR0).try_into().unwrap();

        let is_protected = cr0.contains(CR0::from_bits_retain(0x1));
        let is_in_vm = self.flags.contains(FR::from_bits_retain(0x20000));
        let long_mode_active = self.msrs.ia32_efer.contains(IA32EFER::from_bits_retain(0x400));

        is_protected && !is_in_vm && long_mode_active
    }

    // TODO: paging
    pub fn get_mem_addr(&self) -> u64 {
        if self.is_long_mode() {
            unsafe { self.ip.0.r }
        } else if self.is_protected_mode() {
            unsafe { u64::from(self.ip.0.d + self[SRs::CS].base) }
        } else {
            unsafe { u64::from(self.ip.0.w) + u64::from(self[SRs::CS].base) }
        }
    }

    pub fn increment_ip(&mut self) {
        if self.is_long_mode() {
            unsafe { self.ip.0.r += 1 };
        } else if self.is_protected_mode() {
            unsafe { self.ip.0.d += 1 };
        } else {
            unsafe { self.ip.0.w += 1 };
        }
    }

    pub fn run(&mut self, ram: &mut RAM) {
        self.read_u8(ram);
        Self::OPCODES[self.ir as usize](self, ram);
    }
}
