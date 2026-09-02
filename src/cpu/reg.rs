use std::ops::{Index, IndexMut};

use crate::{cpu::CPU, reg::{CR0, CR3, CR3NOPCID, CR3PCID, CR4, CR8, CRVals, CRs, DR6, DR7, DRVals, DRs, GPR, GPRs, SR, SRs}};

impl Index<GPRs> for CPU {
    type Output = GPR;

    fn index(&self, index: GPRs) -> &Self::Output {
        &self.gprs[index as usize]
    }
}

impl IndexMut<GPRs> for CPU {
    fn index_mut(&mut self, index: GPRs) -> &mut Self::Output {
        &mut self.gprs[index as usize]
    }
}

impl Index<SRs> for CPU {
    type Output = SR;

    fn index(&self, index: SRs) -> &Self::Output {
        &self.srs[index as usize]
    }
}

impl IndexMut<SRs> for CPU {
    fn index_mut(&mut self, index: SRs) -> &mut Self::Output {
        &mut self.srs[index as usize]
    }
}

impl CPU {
    pub fn get_cr(&self, cr: &CRs) -> CRVals {
        match cr {
            CRs::CR0 => CRVals::CR0(CR0::from_bits_retain(self.crs[0])),
            CRs::CR2 => CRVals::CR2(self.crs[2]),
            CRs::CR3 => {
                let raw = self.crs[3];
                let cr4: CR4 = self.get_cr(&CRs::CR4).try_into().unwrap();

                if cr4.contains(CR4::PCIDE) {
                    CRVals::CR3(CR3::ENABLED(
                        CR3PCID::from_bits_retain(raw)
                    ))
                } else {
                    CRVals::CR3(CR3::DISABLED(
                        CR3NOPCID::from_bits_retain(raw)
                    ))
                }
            },
            CRs::CR4 => CRVals::CR4(CR4::from_bits_retain(self.crs[4])),
            CRs::CR8 => CRVals::CR8(CR8::from_raw(self.crs[8])),
        }
    }

    pub fn set_cr(&mut self, cr: CRs, val: CRVals) {
        match (cr, val) {
            (CRs::CR0, CRVals::CR0(cr0)) => self.crs[0] = cr0.bits(),
            (CRs::CR2, CRVals::CR2(cr2)) => self.crs[2] = cr2,
            (CRs::CR3, CRVals::CR3(CR3::DISABLED(cr3))) => self.crs[2] = cr3.bits(),
            (CRs::CR3, CRVals::CR3(CR3::ENABLED(cr3))) => self.crs[2] = cr3.bits(),
            (CRs::CR4, CRVals::CR4(cr4)) => self.crs[4] = cr4.bits(),
            (CRs::CR8, CRVals::CR8(cr8)) => self.crs[4] = cr8.into_raw(),
            _ => unreachable!()
        }
    }

    pub fn read_cr(&self, index: u8) -> Result<CRVals, ()> {
        if index > 16 {
            unreachable!();
        }

        CRs::try_from(index).map_or(Err(()), |cr| Ok(self.get_cr(&cr)))
    }

    pub fn write_cr(&mut self, index: u8, val: CRVals) {
        if index >= 16 {
            unreachable!();
        }

        if let Ok(cr) = CRs::try_from(index) {
            self.set_cr(cr, val);
        } else {
            // #ud
        }
    }

    pub fn get_dr(&self, dr: &DRs) -> DRVals {
        match dr {
            DRs::DR0 => DRVals::DR0(self.drs[0]),
            DRs::DR1 => DRVals::DR1(self.drs[1]),
            DRs::DR2 => DRVals::DR2(self.drs[2]),
            DRs::DR3 => DRVals::DR3(self.drs[3]),
            DRs::DR6 => DRVals::DR6(DR6::from_bits_retain(self.drs[6])),
            DRs::DR7 => DRVals::DR7(DR7::from_bits_retain(self.drs[7])),
        }
    }

    pub fn set_dr(&mut self, dr: DRs, val: DRVals) {
        match (dr, val) {
            (DRs::DR0, DRVals::DR0(dr0)) => self.drs[0] = dr0,
            (DRs::DR1, DRVals::DR1(dr1)) => self.drs[1] = dr1,
            (DRs::DR2, DRVals::DR2(dr2)) => self.drs[2] = dr2,
            (DRs::DR3, DRVals::DR3(dr3)) => self.drs[3] = dr3,
            (DRs::DR6, DRVals::DR6(dr6)) => self.drs[6] = dr6.bits(),
            (DRs::DR7, DRVals::DR7(dr7)) => self.drs[7] = dr7.bits(),
            _ => unreachable!(),
        }
    }

    pub fn read_dr(&self, index: u8) -> Result<DRVals, ()> {
        if index >= 8 {
            unreachable!();
        }

        DRs::try_from(index).map_or(Err(()), |dr| Ok(self.get_dr(&dr)))
    }

    pub fn write_dr(&mut self, index: u8, val: DRVals) {
        if index >= 8 {
            unreachable!();
        }

        if let Ok(dr) = DRs::try_from(index) {
            self.set_dr(dr, val);
        } else {
            // #ud
        }
    }
}
