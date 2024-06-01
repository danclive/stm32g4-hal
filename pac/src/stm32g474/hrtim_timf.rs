#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timfcr: Timfcr,
    timfisr: Timfisr,
    timficr: Timficr,
    timfdier: Timfdier,
    cntfr: Cntfr,
    perfr: Perfr,
    repfr: Repfr,
    cmp1fr: Cmp1fr,
    cmp1cfr: Cmp1cfr,
    cmp2fr: Cmp2fr,
    cmp3fr: Cmp3fr,
    cmp4fr: Cmp4fr,
    cpt1fr: Cpt1fr,
    cpt2fr: Cpt2fr,
    dtfr: Dtfr,
    setf1r: Setf1r,
    rste1r: Rste1r,
    setf2r: Setf2r,
    rstf2r: Rstf2r,
    eeffr1: Eeffr1,
    eeffr2: Eeffr2,
    rstfr: Rstfr,
    chpfr: Chpfr,
    cpt1fcr: Cpt1fcr,
    cpt2fcr: Cpt2fcr,
    outfr: Outfr,
    fltfr: Fltfr,
    timfcr2: Timfcr2,
    feefr3: Feefr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timfcr(&self) -> &Timfcr {
        &self.timfcr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timfisr(&self) -> &Timfisr {
        &self.timfisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timficr(&self) -> &Timficr {
        &self.timficr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timfdier(&self) -> &Timfdier {
        &self.timfdier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntfr(&self) -> &Cntfr {
        &self.cntfr
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perfr(&self) -> &Perfr {
        &self.perfr
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repfr(&self) -> &Repfr {
        &self.repfr
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1fr(&self) -> &Cmp1fr {
        &self.cmp1fr
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cfr(&self) -> &Cmp1cfr {
        &self.cmp1cfr
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2fr(&self) -> &Cmp2fr {
        &self.cmp2fr
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3fr(&self) -> &Cmp3fr {
        &self.cmp3fr
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4fr(&self) -> &Cmp4fr {
        &self.cmp4fr
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1fr(&self) -> &Cpt1fr {
        &self.cpt1fr
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2fr(&self) -> &Cpt2fr {
        &self.cpt2fr
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtfr(&self) -> &Dtfr {
        &self.dtfr
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn setf1r(&self) -> &Setf1r {
        &self.setf1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rste1r(&self) -> &Rste1r {
        &self.rste1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn setf2r(&self) -> &Setf2r {
        &self.setf2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rstf2r(&self) -> &Rstf2r {
        &self.rstf2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eeffr1(&self) -> &Eeffr1 {
        &self.eeffr1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eeffr2(&self) -> &Eeffr2 {
        &self.eeffr2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstfr(&self) -> &Rstfr {
        &self.rstfr
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpfr(&self) -> &Chpfr {
        &self.chpfr
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1fcr(&self) -> &Cpt1fcr {
        &self.cpt1fcr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2fcr(&self) -> &Cpt2fcr {
        &self.cpt2fcr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outfr(&self) -> &Outfr {
        &self.outfr
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltfr(&self) -> &Fltfr {
        &self.fltfr
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timfcr2(&self) -> &Timfcr2 {
        &self.timfcr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn feefr3(&self) -> &Feefr3 {
        &self.feefr3
    }
}
#[doc = "TIMFCR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfcr`]
module"]
#[doc(alias = "TIMFCR")]
pub type Timfcr = crate::Reg<timfcr::TimfcrSpec>;
#[doc = "Timerx Control Register"]
pub mod timfcr;
#[doc = "TIMFISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfisr`]
module"]
#[doc(alias = "TIMFISR")]
pub type Timfisr = crate::Reg<timfisr::TimfisrSpec>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timfisr;
#[doc = "TIMFICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timficr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timficr`]
module"]
#[doc(alias = "TIMFICR")]
pub type Timficr = crate::Reg<timficr::TimficrSpec>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timficr;
#[doc = "TIMFDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfdier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timfdier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfdier`]
module"]
#[doc(alias = "TIMFDIER")]
pub type Timfdier = crate::Reg<timfdier::TimfdierSpec>;
#[doc = "TIMxDIER"]
pub mod timfdier;
#[doc = "CNTFR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntfr`]
module"]
#[doc(alias = "CNTFR")]
pub type Cntfr = crate::Reg<cntfr::CntfrSpec>;
#[doc = "Timerx Counter Register"]
pub mod cntfr;
#[doc = "PERFR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perfr`]
module"]
#[doc(alias = "PERFR")]
pub type Perfr = crate::Reg<perfr::PerfrSpec>;
#[doc = "Timerx Period Register"]
pub mod perfr;
#[doc = "REPFR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repfr`]
module"]
#[doc(alias = "REPFR")]
pub type Repfr = crate::Reg<repfr::RepfrSpec>;
#[doc = "Timerx Repetition Register"]
pub mod repfr;
#[doc = "CMP1FR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1fr`]
module"]
#[doc(alias = "CMP1FR")]
pub type Cmp1fr = crate::Reg<cmp1fr::Cmp1frSpec>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1fr;
#[doc = "CMP1CFR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cfr`]
module"]
#[doc(alias = "CMP1CFR")]
pub type Cmp1cfr = crate::Reg<cmp1cfr::Cmp1cfrSpec>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cfr;
#[doc = "CMP2FR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2fr`]
module"]
#[doc(alias = "CMP2FR")]
pub type Cmp2fr = crate::Reg<cmp2fr::Cmp2frSpec>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2fr;
#[doc = "CMP3FR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3fr`]
module"]
#[doc(alias = "CMP3FR")]
pub type Cmp3fr = crate::Reg<cmp3fr::Cmp3frSpec>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3fr;
#[doc = "CMP4FR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4fr`]
module"]
#[doc(alias = "CMP4FR")]
pub type Cmp4fr = crate::Reg<cmp4fr::Cmp4frSpec>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4fr;
#[doc = "CPT1FR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1fr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1fr`]
module"]
#[doc(alias = "CPT1FR")]
pub type Cpt1fr = crate::Reg<cpt1fr::Cpt1frSpec>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1fr;
#[doc = "CPT2FR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2fr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2fr`]
module"]
#[doc(alias = "CPT2FR")]
pub type Cpt2fr = crate::Reg<cpt2fr::Cpt2frSpec>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2fr;
#[doc = "DTFR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfr`]
module"]
#[doc(alias = "DTFR")]
pub type Dtfr = crate::Reg<dtfr::DtfrSpec>;
#[doc = "Timerx Deadtime Register"]
pub mod dtfr;
#[doc = "SETF1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setf1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setf1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setf1r`]
module"]
#[doc(alias = "SETF1R")]
pub type Setf1r = crate::Reg<setf1r::Setf1rSpec>;
#[doc = "Timerx Output1 Set Register"]
pub mod setf1r;
#[doc = "RSTE1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rste1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rste1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rste1r`]
module"]
#[doc(alias = "RSTE1R")]
pub type Rste1r = crate::Reg<rste1r::Rste1rSpec>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "SETF2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setf2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setf2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setf2r`]
module"]
#[doc(alias = "SETF2R")]
pub type Setf2r = crate::Reg<setf2r::Setf2rSpec>;
#[doc = "Timerx Output2 Set Register"]
pub mod setf2r;
#[doc = "RSTF2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstf2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstf2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstf2r`]
module"]
#[doc(alias = "RSTF2R")]
pub type Rstf2r = crate::Reg<rstf2r::Rstf2rSpec>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstf2r;
#[doc = "EEFFR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeffr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeffr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeffr1`]
module"]
#[doc(alias = "EEFFR1")]
pub type Eeffr1 = crate::Reg<eeffr1::Eeffr1Spec>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eeffr1;
#[doc = "EEFFR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeffr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeffr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeffr2`]
module"]
#[doc(alias = "EEFFR2")]
pub type Eeffr2 = crate::Reg<eeffr2::Eeffr2Spec>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eeffr2;
#[doc = "RSTFR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstfr`]
module"]
#[doc(alias = "RSTFR")]
pub type Rstfr = crate::Reg<rstfr::RstfrSpec>;
#[doc = "TimerA Reset Register"]
pub mod rstfr;
#[doc = "CHPFR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpfr`]
module"]
#[doc(alias = "CHPFR")]
pub type Chpfr = crate::Reg<chpfr::ChpfrSpec>;
#[doc = "Timerx Chopper Register"]
pub mod chpfr;
#[doc = "CPT1FCR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1fcr`]
module"]
#[doc(alias = "CPT1FCR")]
pub type Cpt1fcr = crate::Reg<cpt1fcr::Cpt1fcrSpec>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1fcr;
#[doc = "CPT2FCR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2fcr`]
module"]
#[doc(alias = "CPT2FCR")]
pub type Cpt2fcr = crate::Reg<cpt2fcr::Cpt2fcrSpec>;
#[doc = "CPT2xCR"]
pub mod cpt2fcr;
#[doc = "OUTFR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfr`]
module"]
#[doc(alias = "OUTFR")]
pub type Outfr = crate::Reg<outfr::OutfrSpec>;
#[doc = "Timerx Output Register"]
pub mod outfr;
#[doc = "FLTFR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltfr`]
module"]
#[doc(alias = "FLTFR")]
pub type Fltfr = crate::Reg<fltfr::FltfrSpec>;
#[doc = "Timerx Fault Register"]
pub mod fltfr;
#[doc = "TIMFCR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timfcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timfcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timfcr2`]
module"]
#[doc(alias = "TIMFCR2")]
pub type Timfcr2 = crate::Reg<timfcr2::Timfcr2Spec>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timfcr2;
#[doc = "FEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feefr3`]
module"]
#[doc(alias = "FEEFR3")]
pub type Feefr3 = crate::Reg<feefr3::Feefr3Spec>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod feefr3;
