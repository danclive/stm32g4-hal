#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timbcr: Timbcr,
    timbisr: Timbisr,
    timbicr: Timbicr,
    timbdier: Timbdier,
    cntr: Cntr,
    perbr: Perbr,
    repbr: Repbr,
    cmp1br: Cmp1br,
    cmp1cbr: Cmp1cbr,
    cmp2br: Cmp2br,
    cmp3br: Cmp3br,
    cmp4br: Cmp4br,
    cpt1br: Cpt1br,
    cpt2br: Cpt2br,
    dtbr: Dtbr,
    setb1r: Setb1r,
    rstb1r: Rstb1r,
    setb2r: Setb2r,
    rstb2r: Rstb2r,
    eefbr1: Eefbr1,
    eefbr2: Eefbr2,
    rstbr: Rstbr,
    chpbr: Chpbr,
    cpt1bcr: Cpt1bcr,
    cpt2bcr: Cpt2bcr,
    outbr: Outbr,
    fltbr: Fltbr,
    timbcr2: Timbcr2,
    beefr3: Beefr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timbcr(&self) -> &Timbcr {
        &self.timbcr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timbisr(&self) -> &Timbisr {
        &self.timbisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timbicr(&self) -> &Timbicr {
        &self.timbicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timbdier(&self) -> &Timbdier {
        &self.timbdier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perbr(&self) -> &Perbr {
        &self.perbr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repbr(&self) -> &Repbr {
        &self.repbr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1br(&self) -> &Cmp1br {
        &self.cmp1br
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cbr(&self) -> &Cmp1cbr {
        &self.cmp1cbr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2br(&self) -> &Cmp2br {
        &self.cmp2br
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3br(&self) -> &Cmp3br {
        &self.cmp3br
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4br(&self) -> &Cmp4br {
        &self.cmp4br
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1br(&self) -> &Cpt1br {
        &self.cpt1br
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2br(&self) -> &Cpt2br {
        &self.cpt2br
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtbr(&self) -> &Dtbr {
        &self.dtbr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setb1r(&self) -> &Setb1r {
        &self.setb1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rstb1r(&self) -> &Rstb1r {
        &self.rstb1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setb2r(&self) -> &Setb2r {
        &self.setb2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstb2r(&self) -> &Rstb2r {
        &self.rstb2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefbr1(&self) -> &Eefbr1 {
        &self.eefbr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefbr2(&self) -> &Eefbr2 {
        &self.eefbr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstbr(&self) -> &Rstbr {
        &self.rstbr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpbr(&self) -> &Chpbr {
        &self.chpbr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1bcr(&self) -> &Cpt1bcr {
        &self.cpt1bcr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2bcr(&self) -> &Cpt2bcr {
        &self.cpt2bcr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outbr(&self) -> &Outbr {
        &self.outbr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltbr(&self) -> &Fltbr {
        &self.fltbr
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timbcr2(&self) -> &Timbcr2 {
        &self.timbcr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn beefr3(&self) -> &Beefr3 {
        &self.beefr3
    }
}
#[doc = "TIMBCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbcr`]
module"]
#[doc(alias = "TIMBCR")]
pub type Timbcr = crate::Reg<timbcr::TimbcrSpec>;
#[doc = "Timerx Control Register"]
pub mod timbcr;
#[doc = "TIMBISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbisr`]
module"]
#[doc(alias = "TIMBISR")]
pub type Timbisr = crate::Reg<timbisr::TimbisrSpec>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timbisr;
#[doc = "TIMBICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbicr`]
module"]
#[doc(alias = "TIMBICR")]
pub type Timbicr = crate::Reg<timbicr::TimbicrSpec>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timbicr;
#[doc = "TIMBDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbdier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbdier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbdier`]
module"]
#[doc(alias = "TIMBDIER")]
pub type Timbdier = crate::Reg<timbdier::TimbdierSpec>;
#[doc = "TIMxDIER"]
pub mod timbdier;
#[doc = "CNTR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "Timerx Counter Register"]
pub mod cntr;
#[doc = "PERBR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbr`]
module"]
#[doc(alias = "PERBR")]
pub type Perbr = crate::Reg<perbr::PerbrSpec>;
#[doc = "Timerx Period Register"]
pub mod perbr;
#[doc = "REPBR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repbr`]
module"]
#[doc(alias = "REPBR")]
pub type Repbr = crate::Reg<repbr::RepbrSpec>;
#[doc = "Timerx Repetition Register"]
pub mod repbr;
#[doc = "CMP1BR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1br`]
module"]
#[doc(alias = "CMP1BR")]
pub type Cmp1br = crate::Reg<cmp1br::Cmp1brSpec>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1br;
#[doc = "CMP1CBR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cbr`]
module"]
#[doc(alias = "CMP1CBR")]
pub type Cmp1cbr = crate::Reg<cmp1cbr::Cmp1cbrSpec>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cbr;
#[doc = "CMP2BR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2br`]
module"]
#[doc(alias = "CMP2BR")]
pub type Cmp2br = crate::Reg<cmp2br::Cmp2brSpec>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2br;
#[doc = "CMP3BR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3br`]
module"]
#[doc(alias = "CMP3BR")]
pub type Cmp3br = crate::Reg<cmp3br::Cmp3brSpec>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3br;
#[doc = "CMP4BR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4br`]
module"]
#[doc(alias = "CMP4BR")]
pub type Cmp4br = crate::Reg<cmp4br::Cmp4brSpec>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4br;
#[doc = "CPT1BR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1br`]
module"]
#[doc(alias = "CPT1BR")]
pub type Cpt1br = crate::Reg<cpt1br::Cpt1brSpec>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1br;
#[doc = "CPT2BR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2br::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2br`]
module"]
#[doc(alias = "CPT2BR")]
pub type Cpt2br = crate::Reg<cpt2br::Cpt2brSpec>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2br;
#[doc = "DTBR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbr`]
module"]
#[doc(alias = "DTBR")]
pub type Dtbr = crate::Reg<dtbr::DtbrSpec>;
#[doc = "Timerx Deadtime Register"]
pub mod dtbr;
#[doc = "SETB1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setb1r`]
module"]
#[doc(alias = "SETB1R")]
pub type Setb1r = crate::Reg<setb1r::Setb1rSpec>;
#[doc = "Timerx Output1 Set Register"]
pub mod setb1r;
#[doc = "RSTB1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstb1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstb1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstb1r`]
module"]
#[doc(alias = "RSTB1R")]
pub type Rstb1r = crate::Reg<rstb1r::Rstb1rSpec>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstb1r;
#[doc = "SETB2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setb2r`]
module"]
#[doc(alias = "SETB2R")]
pub type Setb2r = crate::Reg<setb2r::Setb2rSpec>;
#[doc = "Timerx Output2 Set Register"]
pub mod setb2r;
#[doc = "RSTB2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstb2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstb2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstb2r`]
module"]
#[doc(alias = "RSTB2R")]
pub type Rstb2r = crate::Reg<rstb2r::Rstb2rSpec>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstb2r;
#[doc = "EEFBR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefbr1`]
module"]
#[doc(alias = "EEFBR1")]
pub type Eefbr1 = crate::Reg<eefbr1::Eefbr1Spec>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefbr1;
#[doc = "EEFBR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefbr2`]
module"]
#[doc(alias = "EEFBR2")]
pub type Eefbr2 = crate::Reg<eefbr2::Eefbr2Spec>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefbr2;
#[doc = "RSTBR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstbr`]
module"]
#[doc(alias = "RSTBR")]
pub type Rstbr = crate::Reg<rstbr::RstbrSpec>;
#[doc = "TimerA Reset Register"]
pub mod rstbr;
#[doc = "CHPBR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpbr`]
module"]
#[doc(alias = "CHPBR")]
pub type Chpbr = crate::Reg<chpbr::ChpbrSpec>;
#[doc = "Timerx Chopper Register"]
pub mod chpbr;
#[doc = "CPT1BCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1bcr`]
module"]
#[doc(alias = "CPT1BCR")]
pub type Cpt1bcr = crate::Reg<cpt1bcr::Cpt1bcrSpec>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1bcr;
#[doc = "CPT2BCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2bcr`]
module"]
#[doc(alias = "CPT2BCR")]
pub type Cpt2bcr = crate::Reg<cpt2bcr::Cpt2bcrSpec>;
#[doc = "CPT2xCR"]
pub mod cpt2bcr;
#[doc = "OUTBR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outbr`]
module"]
#[doc(alias = "OUTBR")]
pub type Outbr = crate::Reg<outbr::OutbrSpec>;
#[doc = "Timerx Output Register"]
pub mod outbr;
#[doc = "FLTBR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltbr`]
module"]
#[doc(alias = "FLTBR")]
pub type Fltbr = crate::Reg<fltbr::FltbrSpec>;
#[doc = "Timerx Fault Register"]
pub mod fltbr;
#[doc = "TIMBCR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timbcr2`]
module"]
#[doc(alias = "TIMBCR2")]
pub type Timbcr2 = crate::Reg<timbcr2::Timbcr2Spec>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timbcr2;
#[doc = "BEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`beefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`beefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@beefr3`]
module"]
#[doc(alias = "BEEFR3")]
pub type Beefr3 = crate::Reg<beefr3::Beefr3Spec>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod beefr3;
