#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timccr: Timccr,
    timcisr: Timcisr,
    timcicr: Timcicr,
    timcdier: Timcdier,
    cntcr: Cntcr,
    percr: Percr,
    repcr: Repcr,
    cmp1cr: Cmp1cr,
    cmp1ccr: Cmp1ccr,
    cmp2cr: Cmp2cr,
    cmp3cr: Cmp3cr,
    cmp4cr: Cmp4cr,
    cpt1cr: Cpt1cr,
    cpt2cr: Cpt2cr,
    dtcr: Dtcr,
    setc1r: Setc1r,
    rstc1r: Rstc1r,
    setc2r: Setc2r,
    rstc2r: Rstc2r,
    eefcr1: Eefcr1,
    eefcr2: Eefcr2,
    rstcr: Rstcr,
    chpcr: Chpcr,
    cpt1ccr: Cpt1ccr,
    cpt2ccr: Cpt2ccr,
    outcr: Outcr,
    fltcr: Fltcr,
    timccr2: Timccr2,
    ceefr3: Ceefr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timccr(&self) -> &Timccr {
        &self.timccr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timcisr(&self) -> &Timcisr {
        &self.timcisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timcicr(&self) -> &Timcicr {
        &self.timcicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timcdier(&self) -> &Timcdier {
        &self.timcdier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntcr(&self) -> &Cntcr {
        &self.cntcr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn percr(&self) -> &Percr {
        &self.percr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repcr(&self) -> &Repcr {
        &self.repcr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1cr(&self) -> &Cmp1cr {
        &self.cmp1cr
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1ccr(&self) -> &Cmp1ccr {
        &self.cmp1ccr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2cr(&self) -> &Cmp2cr {
        &self.cmp2cr
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3cr(&self) -> &Cmp3cr {
        &self.cmp3cr
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4cr(&self) -> &Cmp4cr {
        &self.cmp4cr
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1cr(&self) -> &Cpt1cr {
        &self.cpt1cr
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2cr(&self) -> &Cpt2cr {
        &self.cpt2cr
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtcr(&self) -> &Dtcr {
        &self.dtcr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setc1r(&self) -> &Setc1r {
        &self.setc1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rstc1r(&self) -> &Rstc1r {
        &self.rstc1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setc2r(&self) -> &Setc2r {
        &self.setc2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstc2r(&self) -> &Rstc2r {
        &self.rstc2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefcr1(&self) -> &Eefcr1 {
        &self.eefcr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefcr2(&self) -> &Eefcr2 {
        &self.eefcr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstcr(&self) -> &Rstcr {
        &self.rstcr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpcr(&self) -> &Chpcr {
        &self.chpcr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1ccr(&self) -> &Cpt1ccr {
        &self.cpt1ccr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2ccr(&self) -> &Cpt2ccr {
        &self.cpt2ccr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outcr(&self) -> &Outcr {
        &self.outcr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltcr(&self) -> &Fltcr {
        &self.fltcr
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timccr2(&self) -> &Timccr2 {
        &self.timccr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn ceefr3(&self) -> &Ceefr3 {
        &self.ceefr3
    }
}
#[doc = "TIMCCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timccr`]
module"]
#[doc(alias = "TIMCCR")]
pub type Timccr = crate::Reg<timccr::TimccrSpec>;
#[doc = "Timerx Control Register"]
pub mod timccr;
#[doc = "TIMCISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timcisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timcisr`]
module"]
#[doc(alias = "TIMCISR")]
pub type Timcisr = crate::Reg<timcisr::TimcisrSpec>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timcisr;
#[doc = "TIMCICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timcicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timcicr`]
module"]
#[doc(alias = "TIMCICR")]
pub type Timcicr = crate::Reg<timcicr::TimcicrSpec>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timcicr;
#[doc = "TIMCDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timcdier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timcdier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timcdier`]
module"]
#[doc(alias = "TIMCDIER")]
pub type Timcdier = crate::Reg<timcdier::TimcdierSpec>;
#[doc = "TIMxDIER"]
pub mod timcdier;
#[doc = "CNTCR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcr`]
module"]
#[doc(alias = "CNTCR")]
pub type Cntcr = crate::Reg<cntcr::CntcrSpec>;
#[doc = "Timerx Counter Register"]
pub mod cntcr;
#[doc = "PERCR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`percr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`percr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@percr`]
module"]
#[doc(alias = "PERCR")]
pub type Percr = crate::Reg<percr::PercrSpec>;
#[doc = "Timerx Period Register"]
pub mod percr;
#[doc = "REPCR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repcr`]
module"]
#[doc(alias = "REPCR")]
pub type Repcr = crate::Reg<repcr::RepcrSpec>;
#[doc = "Timerx Repetition Register"]
pub mod repcr;
#[doc = "CMP1CR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cr`]
module"]
#[doc(alias = "CMP1CR")]
pub type Cmp1cr = crate::Reg<cmp1cr::Cmp1crSpec>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1cr;
#[doc = "CMP1CCR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1ccr`]
module"]
#[doc(alias = "CMP1CCR")]
pub type Cmp1ccr = crate::Reg<cmp1ccr::Cmp1ccrSpec>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1ccr;
#[doc = "CMP2CR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2cr`]
module"]
#[doc(alias = "CMP2CR")]
pub type Cmp2cr = crate::Reg<cmp2cr::Cmp2crSpec>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2cr;
#[doc = "CMP3CR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3cr`]
module"]
#[doc(alias = "CMP3CR")]
pub type Cmp3cr = crate::Reg<cmp3cr::Cmp3crSpec>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3cr;
#[doc = "CMP4CR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4cr`]
module"]
#[doc(alias = "CMP4CR")]
pub type Cmp4cr = crate::Reg<cmp4cr::Cmp4crSpec>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4cr;
#[doc = "CPT1CR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1cr`]
module"]
#[doc(alias = "CPT1CR")]
pub type Cpt1cr = crate::Reg<cpt1cr::Cpt1crSpec>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1cr;
#[doc = "CPT2CR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2cr`]
module"]
#[doc(alias = "CPT2CR")]
pub type Cpt2cr = crate::Reg<cpt2cr::Cpt2crSpec>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2cr;
#[doc = "DTCR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcr`]
module"]
#[doc(alias = "DTCR")]
pub type Dtcr = crate::Reg<dtcr::DtcrSpec>;
#[doc = "Timerx Deadtime Register"]
pub mod dtcr;
#[doc = "SETC1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setc1r`]
module"]
#[doc(alias = "SETC1R")]
pub type Setc1r = crate::Reg<setc1r::Setc1rSpec>;
#[doc = "Timerx Output1 Set Register"]
pub mod setc1r;
#[doc = "RSTC1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstc1r`]
module"]
#[doc(alias = "RSTC1R")]
pub type Rstc1r = crate::Reg<rstc1r::Rstc1rSpec>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstc1r;
#[doc = "SETC2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setc2r`]
module"]
#[doc(alias = "SETC2R")]
pub type Setc2r = crate::Reg<setc2r::Setc2rSpec>;
#[doc = "Timerx Output2 Set Register"]
pub mod setc2r;
#[doc = "RSTC2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstc2r`]
module"]
#[doc(alias = "RSTC2R")]
pub type Rstc2r = crate::Reg<rstc2r::Rstc2rSpec>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstc2r;
#[doc = "EEFCR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefcr1`]
module"]
#[doc(alias = "EEFCR1")]
pub type Eefcr1 = crate::Reg<eefcr1::Eefcr1Spec>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefcr1;
#[doc = "EEFCR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefcr2`]
module"]
#[doc(alias = "EEFCR2")]
pub type Eefcr2 = crate::Reg<eefcr2::Eefcr2Spec>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefcr2;
#[doc = "RSTCR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcr`]
module"]
#[doc(alias = "RSTCR")]
pub type Rstcr = crate::Reg<rstcr::RstcrSpec>;
#[doc = "TimerA Reset Register"]
pub mod rstcr;
#[doc = "CHPCR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpcr`]
module"]
#[doc(alias = "CHPCR")]
pub type Chpcr = crate::Reg<chpcr::ChpcrSpec>;
#[doc = "Timerx Chopper Register"]
pub mod chpcr;
#[doc = "CPT1CCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1ccr`]
module"]
#[doc(alias = "CPT1CCR")]
pub type Cpt1ccr = crate::Reg<cpt1ccr::Cpt1ccrSpec>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ccr;
#[doc = "CPT2CCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2ccr`]
module"]
#[doc(alias = "CPT2CCR")]
pub type Cpt2ccr = crate::Reg<cpt2ccr::Cpt2ccrSpec>;
#[doc = "CPT2xCR"]
pub mod cpt2ccr;
#[doc = "OUTCR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcr`]
module"]
#[doc(alias = "OUTCR")]
pub type Outcr = crate::Reg<outcr::OutcrSpec>;
#[doc = "Timerx Output Register"]
pub mod outcr;
#[doc = "FLTCR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltcr`]
module"]
#[doc(alias = "FLTCR")]
pub type Fltcr = crate::Reg<fltcr::FltcrSpec>;
#[doc = "Timerx Fault Register"]
pub mod fltcr;
#[doc = "TIMCCR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timccr2`]
module"]
#[doc(alias = "TIMCCR2")]
pub type Timccr2 = crate::Reg<timccr2::Timccr2Spec>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timccr2;
#[doc = "CEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ceefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ceefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ceefr3`]
module"]
#[doc(alias = "CEEFR3")]
pub type Ceefr3 = crate::Reg<ceefr3::Ceefr3Spec>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod ceefr3;
