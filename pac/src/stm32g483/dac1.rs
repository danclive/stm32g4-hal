#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dac_cr: DacCr,
    dac_swtrgr: DacSwtrgr,
    dac_dhr12r1: DacDhr12r1,
    dac_dhr12l1: DacDhr12l1,
    dac_dhr8r1: DacDhr8r1,
    dac_dhr12r2: DacDhr12r2,
    dac_dhr12l2: DacDhr12l2,
    dac_dhr8r2: DacDhr8r2,
    dac_dhr12rd: DacDhr12rd,
    dac_dhr12ld: DacDhr12ld,
    dac_dhr8rd: DacDhr8rd,
    dac_dor1: DacDor1,
    dac_dor2: DacDor2,
    dac_sr: DacSr,
    dac_ccr: DacCcr,
    dac_mcr: DacMcr,
    dac_shsr1: DacShsr1,
    dac_shsr2: DacShsr2,
    dac_shhr: DacShhr,
    dac_shrr: DacShrr,
    _reserved20: [u8; 0x08],
    dac_str1: DacStr1,
    dac_str2: DacStr2,
    dac_stmodr: DacStmodr,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn dac_cr(&self) -> &DacCr {
        &self.dac_cr
    }
    #[doc = "0x04 - DAC software trigger register"]
    #[inline(always)]
    pub const fn dac_swtrgr(&self) -> &DacSwtrgr {
        &self.dac_swtrgr
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12r1(&self) -> &DacDhr12r1 {
        &self.dac_dhr12r1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12l1(&self) -> &DacDhr12l1 {
        &self.dac_dhr12l1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8r1(&self) -> &DacDhr8r1 {
        &self.dac_dhr8r1
    }
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12r2(&self) -> &DacDhr12r2 {
        &self.dac_dhr12r2
    }
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12l2(&self) -> &DacDhr12l2 {
        &self.dac_dhr12l2
    }
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8r2(&self) -> &DacDhr8r2 {
        &self.dac_dhr8r2
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12rd(&self) -> &DacDhr12rd {
        &self.dac_dhr12rd
    }
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr12ld(&self) -> &DacDhr12ld {
        &self.dac_dhr12ld
    }
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dac_dhr8rd(&self) -> &DacDhr8rd {
        &self.dac_dhr8rd
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn dac_dor1(&self) -> &DacDor1 {
        &self.dac_dor1
    }
    #[doc = "0x30 - DAC channel2 data output register"]
    #[inline(always)]
    pub const fn dac_dor2(&self) -> &DacDor2 {
        &self.dac_dor2
    }
    #[doc = "0x34 - DAC status register"]
    #[inline(always)]
    pub const fn dac_sr(&self) -> &DacSr {
        &self.dac_sr
    }
    #[doc = "0x38 - DAC calibration control register"]
    #[inline(always)]
    pub const fn dac_ccr(&self) -> &DacCcr {
        &self.dac_ccr
    }
    #[doc = "0x3c - DAC mode control register"]
    #[inline(always)]
    pub const fn dac_mcr(&self) -> &DacMcr {
        &self.dac_mcr
    }
    #[doc = "0x40 - DAC Sample and Hold sample time register 1"]
    #[inline(always)]
    pub const fn dac_shsr1(&self) -> &DacShsr1 {
        &self.dac_shsr1
    }
    #[doc = "0x44 - DAC Sample and Hold sample time register 2"]
    #[inline(always)]
    pub const fn dac_shsr2(&self) -> &DacShsr2 {
        &self.dac_shsr2
    }
    #[doc = "0x48 - DAC Sample and Hold hold time register"]
    #[inline(always)]
    pub const fn dac_shhr(&self) -> &DacShhr {
        &self.dac_shhr
    }
    #[doc = "0x4c - DAC Sample and Hold refresh time register"]
    #[inline(always)]
    pub const fn dac_shrr(&self) -> &DacShrr {
        &self.dac_shrr
    }
    #[doc = "0x58 - Sawtooth register"]
    #[inline(always)]
    pub const fn dac_str1(&self) -> &DacStr1 {
        &self.dac_str1
    }
    #[doc = "0x5c - Sawtooth register"]
    #[inline(always)]
    pub const fn dac_str2(&self) -> &DacStr2 {
        &self.dac_str2
    }
    #[doc = "0x60 - Sawtooth Mode register"]
    #[inline(always)]
    pub const fn dac_stmodr(&self) -> &DacStmodr {
        &self.dac_stmodr
    }
}
#[doc = "DAC_CR (rw) register accessor: DAC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_cr`]
module"]
#[doc(alias = "DAC_CR")]
pub type DacCr = crate::Reg<dac_cr::DacCrSpec>;
#[doc = "DAC control register"]
pub mod dac_cr;
#[doc = "DAC_SWTRGR (w) register accessor: DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_swtrgr`]
module"]
#[doc(alias = "DAC_SWTRGR")]
pub type DacSwtrgr = crate::Reg<dac_swtrgr::DacSwtrgrSpec>;
#[doc = "DAC software trigger register"]
pub mod dac_swtrgr;
#[doc = "DAC_DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12r1`]
module"]
#[doc(alias = "DAC_DHR12R1")]
pub type DacDhr12r1 = crate::Reg<dac_dhr12r1::DacDhr12r1Spec>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dac_dhr12r1;
#[doc = "DAC_DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12l1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12l1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12l1`]
module"]
#[doc(alias = "DAC_DHR12L1")]
pub type DacDhr12l1 = crate::Reg<dac_dhr12l1::DacDhr12l1Spec>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dac_dhr12l1;
#[doc = "DAC_DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8r1`]
module"]
#[doc(alias = "DAC_DHR8R1")]
pub type DacDhr8r1 = crate::Reg<dac_dhr8r1::DacDhr8r1Spec>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dac_dhr8r1;
#[doc = "DAC_DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12r2`]
module"]
#[doc(alias = "DAC_DHR12R2")]
pub type DacDhr12r2 = crate::Reg<dac_dhr12r2::DacDhr12r2Spec>;
#[doc = "DAC channel2 12-bit right aligned data holding register"]
pub mod dac_dhr12r2;
#[doc = "DAC_DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12l2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12l2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12l2`]
module"]
#[doc(alias = "DAC_DHR12L2")]
pub type DacDhr12l2 = crate::Reg<dac_dhr12l2::DacDhr12l2Spec>;
#[doc = "DAC channel2 12-bit left aligned data holding register"]
pub mod dac_dhr12l2;
#[doc = "DAC_DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8r2`]
module"]
#[doc(alias = "DAC_DHR8R2")]
pub type DacDhr8r2 = crate::Reg<dac_dhr8r2::DacDhr8r2Spec>;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dac_dhr8r2;
#[doc = "DAC_DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12rd`]
module"]
#[doc(alias = "DAC_DHR12RD")]
pub type DacDhr12rd = crate::Reg<dac_dhr12rd::DacDhr12rdSpec>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dac_dhr12rd;
#[doc = "DAC_DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12ld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12ld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr12ld`]
module"]
#[doc(alias = "DAC_DHR12LD")]
pub type DacDhr12ld = crate::Reg<dac_dhr12ld::DacDhr12ldSpec>;
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dac_dhr12ld;
#[doc = "DAC_DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dhr8rd`]
module"]
#[doc(alias = "DAC_DHR8RD")]
pub type DacDhr8rd = crate::Reg<dac_dhr8rd::DacDhr8rdSpec>;
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dac_dhr8rd;
#[doc = "DAC_DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dor1`]
module"]
#[doc(alias = "DAC_DOR1")]
pub type DacDor1 = crate::Reg<dac_dor1::DacDor1Spec>;
#[doc = "DAC channel1 data output register"]
pub mod dac_dor1;
#[doc = "DAC_DOR2 (r) register accessor: DAC channel2 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dor2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_dor2`]
module"]
#[doc(alias = "DAC_DOR2")]
pub type DacDor2 = crate::Reg<dac_dor2::DacDor2Spec>;
#[doc = "DAC channel2 data output register"]
pub mod dac_dor2;
#[doc = "DAC_SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_sr`]
module"]
#[doc(alias = "DAC_SR")]
pub type DacSr = crate::Reg<dac_sr::DacSrSpec>;
#[doc = "DAC status register"]
pub mod dac_sr;
#[doc = "DAC_CCR (rw) register accessor: DAC calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_ccr`]
module"]
#[doc(alias = "DAC_CCR")]
pub type DacCcr = crate::Reg<dac_ccr::DacCcrSpec>;
#[doc = "DAC calibration control register"]
pub mod dac_ccr;
#[doc = "DAC_MCR (rw) register accessor: DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_mcr`]
module"]
#[doc(alias = "DAC_MCR")]
pub type DacMcr = crate::Reg<dac_mcr::DacMcrSpec>;
#[doc = "DAC mode control register"]
pub mod dac_mcr;
#[doc = "DAC_SHSR1 (rw) register accessor: DAC Sample and Hold sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shsr1`]
module"]
#[doc(alias = "DAC_SHSR1")]
pub type DacShsr1 = crate::Reg<dac_shsr1::DacShsr1Spec>;
#[doc = "DAC Sample and Hold sample time register 1"]
pub mod dac_shsr1;
#[doc = "DAC_SHSR2 (rw) register accessor: DAC Sample and Hold sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shsr2`]
module"]
#[doc(alias = "DAC_SHSR2")]
pub type DacShsr2 = crate::Reg<dac_shsr2::DacShsr2Spec>;
#[doc = "DAC Sample and Hold sample time register 2"]
pub mod dac_shsr2;
#[doc = "DAC_SHHR (rw) register accessor: DAC Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shhr`]
module"]
#[doc(alias = "DAC_SHHR")]
pub type DacShhr = crate::Reg<dac_shhr::DacShhrSpec>;
#[doc = "DAC Sample and Hold hold time register"]
pub mod dac_shhr;
#[doc = "DAC_SHRR (rw) register accessor: DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_shrr`]
module"]
#[doc(alias = "DAC_SHRR")]
pub type DacShrr = crate::Reg<dac_shrr::DacShrrSpec>;
#[doc = "DAC Sample and Hold refresh time register"]
pub mod dac_shrr;
#[doc = "DAC_STR1 (rw) register accessor: Sawtooth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_str1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_str1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_str1`]
module"]
#[doc(alias = "DAC_STR1")]
pub type DacStr1 = crate::Reg<dac_str1::DacStr1Spec>;
#[doc = "Sawtooth register"]
pub mod dac_str1;
#[doc = "DAC_STR2 (rw) register accessor: Sawtooth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_str2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_str2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_str2`]
module"]
#[doc(alias = "DAC_STR2")]
pub type DacStr2 = crate::Reg<dac_str2::DacStr2Spec>;
#[doc = "Sawtooth register"]
pub mod dac_str2;
#[doc = "DAC_STMODR (rw) register accessor: Sawtooth Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_stmodr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_stmodr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_stmodr`]
module"]
#[doc(alias = "DAC_STMODR")]
pub type DacStmodr = crate::Reg<dac_stmodr::DacStmodrSpec>;
#[doc = "Sawtooth Mode register"]
pub mod dac_stmodr;
