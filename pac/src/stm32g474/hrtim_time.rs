#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timecr: Timecr,
    timeisr: Timeisr,
    timeicr: Timeicr,
    timedier: Timedier,
    cnter: Cnter,
    perer: Perer,
    reper: Reper,
    cmp1er: Cmp1er,
    cmp1cer: Cmp1cer,
    cmp2er: Cmp2er,
    cmp3er: Cmp3er,
    cmp4er: Cmp4er,
    cpt1er: Cpt1er,
    cpt2er: Cpt2er,
    dter: Dter,
    sete1r: Sete1r,
    rste1r: Rste1r,
    sete2r: Sete2r,
    rste2r: Rste2r,
    eefer1: Eefer1,
    eefer2: Eefer2,
    rster: Rster,
    chper: Chper,
    cpt1ecr: Cpt1ecr,
    cpt2ecr: Cpt2ecr,
    outer: Outer,
    flter: Flter,
    timecr2: Timecr2,
    eeefr3: Eeefr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timecr(&self) -> &Timecr {
        &self.timecr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timeisr(&self) -> &Timeisr {
        &self.timeisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timeicr(&self) -> &Timeicr {
        &self.timeicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timedier(&self) -> &Timedier {
        &self.timedier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cnter(&self) -> &Cnter {
        &self.cnter
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perer(&self) -> &Perer {
        &self.perer
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn reper(&self) -> &Reper {
        &self.reper
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1er(&self) -> &Cmp1er {
        &self.cmp1er
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1cer(&self) -> &Cmp1cer {
        &self.cmp1cer
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2er(&self) -> &Cmp2er {
        &self.cmp2er
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3er(&self) -> &Cmp3er {
        &self.cmp3er
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4er(&self) -> &Cmp4er {
        &self.cmp4er
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1er(&self) -> &Cpt1er {
        &self.cpt1er
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2er(&self) -> &Cpt2er {
        &self.cpt2er
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dter(&self) -> &Dter {
        &self.dter
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn sete1r(&self) -> &Sete1r {
        &self.sete1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rste1r(&self) -> &Rste1r {
        &self.rste1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn sete2r(&self) -> &Sete2r {
        &self.sete2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rste2r(&self) -> &Rste2r {
        &self.rste2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefer1(&self) -> &Eefer1 {
        &self.eefer1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefer2(&self) -> &Eefer2 {
        &self.eefer2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rster(&self) -> &Rster {
        &self.rster
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chper(&self) -> &Chper {
        &self.chper
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1ecr(&self) -> &Cpt1ecr {
        &self.cpt1ecr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2ecr(&self) -> &Cpt2ecr {
        &self.cpt2ecr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outer(&self) -> &Outer {
        &self.outer
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn flter(&self) -> &Flter {
        &self.flter
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timecr2(&self) -> &Timecr2 {
        &self.timecr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn eeefr3(&self) -> &Eeefr3 {
        &self.eeefr3
    }
}
#[doc = "TIMECR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecr`]
module"]
#[doc(alias = "TIMECR")]
pub type Timecr = crate::Reg<timecr::TimecrSpec>;
#[doc = "Timerx Control Register"]
pub mod timecr;
#[doc = "TIMEISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeisr`]
module"]
#[doc(alias = "TIMEISR")]
pub type Timeisr = crate::Reg<timeisr::TimeisrSpec>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timeisr;
#[doc = "TIMEICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeicr`]
module"]
#[doc(alias = "TIMEICR")]
pub type Timeicr = crate::Reg<timeicr::TimeicrSpec>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timeicr;
#[doc = "TIMEDIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timedier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timedier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timedier`]
module"]
#[doc(alias = "TIMEDIER")]
pub type Timedier = crate::Reg<timedier::TimedierSpec>;
#[doc = "TIMxDIER"]
pub mod timedier;
#[doc = "CNTER (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnter`]
module"]
#[doc(alias = "CNTER")]
pub type Cnter = crate::Reg<cnter::CnterSpec>;
#[doc = "Timerx Counter Register"]
pub mod cnter;
#[doc = "PERER (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perer`]
module"]
#[doc(alias = "PERER")]
pub type Perer = crate::Reg<perer::PererSpec>;
#[doc = "Timerx Period Register"]
pub mod perer;
#[doc = "REPER (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reper`]
module"]
#[doc(alias = "REPER")]
pub type Reper = crate::Reg<reper::ReperSpec>;
#[doc = "Timerx Repetition Register"]
pub mod reper;
#[doc = "CMP1ER (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1er`]
module"]
#[doc(alias = "CMP1ER")]
pub type Cmp1er = crate::Reg<cmp1er::Cmp1erSpec>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1er;
#[doc = "CMP1CER (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1cer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1cer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1cer`]
module"]
#[doc(alias = "CMP1CER")]
pub type Cmp1cer = crate::Reg<cmp1cer::Cmp1cerSpec>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cer;
#[doc = "CMP2ER (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2er`]
module"]
#[doc(alias = "CMP2ER")]
pub type Cmp2er = crate::Reg<cmp2er::Cmp2erSpec>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2er;
#[doc = "CMP3ER (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3er`]
module"]
#[doc(alias = "CMP3ER")]
pub type Cmp3er = crate::Reg<cmp3er::Cmp3erSpec>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3er;
#[doc = "CMP4ER (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4er::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4er::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4er`]
module"]
#[doc(alias = "CMP4ER")]
pub type Cmp4er = crate::Reg<cmp4er::Cmp4erSpec>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4er;
#[doc = "CPT1ER (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1er::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1er`]
module"]
#[doc(alias = "CPT1ER")]
pub type Cpt1er = crate::Reg<cpt1er::Cpt1erSpec>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1er;
#[doc = "CPT2ER (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2er::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2er`]
module"]
#[doc(alias = "CPT2ER")]
pub type Cpt2er = crate::Reg<cpt2er::Cpt2erSpec>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2er;
#[doc = "DTER (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dter`]
module"]
#[doc(alias = "DTER")]
pub type Dter = crate::Reg<dter::DterSpec>;
#[doc = "Timerx Deadtime Register"]
pub mod dter;
#[doc = "SETE1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sete1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sete1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sete1r`]
module"]
#[doc(alias = "SETE1R")]
pub type Sete1r = crate::Reg<sete1r::Sete1rSpec>;
#[doc = "Timerx Output1 Set Register"]
pub mod sete1r;
#[doc = "RSTE1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rste1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rste1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rste1r`]
module"]
#[doc(alias = "RSTE1R")]
pub type Rste1r = crate::Reg<rste1r::Rste1rSpec>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "SETE2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sete2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sete2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sete2r`]
module"]
#[doc(alias = "SETE2R")]
pub type Sete2r = crate::Reg<sete2r::Sete2rSpec>;
#[doc = "Timerx Output2 Set Register"]
pub mod sete2r;
#[doc = "RSTE2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rste2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rste2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rste2r`]
module"]
#[doc(alias = "RSTE2R")]
pub type Rste2r = crate::Reg<rste2r::Rste2rSpec>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rste2r;
#[doc = "EEFER1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefer1`]
module"]
#[doc(alias = "EEFER1")]
pub type Eefer1 = crate::Reg<eefer1::Eefer1Spec>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefer1;
#[doc = "EEFER2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefer2`]
module"]
#[doc(alias = "EEFER2")]
pub type Eefer2 = crate::Reg<eefer2::Eefer2Spec>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefer2;
#[doc = "RSTER (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rster::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rster::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rster`]
module"]
#[doc(alias = "RSTER")]
pub type Rster = crate::Reg<rster::RsterSpec>;
#[doc = "TimerA Reset Register"]
pub mod rster;
#[doc = "CHPER (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chper`]
module"]
#[doc(alias = "CHPER")]
pub type Chper = crate::Reg<chper::ChperSpec>;
#[doc = "Timerx Chopper Register"]
pub mod chper;
#[doc = "CPT1ECR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1ecr`]
module"]
#[doc(alias = "CPT1ECR")]
pub type Cpt1ecr = crate::Reg<cpt1ecr::Cpt1ecrSpec>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ecr;
#[doc = "CPT2ECR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2ecr`]
module"]
#[doc(alias = "CPT2ECR")]
pub type Cpt2ecr = crate::Reg<cpt2ecr::Cpt2ecrSpec>;
#[doc = "CPT2xCR"]
pub mod cpt2ecr;
#[doc = "OUTER (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outer`]
module"]
#[doc(alias = "OUTER")]
pub type Outer = crate::Reg<outer::OuterSpec>;
#[doc = "Timerx Output Register"]
pub mod outer;
#[doc = "FLTER (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flter`]
module"]
#[doc(alias = "FLTER")]
pub type Flter = crate::Reg<flter::FlterSpec>;
#[doc = "Timerx Fault Register"]
pub mod flter;
#[doc = "TIMECR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecr2`]
module"]
#[doc(alias = "TIMECR2")]
pub type Timecr2 = crate::Reg<timecr2::Timecr2Spec>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timecr2;
#[doc = "EEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeefr3`]
module"]
#[doc(alias = "EEEFR3")]
pub type Eeefr3 = crate::Reg<eeefr3::Eeefr3Spec>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod eeefr3;
