#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    isr: Isr,
    icr: Icr,
    ier: Ier,
    oenr: Oenr,
    odisr: Odisr,
    odsr: Odsr,
    bmcr: Bmcr,
    bmtrg: Bmtrg,
    bmcmpr: Bmcmpr,
    bmper: Bmper,
    eecr1: Eecr1,
    eecr2: Eecr2,
    eecr3: Eecr3,
    adc1r: Adc1r,
    adc2r: Adc2r,
    adc3r: Adc3r,
    adc4r: Adc4r,
    dllcr: Dllcr,
    fltinr1: Fltinr1,
    fltinr2: Fltinr2,
    bdmupdr: Bdmupdr,
    bdtaupr: Bdtaupr,
    bdtbupr: Bdtbupr,
    bdtcupr: Bdtcupr,
    bdtdupr: Bdtdupr,
    bdteupr: Bdteupr,
    bdmadr: Bdmadr,
    bdtfupr: Bdtfupr,
    adcer: Adcer,
    adcur: Adcur,
    adcps1: Adcps1,
    adcps2: Adcps2,
    fltinr3: Fltinr3,
    fltinr4: Fltinr4,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Control Register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x0c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Output Enable Register"]
    #[inline(always)]
    pub const fn oenr(&self) -> &Oenr {
        &self.oenr
    }
    #[doc = "0x18 - ODISR"]
    #[inline(always)]
    pub const fn odisr(&self) -> &Odisr {
        &self.odisr
    }
    #[doc = "0x1c - Output Disable Status Register"]
    #[inline(always)]
    pub const fn odsr(&self) -> &Odsr {
        &self.odsr
    }
    #[doc = "0x20 - Burst Mode Control Register"]
    #[inline(always)]
    pub const fn bmcr(&self) -> &Bmcr {
        &self.bmcr
    }
    #[doc = "0x24 - BMTRG"]
    #[inline(always)]
    pub const fn bmtrg(&self) -> &Bmtrg {
        &self.bmtrg
    }
    #[doc = "0x28 - BMCMPR"]
    #[inline(always)]
    pub const fn bmcmpr(&self) -> &Bmcmpr {
        &self.bmcmpr
    }
    #[doc = "0x2c - Burst Mode Period Register"]
    #[inline(always)]
    pub const fn bmper(&self) -> &Bmper {
        &self.bmper
    }
    #[doc = "0x30 - Timer External Event Control Register 1"]
    #[inline(always)]
    pub const fn eecr1(&self) -> &Eecr1 {
        &self.eecr1
    }
    #[doc = "0x34 - Timer External Event Control Register 2"]
    #[inline(always)]
    pub const fn eecr2(&self) -> &Eecr2 {
        &self.eecr2
    }
    #[doc = "0x38 - Timer External Event Control Register 3"]
    #[inline(always)]
    pub const fn eecr3(&self) -> &Eecr3 {
        &self.eecr3
    }
    #[doc = "0x3c - ADC Trigger 1 Register"]
    #[inline(always)]
    pub const fn adc1r(&self) -> &Adc1r {
        &self.adc1r
    }
    #[doc = "0x40 - ADC Trigger 2 Register"]
    #[inline(always)]
    pub const fn adc2r(&self) -> &Adc2r {
        &self.adc2r
    }
    #[doc = "0x44 - ADC Trigger 3 Register"]
    #[inline(always)]
    pub const fn adc3r(&self) -> &Adc3r {
        &self.adc3r
    }
    #[doc = "0x48 - ADC Trigger 4 Register"]
    #[inline(always)]
    pub const fn adc4r(&self) -> &Adc4r {
        &self.adc4r
    }
    #[doc = "0x4c - DLL Control Register"]
    #[inline(always)]
    pub const fn dllcr(&self) -> &Dllcr {
        &self.dllcr
    }
    #[doc = "0x50 - HRTIM Fault Input Register 1"]
    #[inline(always)]
    pub const fn fltinr1(&self) -> &Fltinr1 {
        &self.fltinr1
    }
    #[doc = "0x54 - HRTIM Fault Input Register 2"]
    #[inline(always)]
    pub const fn fltinr2(&self) -> &Fltinr2 {
        &self.fltinr2
    }
    #[doc = "0x58 - BDMUPDR"]
    #[inline(always)]
    pub const fn bdmupdr(&self) -> &Bdmupdr {
        &self.bdmupdr
    }
    #[doc = "0x5c - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtaupr(&self) -> &Bdtaupr {
        &self.bdtaupr
    }
    #[doc = "0x60 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtbupr(&self) -> &Bdtbupr {
        &self.bdtbupr
    }
    #[doc = "0x64 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtcupr(&self) -> &Bdtcupr {
        &self.bdtcupr
    }
    #[doc = "0x68 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtdupr(&self) -> &Bdtdupr {
        &self.bdtdupr
    }
    #[doc = "0x6c - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdteupr(&self) -> &Bdteupr {
        &self.bdteupr
    }
    #[doc = "0x70 - Burst DMA Data Register"]
    #[inline(always)]
    pub const fn bdmadr(&self) -> &Bdmadr {
        &self.bdmadr
    }
    #[doc = "0x74 - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtfupr(&self) -> &Bdtfupr {
        &self.bdtfupr
    }
    #[doc = "0x78 - HRTIM ADC Extended Trigger Register"]
    #[inline(always)]
    pub const fn adcer(&self) -> &Adcer {
        &self.adcer
    }
    #[doc = "0x7c - HRTIM ADC Trigger Update Register"]
    #[inline(always)]
    pub const fn adcur(&self) -> &Adcur {
        &self.adcur
    }
    #[doc = "0x80 - HRTIM ADC Post Scaler Register 1"]
    #[inline(always)]
    pub const fn adcps1(&self) -> &Adcps1 {
        &self.adcps1
    }
    #[doc = "0x84 - HRTIM ADC Post Scaler Register 2"]
    #[inline(always)]
    pub const fn adcps2(&self) -> &Adcps2 {
        &self.adcps2
    }
    #[doc = "0x88 - HRTIM Fault Input Register 3"]
    #[inline(always)]
    pub const fn fltinr3(&self) -> &Fltinr3 {
        &self.fltinr3
    }
    #[doc = "0x8c - HRTIM Fault Input Register 4"]
    #[inline(always)]
    pub const fn fltinr4(&self) -> &Fltinr4 {
        &self.fltinr4
    }
}
#[doc = "CR1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "OENR (rw) register accessor: Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oenr`]
module"]
#[doc(alias = "OENR")]
pub type Oenr = crate::Reg<oenr::OenrSpec>;
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "ODISR (w) register accessor: ODISR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odisr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odisr`]
module"]
#[doc(alias = "ODISR")]
pub type Odisr = crate::Reg<odisr::OdisrSpec>;
#[doc = "ODISR"]
pub mod odisr;
#[doc = "ODSR (r) register accessor: Output Disable Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odsr`]
module"]
#[doc(alias = "ODSR")]
pub type Odsr = crate::Reg<odsr::OdsrSpec>;
#[doc = "Output Disable Status Register"]
pub mod odsr;
#[doc = "BMCR (rw) register accessor: Burst Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcr`]
module"]
#[doc(alias = "BMCR")]
pub type Bmcr = crate::Reg<bmcr::BmcrSpec>;
#[doc = "Burst Mode Control Register"]
pub mod bmcr;
#[doc = "BMTRG (rw) register accessor: BMTRG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmtrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmtrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmtrg`]
module"]
#[doc(alias = "BMTRG")]
pub type Bmtrg = crate::Reg<bmtrg::BmtrgSpec>;
#[doc = "BMTRG"]
pub mod bmtrg;
#[doc = "BMCMPR (rw) register accessor: BMCMPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcmpr`]
module"]
#[doc(alias = "BMCMPR")]
pub type Bmcmpr = crate::Reg<bmcmpr::BmcmprSpec>;
#[doc = "BMCMPR"]
pub mod bmcmpr;
#[doc = "BMPER (rw) register accessor: Burst Mode Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmper`]
module"]
#[doc(alias = "BMPER")]
pub type Bmper = crate::Reg<bmper::BmperSpec>;
#[doc = "Burst Mode Period Register"]
pub mod bmper;
#[doc = "EECR1 (rw) register accessor: Timer External Event Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr1`]
module"]
#[doc(alias = "EECR1")]
pub type Eecr1 = crate::Reg<eecr1::Eecr1Spec>;
#[doc = "Timer External Event Control Register 1"]
pub mod eecr1;
#[doc = "EECR2 (rw) register accessor: Timer External Event Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr2`]
module"]
#[doc(alias = "EECR2")]
pub type Eecr2 = crate::Reg<eecr2::Eecr2Spec>;
#[doc = "Timer External Event Control Register 2"]
pub mod eecr2;
#[doc = "EECR3 (rw) register accessor: Timer External Event Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr3`]
module"]
#[doc(alias = "EECR3")]
pub type Eecr3 = crate::Reg<eecr3::Eecr3Spec>;
#[doc = "Timer External Event Control Register 3"]
pub mod eecr3;
#[doc = "ADC1R (rw) register accessor: ADC Trigger 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1r`]
module"]
#[doc(alias = "ADC1R")]
pub type Adc1r = crate::Reg<adc1r::Adc1rSpec>;
#[doc = "ADC Trigger 1 Register"]
pub mod adc1r;
#[doc = "ADC2R (rw) register accessor: ADC Trigger 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc2r`]
module"]
#[doc(alias = "ADC2R")]
pub type Adc2r = crate::Reg<adc2r::Adc2rSpec>;
#[doc = "ADC Trigger 2 Register"]
pub mod adc2r;
#[doc = "ADC3R (rw) register accessor: ADC Trigger 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc3r`]
module"]
#[doc(alias = "ADC3R")]
pub type Adc3r = crate::Reg<adc3r::Adc3rSpec>;
#[doc = "ADC Trigger 3 Register"]
pub mod adc3r;
#[doc = "ADC4R (rw) register accessor: ADC Trigger 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc4r`]
module"]
#[doc(alias = "ADC4R")]
pub type Adc4r = crate::Reg<adc4r::Adc4rSpec>;
#[doc = "ADC Trigger 4 Register"]
pub mod adc4r;
#[doc = "DLLCR (rw) register accessor: DLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dllcr`]
module"]
#[doc(alias = "DLLCR")]
pub type Dllcr = crate::Reg<dllcr::DllcrSpec>;
#[doc = "DLL Control Register"]
pub mod dllcr;
#[doc = "FLTINR1 (rw) register accessor: HRTIM Fault Input Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr1`]
module"]
#[doc(alias = "FLTINR1")]
pub type Fltinr1 = crate::Reg<fltinr1::Fltinr1Spec>;
#[doc = "HRTIM Fault Input Register 1"]
pub mod fltinr1;
#[doc = "FLTINR2 (rw) register accessor: HRTIM Fault Input Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr2`]
module"]
#[doc(alias = "FLTINR2")]
pub type Fltinr2 = crate::Reg<fltinr2::Fltinr2Spec>;
#[doc = "HRTIM Fault Input Register 2"]
pub mod fltinr2;
#[doc = "BDMUPDR (rw) register accessor: BDMUPDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdmupdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmupdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmupdr`]
module"]
#[doc(alias = "BDMUPDR")]
pub type Bdmupdr = crate::Reg<bdmupdr::BdmupdrSpec>;
#[doc = "BDMUPDR"]
pub mod bdmupdr;
#[doc = "BDTAUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtaupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtaupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtaupr`]
module"]
#[doc(alias = "BDTAUPR")]
pub type Bdtaupr = crate::Reg<bdtaupr::BdtauprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtaupr;
#[doc = "BDTBUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtbupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtbupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtbupr`]
module"]
#[doc(alias = "BDTBUPR")]
pub type Bdtbupr = crate::Reg<bdtbupr::BdtbuprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtbupr;
#[doc = "BDTCUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtcupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtcupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtcupr`]
module"]
#[doc(alias = "BDTCUPR")]
pub type Bdtcupr = crate::Reg<bdtcupr::BdtcuprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtcupr;
#[doc = "BDTDUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtdupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtdupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtdupr`]
module"]
#[doc(alias = "BDTDUPR")]
pub type Bdtdupr = crate::Reg<bdtdupr::BdtduprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtdupr;
#[doc = "BDTEUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdteupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdteupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdteupr`]
module"]
#[doc(alias = "BDTEUPR")]
pub type Bdteupr = crate::Reg<bdteupr::BdteuprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdteupr;
#[doc = "BDTFUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtfupr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtfupr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtfupr`]
module"]
#[doc(alias = "BDTFUPR")]
pub type Bdtfupr = crate::Reg<bdtfupr::BdtfuprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtfupr;
#[doc = "BDMADR (w) register accessor: Burst DMA Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmadr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmadr`]
module"]
#[doc(alias = "BDMADR")]
pub type Bdmadr = crate::Reg<bdmadr::BdmadrSpec>;
#[doc = "Burst DMA Data Register"]
pub mod bdmadr;
#[doc = "ADCER (rw) register accessor: HRTIM ADC Extended Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcer`]
module"]
#[doc(alias = "ADCER")]
pub type Adcer = crate::Reg<adcer::AdcerSpec>;
#[doc = "HRTIM ADC Extended Trigger Register"]
pub mod adcer;
#[doc = "ADCUR (rw) register accessor: HRTIM ADC Trigger Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcur`]
module"]
#[doc(alias = "ADCUR")]
pub type Adcur = crate::Reg<adcur::AdcurSpec>;
#[doc = "HRTIM ADC Trigger Update Register"]
pub mod adcur;
#[doc = "ADCPS1 (rw) register accessor: HRTIM ADC Post Scaler Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcps1`]
module"]
#[doc(alias = "ADCPS1")]
pub type Adcps1 = crate::Reg<adcps1::Adcps1Spec>;
#[doc = "HRTIM ADC Post Scaler Register 1"]
pub mod adcps1;
#[doc = "ADCPS2 (rw) register accessor: HRTIM ADC Post Scaler Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcps2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcps2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcps2`]
module"]
#[doc(alias = "ADCPS2")]
pub type Adcps2 = crate::Reg<adcps2::Adcps2Spec>;
#[doc = "HRTIM ADC Post Scaler Register 2"]
pub mod adcps2;
#[doc = "FLTINR3 (rw) register accessor: HRTIM Fault Input Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr3`]
module"]
#[doc(alias = "FLTINR3")]
pub type Fltinr3 = crate::Reg<fltinr3::Fltinr3Spec>;
#[doc = "HRTIM Fault Input Register 3"]
pub mod fltinr3;
#[doc = "FLTINR4 (rw) register accessor: HRTIM Fault Input Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr4`]
module"]
#[doc(alias = "FLTINR4")]
pub type Fltinr4 = crate::Reg<fltinr4::Fltinr4Spec>;
#[doc = "HRTIM Fault Input Register 4"]
pub mod fltinr4;
