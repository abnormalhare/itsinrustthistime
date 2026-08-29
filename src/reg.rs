use bitflags::bitflags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UHighBits {
    pub ls: u8,
    pub ms: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LHighBits {
    pub ls: u8,
    pub ms: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union GPR {
    pub ub: UHighBits,
    pub sb: LHighBits,
    pub uw: u16,
    pub sw: i16,
    pub ud: u32,
    pub sd: i32,
    pub ur: u64,
    pub sr: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GPRs {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Default for GPR {
    fn default() -> Self {
        GPR { ur: 0 }
    }
}

pub union Mem {
    pub w: u16,
    pub d: u32,
    pub r: u64,
}

pub struct IPR(pub Mem);

impl Default for IPR {
    fn default() -> Self {
        IPR { 0: Mem { r: 0 } }
    }
}

pub struct SR {
    pub base: u32,
    pub limit: u16,
}

impl SR {
    pub fn new(val: u16) -> Self {
        let mut ret: Self = SR { base: 0, limit: 0xFFF };
        ret.set(val);
        ret
    }

    pub fn get(&self) -> u16 {
        ((self.base & 0x000FFFF0) >> 4) as u16
    }

    pub fn set(&mut self, val: u16) {
        self.base = (val as u32) << 4;
    }
}

pub enum SRs {
    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
}

pub type MMR = u64;

pub union XMMR {
    xi: [u128; 4],
    xf: [f128; 4],
    ri: [u64;  8],
    rf: [u64;  8],
    ei: [u32; 16],
    ef: [f32; 16],
}

pub struct DTR {
    pub limit: u16,
    pub base: Mem,
}

impl Default for DTR {
    fn default() -> Self {
        DTR { limit: 0xFFFF, base: Mem { r: 0 } }
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct FR: u64 {
        const CF    = 0x00000001;
        const ON    = 0x00000002;
        const PF    = 0x00000004;
        const AF    = 0x00000010;
        const ZF    = 0x00000040;
        const SF    = 0x00000080;
        const TF    = 0x00000100;
        const IF    = 0x00000200;
        const DF    = 0x00000400;
        const OF    = 0x00000800;
        const IOPL  = 0x00003000;
        const NT    = 0x00004000;
        const RF    = 0x00010000;
        const VM    = 0x00020000;
        const AC    = 0x00040000;
        const VIF   = 0x00080000;
        const VIP   = 0x00100000;
        const ID    = 0x00200000;
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct CR0: u64 {
        const PE    = 0x00000001;
        const MP    = 0x00000002;
        const EM    = 0x00000004;
        const TS    = 0x00000008;
        const ET    = 0x00000010;
        const NE    = 0x00000020;
        const WP    = 0x00010000;
        const AM    = 0x00040000;
        const NW    = 0x20000000;
        const CD    = 0x40000000;
        const PG    = 0x80000000;
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct CR3NOPCID: u64 {
        const PWT   = 0x04;
        const PCD   = 0x10;
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct CR3PCID: u64 {
        const PCID  = 0xFFF;
    }

    pub struct CR4: u64 {
        const VME        = 0x000001;
        const PVI        = 0x000002;
        const TSD        = 0x000004;
        const DE         = 0x000008;
        const PSE        = 0x000010;
        const PAE        = 0x000020;
        const MCE        = 0x000040;
        const PGE        = 0x000080;
        const PCE        = 0x000100;
        const OSFXSR     = 0x000200;
        const OSXMMEXCPT = 0x000400;
        const UMIP       = 0x000800;
        const LA57       = 0x001000;
        const VMXE       = 0x002000;
        const SMXE       = 0x004000;
        const FSGSBASE   = 0x010000;
        const PCIDE      = 0x020000;
        const OSXSAVE    = 0x040000;
        const SMEP       = 0x100000;
        const SMAP       = 0x200000;
        const PKE        = 0x400000;
        const CET        = 0x800000;
        const PKS        = 0x1000000;
    }
}

impl TryFrom<CRVals> for CR0 {
    type Error = ();

    fn try_from(value: CRVals) -> Result<Self, Self::Error> {
        match value {
            CRVals::CR0(val) => Ok(val),
            _ => Err(()),
        }
    }
}

pub enum CR3 {
    DISABLED(CR3NOPCID),
    ENABLED(CR3PCID),
}

impl TryFrom<CR3> for CR3NOPCID {
    type Error = ();

    fn try_from(value: CR3) -> Result<Self, Self::Error> {
        match value {
            CR3::DISABLED(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl TryFrom<CR3> for CR3PCID {
    type Error = ();

    fn try_from(value: CR3) -> Result<Self, Self::Error> {
        match value {
            CR3::ENABLED(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl TryFrom<CRVals> for CR3 {
    type Error = ();

    fn try_from(value: CRVals) -> Result<Self, Self::Error> {
        match value {
            CRVals::CR3(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl TryFrom<CRVals> for CR4 {
    type Error = ();

    fn try_from(value: CRVals) -> Result<Self, Self::Error> {
        match value {
            CRVals::CR4(val) => Ok(val),
            _ => Err(()),
        }
    }
}

pub struct CR8(u64);

impl TryFrom<CRVals> for CR8 {
    type Error = ();

    fn try_from(value: CRVals) -> Result<Self, Self::Error> {
        match value {
            CRVals::CR8(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl CR8 {
    pub fn new(val: u8) -> Self {
        let mut ret: Self = CR8(0);
        ret.set(val);
        ret
    }

    pub fn from_raw(val: u64) -> Self {
        CR8(val)
    }

    pub fn into_raw(self) -> u64 {
        self.0
    }

    pub fn get(&self) -> u8 {
        (self.0 & 0b1111) as u8
    }

    pub fn set(&mut self, val: u8) {
        self.0 = (val & 0b1111) as u64;
    }
}

pub enum CRs {
    CR0 = 0,
    CR2 = 2,
    CR3 = 3,
    CR4 = 4,
    CR8 = 8,
}

impl TryFrom<u8> for CRs {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CRs::CR0),
            2 => Ok(CRs::CR2),
            3 => Ok(CRs::CR3),
            4 => Ok(CRs::CR4),
            8 => Ok(CRs::CR8),
            _ => unreachable!(),
        }
    }
}

pub enum CRVals {
    CR0(CR0),
    CR2(u64),
    CR3(CR3),
    CR4(CR4),
    CR8(CR8),
}

bitflags! {
    pub struct DR6: u64 {
        const B0    = 0x0001;
        const B1    = 0x0002;
        const B2    = 0x0004;
        const B3    = 0x0008;
        const BLD   = 0x0800;
        const BK    = 0x1000;
        const BD    = 0x2000;
        const BS    = 0x4000;
        const BT    = 0x8000;
        const RTM   = 0x10000;
        const ON    = 0xFFFE0000;
    }

    pub struct DR7: u64 {
        const L0    = 0x00000001;
        const G0    = 0x00000002;
        const L1    = 0x00000004;
        const G1    = 0x00000008;
        const L2    = 0x00000010;
        const G2    = 0x00000020;
        const L3    = 0x00000040;
        const G3    = 0x00000080;

        const RW0   = 0x00000300;
        const LEN0  = 0x00000C00;
        const RW1   = 0x00000300;
        const LEN1  = 0x00000C00;
        const RW2   = 0x00000300;
        const LEN2  = 0x00000C00;
        const RW3   = 0x00000300;
        const LEN3  = 0x00000C00;
    }
}

pub enum DRs {
    DR0,
    DR1,
    DR2,
    DR3,
    DR6,
    DR7,
}

impl TryFrom<u8> for DRs {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DRs::DR0),
            1 => Ok(DRs::DR1),
            2 => Ok(DRs::DR2),
            3 => Ok(DRs::DR3),
            6 => Ok(DRs::DR6),
            7 => Ok(DRs::DR7),
            _ => unreachable!(),
        }
    }
}

pub enum DRVals {
    DR0(u64),
    DR1(u64),
    DR2(u64),
    DR3(u64),
    DR6(DR6),
    DR7(DR7),
}

pub enum Register {
    GPR(GPR),
    SR(SR),
    MMR(MMR),
    XMMR(XMMR),
    CR(CRs),
    DR(DRs),
}




bitflags! {
    pub struct REX: u8 {
        const B     = 0b0001;
        const X     = 0b0010;
        const R     = 0b0100;
        const W     = 0b1000;
    }
}

pub struct InstrData {
    pub rex: REX,
    pub null: bool,
    pub fs: bool,
    pub gs: bool,
    pub oper: bool,
    pub addr: bool,
    pub wait: bool,
    pub lock: bool,
    pub rep: bool,
}

impl Default for InstrData {
    fn default() -> Self {
        Self { rex: REX::empty(), null: false, fs: false, gs: false, oper: false, addr: false, wait: false, lock: false, rep: false }
    }
}
