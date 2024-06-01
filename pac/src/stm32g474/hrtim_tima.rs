#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timacr: Timacr,
    timaisr: Timaisr,
    timaicr: Timaicr,
    timadier: Timadier,
    cntar: Cntar,
    perar: Perar,
    repar: Repar,
    cmp1ar: Cmp1ar,
    cmp1car: Cmp1car,
    cmp2ar: Cmp2ar,
    cmp3ar: Cmp3ar,
    cmp4ar: Cmp4ar,
    cpt1ar: Cpt1ar,
    cpt2ar: Cpt2ar,
    dtar: Dtar,
    seta1r: Seta1r,
    rsta1r: Rsta1r,
    seta2r: Seta2r,
    rsta2r: Rsta2r,
    eefar1: Eefar1,
    eefar2: Eefar2,
    rstar: Rstar,
    chpar: Chpar,
    cpt1acr: Cpt1acr,
    cpt2acr: Cpt2acr,
    outar: Outar,
    fltar: Fltar,
    timacr2: Timacr2,
    aeefr3: Aeefr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    #[inline(always)]
    pub const fn timacr(&self) -> &Timacr {
        &self.timacr
    }
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    #[inline(always)]
    pub const fn timaisr(&self) -> &Timaisr {
        &self.timaisr
    }
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    #[inline(always)]
    pub const fn timaicr(&self) -> &Timaicr {
        &self.timaicr
    }
    #[doc = "0x0c - TIMxDIER"]
    #[inline(always)]
    pub const fn timadier(&self) -> &Timadier {
        &self.timadier
    }
    #[doc = "0x10 - Timerx Counter Register"]
    #[inline(always)]
    pub const fn cntar(&self) -> &Cntar {
        &self.cntar
    }
    #[doc = "0x14 - Timerx Period Register"]
    #[inline(always)]
    pub const fn perar(&self) -> &Perar {
        &self.perar
    }
    #[doc = "0x18 - Timerx Repetition Register"]
    #[inline(always)]
    pub const fn repar(&self) -> &Repar {
        &self.repar
    }
    #[doc = "0x1c - Timerx Compare 1 Register"]
    #[inline(always)]
    pub const fn cmp1ar(&self) -> &Cmp1ar {
        &self.cmp1ar
    }
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    #[inline(always)]
    pub const fn cmp1car(&self) -> &Cmp1car {
        &self.cmp1car
    }
    #[doc = "0x24 - Timerx Compare 2 Register"]
    #[inline(always)]
    pub const fn cmp2ar(&self) -> &Cmp2ar {
        &self.cmp2ar
    }
    #[doc = "0x28 - Timerx Compare 3 Register"]
    #[inline(always)]
    pub const fn cmp3ar(&self) -> &Cmp3ar {
        &self.cmp3ar
    }
    #[doc = "0x2c - Timerx Compare 4 Register"]
    #[inline(always)]
    pub const fn cmp4ar(&self) -> &Cmp4ar {
        &self.cmp4ar
    }
    #[doc = "0x30 - Timerx Capture 1 Register"]
    #[inline(always)]
    pub const fn cpt1ar(&self) -> &Cpt1ar {
        &self.cpt1ar
    }
    #[doc = "0x34 - Timerx Capture 2 Register"]
    #[inline(always)]
    pub const fn cpt2ar(&self) -> &Cpt2ar {
        &self.cpt2ar
    }
    #[doc = "0x38 - Timerx Deadtime Register"]
    #[inline(always)]
    pub const fn dtar(&self) -> &Dtar {
        &self.dtar
    }
    #[doc = "0x3c - Timerx Output1 Set Register"]
    #[inline(always)]
    pub const fn seta1r(&self) -> &Seta1r {
        &self.seta1r
    }
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    #[inline(always)]
    pub const fn rsta1r(&self) -> &Rsta1r {
        &self.rsta1r
    }
    #[doc = "0x44 - Timerx Output2 Set Register"]
    #[inline(always)]
    pub const fn seta2r(&self) -> &Seta2r {
        &self.seta2r
    }
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    #[inline(always)]
    pub const fn rsta2r(&self) -> &Rsta2r {
        &self.rsta2r
    }
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    #[inline(always)]
    pub const fn eefar1(&self) -> &Eefar1 {
        &self.eefar1
    }
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    #[inline(always)]
    pub const fn eefar2(&self) -> &Eefar2 {
        &self.eefar2
    }
    #[doc = "0x54 - TimerA Reset Register"]
    #[inline(always)]
    pub const fn rstar(&self) -> &Rstar {
        &self.rstar
    }
    #[doc = "0x58 - Timerx Chopper Register"]
    #[inline(always)]
    pub const fn chpar(&self) -> &Chpar {
        &self.chpar
    }
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    #[inline(always)]
    pub const fn cpt1acr(&self) -> &Cpt1acr {
        &self.cpt1acr
    }
    #[doc = "0x60 - CPT2xCR"]
    #[inline(always)]
    pub const fn cpt2acr(&self) -> &Cpt2acr {
        &self.cpt2acr
    }
    #[doc = "0x64 - Timerx Output Register"]
    #[inline(always)]
    pub const fn outar(&self) -> &Outar {
        &self.outar
    }
    #[doc = "0x68 - Timerx Fault Register"]
    #[inline(always)]
    pub const fn fltar(&self) -> &Fltar {
        &self.fltar
    }
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    #[inline(always)]
    pub const fn timacr2(&self) -> &Timacr2 {
        &self.timacr2
    }
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    #[inline(always)]
    pub const fn aeefr3(&self) -> &Aeefr3 {
        &self.aeefr3
    }
}
#[doc = "TIMACR (rw) register accessor: Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timacr`]
module"]
#[doc(alias = "TIMACR")]
pub type Timacr = crate::Reg<timacr::TimacrSpec>;
#[doc = "Timerx Control Register"]
pub mod timacr;
#[doc = "TIMAISR (r) register accessor: Timerx Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timaisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timaisr`]
module"]
#[doc(alias = "TIMAISR")]
pub type Timaisr = crate::Reg<timaisr::TimaisrSpec>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timaisr;
#[doc = "TIMAICR (w) register accessor: Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timaicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timaicr`]
module"]
#[doc(alias = "TIMAICR")]
pub type Timaicr = crate::Reg<timaicr::TimaicrSpec>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timaicr;
#[doc = "TIMADIER (rw) register accessor: TIMxDIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timadier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timadier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timadier`]
module"]
#[doc(alias = "TIMADIER")]
pub type Timadier = crate::Reg<timadier::TimadierSpec>;
#[doc = "TIMxDIER"]
pub mod timadier;
#[doc = "CNTAR (rw) register accessor: Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntar`]
module"]
#[doc(alias = "CNTAR")]
pub type Cntar = crate::Reg<cntar::CntarSpec>;
#[doc = "Timerx Counter Register"]
pub mod cntar;
#[doc = "PERAR (rw) register accessor: Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perar`]
module"]
#[doc(alias = "PERAR")]
pub type Perar = crate::Reg<perar::PerarSpec>;
#[doc = "Timerx Period Register"]
pub mod perar;
#[doc = "REPAR (rw) register accessor: Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@repar`]
module"]
#[doc(alias = "REPAR")]
pub type Repar = crate::Reg<repar::ReparSpec>;
#[doc = "Timerx Repetition Register"]
pub mod repar;
#[doc = "CMP1AR (rw) register accessor: Timerx Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1ar`]
module"]
#[doc(alias = "CMP1AR")]
pub type Cmp1ar = crate::Reg<cmp1ar::Cmp1arSpec>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1ar;
#[doc = "CMP1CAR (rw) register accessor: Timerx Compare 1 Compound Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1car::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1car::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1car`]
module"]
#[doc(alias = "CMP1CAR")]
pub type Cmp1car = crate::Reg<cmp1car::Cmp1carSpec>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1car;
#[doc = "CMP2AR (rw) register accessor: Timerx Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp2ar`]
module"]
#[doc(alias = "CMP2AR")]
pub type Cmp2ar = crate::Reg<cmp2ar::Cmp2arSpec>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2ar;
#[doc = "CMP3AR (rw) register accessor: Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp3ar`]
module"]
#[doc(alias = "CMP3AR")]
pub type Cmp3ar = crate::Reg<cmp3ar::Cmp3arSpec>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3ar;
#[doc = "CMP4AR (rw) register accessor: Timerx Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp4ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp4ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp4ar`]
module"]
#[doc(alias = "CMP4AR")]
pub type Cmp4ar = crate::Reg<cmp4ar::Cmp4arSpec>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4ar;
#[doc = "CPT1AR (r) register accessor: Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1ar`]
module"]
#[doc(alias = "CPT1AR")]
pub type Cpt1ar = crate::Reg<cpt1ar::Cpt1arSpec>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1ar;
#[doc = "CPT2AR (r) register accessor: Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2ar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2ar`]
module"]
#[doc(alias = "CPT2AR")]
pub type Cpt2ar = crate::Reg<cpt2ar::Cpt2arSpec>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2ar;
#[doc = "DTAR (rw) register accessor: Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtar`]
module"]
#[doc(alias = "DTAR")]
pub type Dtar = crate::Reg<dtar::DtarSpec>;
#[doc = "Timerx Deadtime Register"]
pub mod dtar;
#[doc = "SETA1R (rw) register accessor: Timerx Output1 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seta1r`]
module"]
#[doc(alias = "SETA1R")]
pub type Seta1r = crate::Reg<seta1r::Seta1rSpec>;
#[doc = "Timerx Output1 Set Register"]
pub mod seta1r;
#[doc = "RSTA1R (rw) register accessor: Timerx Output1 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsta1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsta1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsta1r`]
module"]
#[doc(alias = "RSTA1R")]
pub type Rsta1r = crate::Reg<rsta1r::Rsta1rSpec>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rsta1r;
#[doc = "SETA2R (rw) register accessor: Timerx Output2 Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seta2r`]
module"]
#[doc(alias = "SETA2R")]
pub type Seta2r = crate::Reg<seta2r::Seta2rSpec>;
#[doc = "Timerx Output2 Set Register"]
pub mod seta2r;
#[doc = "RSTA2R (rw) register accessor: Timerx Output2 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsta2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsta2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsta2r`]
module"]
#[doc(alias = "RSTA2R")]
pub type Rsta2r = crate::Reg<rsta2r::Rsta2rSpec>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rsta2r;
#[doc = "EEFAR1 (rw) register accessor: Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefar1`]
module"]
#[doc(alias = "EEFAR1")]
pub type Eefar1 = crate::Reg<eefar1::Eefar1Spec>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefar1;
#[doc = "EEFAR2 (rw) register accessor: Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eefar2`]
module"]
#[doc(alias = "EEFAR2")]
pub type Eefar2 = crate::Reg<eefar2::Eefar2Spec>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefar2;
#[doc = "RSTAR (rw) register accessor: TimerA Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstar`]
module"]
#[doc(alias = "RSTAR")]
pub type Rstar = crate::Reg<rstar::RstarSpec>;
#[doc = "TimerA Reset Register"]
pub mod rstar;
#[doc = "CHPAR (rw) register accessor: Timerx Chopper Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chpar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chpar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chpar`]
module"]
#[doc(alias = "CHPAR")]
pub type Chpar = crate::Reg<chpar::ChparSpec>;
#[doc = "Timerx Chopper Register"]
pub mod chpar;
#[doc = "CPT1ACR (rw) register accessor: Timerx Capture 2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt1acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt1acr`]
module"]
#[doc(alias = "CPT1ACR")]
pub type Cpt1acr = crate::Reg<cpt1acr::Cpt1acrSpec>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1acr;
#[doc = "CPT2ACR (rw) register accessor: CPT2xCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpt2acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpt2acr`]
module"]
#[doc(alias = "CPT2ACR")]
pub type Cpt2acr = crate::Reg<cpt2acr::Cpt2acrSpec>;
#[doc = "CPT2xCR"]
pub mod cpt2acr;
#[doc = "OUTAR (rw) register accessor: Timerx Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outar`]
module"]
#[doc(alias = "OUTAR")]
pub type Outar = crate::Reg<outar::OutarSpec>;
#[doc = "Timerx Output Register"]
pub mod outar;
#[doc = "FLTAR (rw) register accessor: Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltar`]
module"]
#[doc(alias = "FLTAR")]
pub type Fltar = crate::Reg<fltar::FltarSpec>;
#[doc = "Timerx Fault Register"]
pub mod fltar;
#[doc = "TIMACR2 (rw) register accessor: HRTIM Timerx Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timacr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timacr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timacr2`]
module"]
#[doc(alias = "TIMACR2")]
pub type Timacr2 = crate::Reg<timacr2::Timacr2Spec>;
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timacr2;
#[doc = "AEEFR3 (rw) register accessor: HRTIM Timerx External Event Filtering Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeefr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeefr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeefr3`]
module"]
#[doc(alias = "AEEFR3")]
pub type Aeefr3 = crate::Reg<aeefr3::Aeefr3Spec>;
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod aeefr3;
