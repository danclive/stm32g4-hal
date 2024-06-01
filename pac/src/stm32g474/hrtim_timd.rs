#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timdcr: Timdcr,
    timdisr: Timdisr,
    timdicr: Timdicr,
    timddier: Timddier,
    cntdr: Cntdr,
    perdr: Perdr,
    repdr: Repdr,
    cmp1dr: Cmp1dr,
    cmp1cdr: Cmp1cdr,
    cmp2dr: Cmp2dr,
    cmp3dr: Cmp3dr,
    cmp4dr: Cmp4dr,
    cpt1dr: Cpt1dr,
    cpt2dr: Cpt2dr,
    dtdr: Dtdr,
    setd1r: Setd1r,
    rstd1r: Rstd1r,
    setd2r: Setd2r,
    rstd2r: Rstd2r,
    eefdr1: Eefdr1,
    eefdr2: Eefdr2,
    rstdr: Rstdr,
    chpdr: Chpdr,
    cpt1dcr: Cpt1dcr,
    cpt2dcr: Cpt2dcr,
    outdr: Outdr,
    fltdr: Fltdr,
    timdcr2: Timdcr2,
    deefr3: Deefr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timdcr(&self) -> &Timdcr {
        &self.timdcr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timdisr(&self) -> &Timdisr {
        &self.timdisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timdicr(&self) -> &Timdicr {
        &self.timdicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timddier(&self) -> &Timddier {
        &self.timddier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntdr(&self) -> &Cntdr {
        &self.cntdr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perdr(&self) -> &Perdr {
        &self.perdr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repdr(&self) -> &Repdr {
        &self.repdr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1dr(&self) -> &Cmp1dr {
        &self.cmp1dr
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cdr(&self) -> &Cmp1cdr {
        &self.cmp1cdr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2dr(&self) -> &Cmp2dr {
        &self.cmp2dr
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3dr(&self) -> &Cmp3dr {
        &self.cmp3dr
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4dr(&self) -> &Cmp4dr {
        &self.cmp4dr
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1dr(&self) -> &Cpt1dr {
        &self.cpt1dr
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2dr(&self) -> &Cpt2dr {
        &self.cpt2dr
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtdr(&self) -> &Dtdr {
        &self.dtdr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setd1r(&self) -> &Setd1r {
        &self.setd1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rstd1r(&self) -> &Rstd1r {
        &self.rstd1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setd2r(&self) -> &Setd2r {
        &self.setd2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstd2r(&self) -> &Rstd2r {
        &self.rstd2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefdr1(&self) -> &Eefdr1 {
        &self.eefdr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefdr2(&self) -> &Eefdr2 {
        &self.eefdr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstdr(&self) -> &Rstdr {
        &self.rstdr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpdr(&self) -> &Chpdr {
        &self.chpdr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1dcr(&self) -> &Cpt1dcr {
        &self.cpt1dcr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2dcr(&self) -> &Cpt2dcr {
        &self.cpt2dcr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outdr(&self) -> &Outdr {
        &self.outdr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltdr(&self) -> &Fltdr {
        &self.fltdr
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timdcr2(&self) -> &Timdcr2 {
        &self.timdcr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn deefr3(&self) -> &Deefr3 {
        &self.deefr3
    }
}
#[doc = "TIMDCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdcr`]
module"]
#[doc(alias = "TIMDCR")]
pub type Timdcr = crate::Reg<timdcr::TimdcrSpec>;
#[doc = "Timerx Control Register"]
pub mod timdcr;
#[doc = "TIMDISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdisr`]
module"]
#[doc(alias = "TIMDISR")]
pub type Timdisr = crate::Reg<timdisr::TimdisrSpec>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timdisr;
#[doc = "TIMDICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdicr`]
module"]
#[doc(alias = "TIMDICR")]
pub type Timdicr = crate::Reg<timdicr::TimdicrSpec>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timdicr;
#[doc = "TIMDDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timddier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timddier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timddier`]
module"]
#[doc(alias = "TIMDDIER")]
pub type Timddier = crate::Reg<timddier::TimddierSpec>;
#[doc = "TIMxDIER"]
pub mod timddier;
#[doc = "CNTDR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntdr`]
module"]
#[doc(alias = "CNTDR")]
pub type Cntdr = crate::Reg<cntdr::CntdrSpec>;
#[doc = "Timerx Counter Register"]
pub mod cntdr;
#[doc = "PERDR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perdr`]
module"]
#[doc(alias = "PERDR")]
pub type Perdr = crate::Reg<perdr::PerdrSpec>;
#[doc = "Timerx Period Register"]
pub mod perdr;
#[doc = "REPDR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repdr`]
module"]
#[doc(alias = "REPDR")]
pub type Repdr = crate::Reg<repdr::RepdrSpec>;
#[doc = "Timerx Repetition Register"]
pub mod repdr;
#[doc = "CMP1DR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1dr`]
module"]
#[doc(alias = "CMP1DR")]
pub type Cmp1dr = crate::Reg<cmp1dr::Cmp1drSpec>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1dr;
#[doc = "CMP1CDR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cdr`]
module"]
#[doc(alias = "CMP1CDR")]
pub type Cmp1cdr = crate::Reg<cmp1cdr::Cmp1cdrSpec>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cdr;
#[doc = "CMP2DR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2dr`]
module"]
#[doc(alias = "CMP2DR")]
pub type Cmp2dr = crate::Reg<cmp2dr::Cmp2drSpec>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2dr;
#[doc = "CMP3DR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3dr`]
module"]
#[doc(alias = "CMP3DR")]
pub type Cmp3dr = crate::Reg<cmp3dr::Cmp3drSpec>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3dr;
#[doc = "CMP4DR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4dr`]
module"]
#[doc(alias = "CMP4DR")]
pub type Cmp4dr = crate::Reg<cmp4dr::Cmp4drSpec>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4dr;
#[doc = "CPT1DR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1dr`]
module"]
#[doc(alias = "CPT1DR")]
pub type Cpt1dr = crate::Reg<cpt1dr::Cpt1drSpec>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1dr;
#[doc = "CPT2DR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2dr`]
module"]
#[doc(alias = "CPT2DR")]
pub type Cpt2dr = crate::Reg<cpt2dr::Cpt2drSpec>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2dr;
#[doc = "DTDR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtdr`]
module"]
#[doc(alias = "DTDR")]
pub type Dtdr = crate::Reg<dtdr::DtdrSpec>;
#[doc = "Timerx Deadtime Register"]
pub mod dtdr;
#[doc = "SETD1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setd1r`]
module"]
#[doc(alias = "SETD1R")]
pub type Setd1r = crate::Reg<setd1r::Setd1rSpec>;
#[doc = "Timerx Output1 Set Register"]
pub mod setd1r;
#[doc = "RSTD1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstd1r`]
module"]
#[doc(alias = "RSTD1R")]
pub type Rstd1r = crate::Reg<rstd1r::Rstd1rSpec>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstd1r;
#[doc = "SETD2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setd2r`]
module"]
#[doc(alias = "SETD2R")]
pub type Setd2r = crate::Reg<setd2r::Setd2rSpec>;
#[doc = "Timerx Output2 Set Register"]
pub mod setd2r;
#[doc = "RSTD2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstd2r`]
module"]
#[doc(alias = "RSTD2R")]
pub type Rstd2r = crate::Reg<rstd2r::Rstd2rSpec>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstd2r;
#[doc = "EEFDR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefdr1`]
module"]
#[doc(alias = "EEFDR1")]
pub type Eefdr1 = crate::Reg<eefdr1::Eefdr1Spec>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefdr1;
#[doc = "EEFDR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefdr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefdr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefdr2`]
module"]
#[doc(alias = "EEFDR2")]
pub type Eefdr2 = crate::Reg<eefdr2::Eefdr2Spec>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefdr2;
#[doc = "RSTDR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstdr`]
module"]
#[doc(alias = "RSTDR")]
pub type Rstdr = crate::Reg<rstdr::RstdrSpec>;
#[doc = "TimerA Reset Register"]
pub mod rstdr;
#[doc = "CHPDR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpdr`]
module"]
#[doc(alias = "CHPDR")]
pub type Chpdr = crate::Reg<chpdr::ChpdrSpec>;
#[doc = "Timerx Chopper Register"]
pub mod chpdr;
#[doc = "CPT1DCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1dcr`]
module"]
#[doc(alias = "CPT1DCR")]
pub type Cpt1dcr = crate::Reg<cpt1dcr::Cpt1dcrSpec>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1dcr;
#[doc = "CPT2DCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2dcr`]
module"]
#[doc(alias = "CPT2DCR")]
pub type Cpt2dcr = crate::Reg<cpt2dcr::Cpt2dcrSpec>;
#[doc = "CPT2xCR"]
pub mod cpt2dcr;
#[doc = "OUTDR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outdr`]
module"]
#[doc(alias = "OUTDR")]
pub type Outdr = crate::Reg<outdr::OutdrSpec>;
#[doc = "Timerx Output Register"]
pub mod outdr;
#[doc = "FLTDR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltdr`]
module"]
#[doc(alias = "FLTDR")]
pub type Fltdr = crate::Reg<fltdr::FltdrSpec>;
#[doc = "Timerx Fault Register"]
pub mod fltdr;
#[doc = "TIMDCR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timdcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timdcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timdcr2`]
module"]
#[doc(alias = "TIMDCR2")]
pub type Timdcr2 = crate::Reg<timdcr2::Timdcr2Spec>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timdcr2;
#[doc = "DEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deefr3`]
module"]
#[doc(alias = "DEEFR3")]
pub type Deefr3 = crate::Reg<deefr3::Deefr3Spec>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod deefr3;
