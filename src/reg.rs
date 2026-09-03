use std::fmt::Display;

use bitflags::bitflags;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UHighBits {
    pub ls: u8,
    pub ms: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHighBits {
    pub ls: i8,
    pub ms: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union GPR {
    pub ub: UHighBits,
    pub sb: SHighBits,
    pub uw: u16,
    pub sw: i16,
    pub ud: u32,
    pub sd: i32,
    pub ur: u64,
    pub sr: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GPRs {
    AX  = 0x0,
    CX  = 0x1,
    DX  = 0x2,
    BX  = 0x3,
    SP  = 0x4,
    BP  = 0x5,
    SI  = 0x6,
    DI  = 0x7,
    R8  = 0x8,
    R9  = 0x9,
    R10 = 0xA,
    R11 = 0xB,
    R12 = 0xC,
    R13 = 0xD,
    R14 = 0xE,
    R15 = 0xF,
}

impl Default for GPR {
    fn default() -> Self {
        GPR { ur: 0 }
    }
}

impl TryFrom<u8> for GPRs {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(Self::AX),
            1  => Ok(Self::CX),
            2  => Ok(Self::DX),
            3  => Ok(Self::BX),
            4  => Ok(Self::SP),
            5  => Ok(Self::BP),
            6  => Ok(Self::SI),
            7  => Ok(Self::DI),
            8  => Ok(Self::R8),
            9  => Ok(Self::R9),
            10 => Ok(Self::R10),
            11 => Ok(Self::R11),
            12 => Ok(Self::R12),
            13 => Ok(Self::R13),
            14 => Ok(Self::R14),
            15 => Ok(Self::R15),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy)]
pub union Mem {
    pub w: u16,
    pub d: u32,
    pub r: u64,
}

#[allow(clippy::upper_case_acronyms)]
pub type IPR = Mem;

impl Default for IPR {
    fn default() -> Self {
        Self { r: 0x0000_FFF0 }
    }
}

pub struct SR {
    pub base: u32,
    pub limit: u16,
}

impl SR {
    pub const fn new(val: u16) -> Self {
        let mut ret = Self { base: 0, limit: 0xFFF };
        ret.set(val);
        ret
    }

    pub const fn get(&self) -> u16 {
        ((self.base & 0x000F_FFF0) >> 4) as u16
    }

    pub const fn set(&mut self, val: u16) {
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
        Self { limit: 0xFFFF, base: Mem { r: 0 } }
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct FR: u64 {
        const CF    = 0x0000_0001;
        const ON    = 0x0000_0002;
        const PF    = 0x0000_0004;
        const AF    = 0x0000_0010;
        const ZF    = 0x0000_0040;
        const SF    = 0x0000_0080;
        const TF    = 0x0000_0100;
        const IF    = 0x0000_0200;
        const DF    = 0x0000_0400;
        const OF    = 0x0000_0800;
        const IOPL  = 0x0000_3000;
        const NT    = 0x0000_4000;
        const RF    = 0x0001_0000;
        const VM    = 0x0002_0000;
        const AC    = 0x0004_0000;
        const VIF   = 0x0008_0000;
        const VIP   = 0x0010_0000;
        const ID    = 0x0020_0000;
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct CR0: u64 {
        const PE    = 0x0000_0001;
        const MP    = 0x0000_0002;
        const EM    = 0x0000_0004;
        const TS    = 0x0000_0008;
        const ET    = 0x0000_0010;
        const NE    = 0x0000_0020;
        const WP    = 0x0001_0000;
        const AM    = 0x0004_0000;
        const NW    = 0x2000_0000;
        const CD    = 0x4000_0000;
        const PG    = 0x8000_0000;
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
        const VME        = 0x0000_0001;
        const PVI        = 0x0000_0002;
        const TSD        = 0x0000_0004;
        const DE         = 0x0000_0008;
        const PSE        = 0x0000_0010;
        const PAE        = 0x0000_0020;
        const MCE        = 0x0000_0040;
        const PGE        = 0x0000_0080;
        const PCE        = 0x0000_0100;
        const OSFXSR     = 0x0000_0200;
        const OSXMMEXCPT = 0x0000_0400;
        const UMIP       = 0x0000_0800;
        const LA57       = 0x0000_1000;
        const VMXE       = 0x0000_2000;
        const SMXE       = 0x0000_4000;
        const FSGSBASE   = 0x0001_0000;
        const PCIDE      = 0x0002_0000;
        const OSXSAVE    = 0x0004_0000;
        const SMEP       = 0x0010_0000;
        const SMAP       = 0x0020_0000;
        const PKE        = 0x0040_0000;
        const CET        = 0x0080_0000;
        const PKS        = 0x0100_0000;
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
    Disabled(CR3NOPCID),
    Enabled(CR3PCID),
}

impl TryFrom<CR3> for CR3NOPCID {
    type Error = ();

    fn try_from(value: CR3) -> Result<Self, Self::Error> {
        match value {
            CR3::Disabled(val) => Ok(val),
            CR3::Enabled(_) => Err(()),
        }
    }
}

impl TryFrom<CR3> for CR3PCID {
    type Error = ();

    fn try_from(value: CR3) -> Result<Self, Self::Error> {
        match value {
            CR3::Disabled(_) => Err(()),
            CR3::Enabled(val) => Ok(val),
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
    pub const fn new(val: u8) -> Self {
        let mut ret = Self(0);
        ret.set(val);
        ret
    }

    pub const fn from_raw(val: u64) -> Self {
        Self(val)
    }

    pub const fn into_raw(self) -> u64 {
        self.0
    }

    pub const fn get(&self) -> u8 {
        (self.0 & 0b1111) as u8
    }

    pub const fn set(&mut self, val: u8) {
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
            0 => Ok(Self::CR0),
            2 => Ok(Self::CR2),
            3 => Ok(Self::CR3),
            4 => Ok(Self::CR4),
            8 => Ok(Self::CR8),
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
        const B0    = 0x00001;
        const B1    = 0x00002;
        const B2    = 0x00004;
        const B3    = 0x00008;
        const BLD   = 0x00800;
        const BK    = 0x01000;
        const BD    = 0x02000;
        const BS    = 0x04000;
        const BT    = 0x08000;
        const RTM   = 0x10000;
        const ON    = 0xFFFE_0000;
    }

    pub struct DR7: u64 {
        const L0    = 0x0000_0001;
        const G0    = 0x0000_0002;
        const L1    = 0x0000_0004;
        const G1    = 0x0000_0008;
        const L2    = 0x0000_0010;
        const G2    = 0x0000_0020;
        const L3    = 0x0000_0040;
        const G3    = 0x0000_0080;

        const RW0   = 0x0000_0300;
        const LEN0  = 0x0000_0C00;
        const RW1   = 0x0000_3000;
        const LEN1  = 0x0000_C000;
        const RW2   = 0x0003_0000;
        const LEN2  = 0x000C_0000;
        const RW3   = 0x0030_0000;
        const LEN3  = 0x00C0_0000;
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
            0 => Ok(Self::DR0),
            1 => Ok(Self::DR1),
            2 => Ok(Self::DR2),
            3 => Ok(Self::DR3),
            6 => Ok(Self::DR6),
            7 => Ok(Self::DR7),
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

bitflags! {
    pub struct REX: u8 {
        const B     = 0b0001;
        const X     = 0b0010;
        const R     = 0b0100;
        const W     = 0b1000;
    }
}

// all of these bools are independent
#[allow(clippy::struct_excessive_bools)]
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
