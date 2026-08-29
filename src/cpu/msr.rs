use bitflags::bitflags;

bitflags! {
    pub struct IA32EFER: u64 {
        const SCE   = 0x001;
        const LME   = 0x100;
        const LMA   = 0x400;
        const NXE   = 0x800;
    }
}

pub struct MSRs {
    pub ia32_efer: IA32EFER,
}

pub enum MSRIdx {
    IA32EFER = 0xC0000080,
}

impl Default for MSRs {
    fn default() -> Self {
        MSRs { ia32_efer: IA32EFER::empty() }
    }
}
