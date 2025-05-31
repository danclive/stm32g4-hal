#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcc_cr: RccCr,
    rcc_icscr: RccIcscr,
    rcc_cfgr: RccCfgr,
    rcc_pllcfgr: RccPllcfgr,
    _reserved4: [u8; 0x08],
    rcc_cier: RccCier,
    rcc_cifr: RccCifr,
    rcc_cicr: RccCicr,
    _reserved7: [u8; 0x04],
    rcc_ahb1rstr: RccAhb1rstr,
    rcc_ahb2rstr: RccAhb2rstr,
    rcc_ahb3rstr: RccAhb3rstr,
    _reserved10: [u8; 0x04],
    rcc_apb1rstr1: RccApb1rstr1,
    rcc_apb1rstr2: RccApb1rstr2,
    rcc_apb2rstr: RccApb2rstr,
    _reserved13: [u8; 0x04],
    rcc_ahb1enr: RccAhb1enr,
    rcc_ahb2enr: RccAhb2enr,
    rcc_ahb3enr: RccAhb3enr,
    _reserved16: [u8; 0x04],
    rcc_apb1enr1: RccApb1enr1,
    rcc_apb1enr2: RccApb1enr2,
    rcc_apb2enr: RccApb2enr,
    _reserved19: [u8; 0x04],
    rcc_ahb1smenr: RccAhb1smenr,
    rcc_ahb2smenr: RccAhb2smenr,
    rcc_ahb3smenr: RccAhb3smenr,
    _reserved22: [u8; 0x04],
    rcc_apb1smenr1: RccApb1smenr1,
    rcc_apb1smenr2: RccApb1smenr2,
    rcc_apb2smenr: RccApb2smenr,
    _reserved25: [u8; 0x04],
    rcc_ccipr: RccCcipr,
    _reserved26: [u8; 0x04],
    rcc_bdcr: RccBdcr,
    rcc_csr: RccCsr,
    rcc_crrcr: RccCrrcr,
    rcc_ccipr2: RccCcipr2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn rcc_cr(&self) -> &RccCr {
        &self.rcc_cr
    }
    #[doc = "0x04 - Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn rcc_icscr(&self) -> &RccIcscr {
        &self.rcc_icscr
    }
    #[doc = "0x08 - Clock configuration register"]
    #[inline(always)]
    pub const fn rcc_cfgr(&self) -> &RccCfgr {
        &self.rcc_cfgr
    }
    #[doc = "0x0c - PLL configuration register"]
    #[inline(always)]
    pub const fn rcc_pllcfgr(&self) -> &RccPllcfgr {
        &self.rcc_pllcfgr
    }
    #[doc = "0x18 - Clock interrupt enable register"]
    #[inline(always)]
    pub const fn rcc_cier(&self) -> &RccCier {
        &self.rcc_cier
    }
    #[doc = "0x1c - Clock interrupt flag register"]
    #[inline(always)]
    pub const fn rcc_cifr(&self) -> &RccCifr {
        &self.rcc_cifr
    }
    #[doc = "0x20 - Clock interrupt clear register"]
    #[inline(always)]
    pub const fn rcc_cicr(&self) -> &RccCicr {
        &self.rcc_cicr
    }
    #[doc = "0x28 - AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_ahb1rstr(&self) -> &RccAhb1rstr {
        &self.rcc_ahb1rstr
    }
    #[doc = "0x2c - AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_ahb2rstr(&self) -> &RccAhb2rstr {
        &self.rcc_ahb2rstr
    }
    #[doc = "0x30 - AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_ahb3rstr(&self) -> &RccAhb3rstr {
        &self.rcc_ahb3rstr
    }
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn rcc_apb1rstr1(&self) -> &RccApb1rstr1 {
        &self.rcc_apb1rstr1
    }
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn rcc_apb1rstr2(&self) -> &RccApb1rstr2 {
        &self.rcc_apb1rstr2
    }
    #[doc = "0x40 - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn rcc_apb2rstr(&self) -> &RccApb2rstr {
        &self.rcc_apb2rstr
    }
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_ahb1enr(&self) -> &RccAhb1enr {
        &self.rcc_ahb1enr
    }
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_ahb2enr(&self) -> &RccAhb2enr {
        &self.rcc_ahb2enr
    }
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_ahb3enr(&self) -> &RccAhb3enr {
        &self.rcc_ahb3enr
    }
    #[doc = "0x58 - APB1 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn rcc_apb1enr1(&self) -> &RccApb1enr1 {
        &self.rcc_apb1enr1
    }
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn rcc_apb1enr2(&self) -> &RccApb1enr2 {
        &self.rcc_apb1enr2
    }
    #[doc = "0x60 - APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn rcc_apb2enr(&self) -> &RccApb2enr {
        &self.rcc_apb2enr
    }
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_ahb1smenr(&self) -> &RccAhb1smenr {
        &self.rcc_ahb1smenr
    }
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_ahb2smenr(&self) -> &RccAhb2smenr {
        &self.rcc_ahb2smenr
    }
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_ahb3smenr(&self) -> &RccAhb3smenr {
        &self.rcc_ahb3smenr
    }
    #[doc = "0x78 - APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn rcc_apb1smenr1(&self) -> &RccApb1smenr1 {
        &self.rcc_apb1smenr1
    }
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn rcc_apb1smenr2(&self) -> &RccApb1smenr2 {
        &self.rcc_apb1smenr2
    }
    #[doc = "0x80 - APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn rcc_apb2smenr(&self) -> &RccApb2smenr {
        &self.rcc_apb2smenr
    }
    #[doc = "0x88 - Peripherals independent clock configuration register"]
    #[inline(always)]
    pub const fn rcc_ccipr(&self) -> &RccCcipr {
        &self.rcc_ccipr
    }
    #[doc = "0x90 - RTC domain control register"]
    #[inline(always)]
    pub const fn rcc_bdcr(&self) -> &RccBdcr {
        &self.rcc_bdcr
    }
    #[doc = "0x94 - Control/status register"]
    #[inline(always)]
    pub const fn rcc_csr(&self) -> &RccCsr {
        &self.rcc_csr
    }
    #[doc = "0x98 - Clock recovery RC register"]
    #[inline(always)]
    pub const fn rcc_crrcr(&self) -> &RccCrrcr {
        &self.rcc_crrcr
    }
    #[doc = "0x9c - Peripherals independent clock configuration register"]
    #[inline(always)]
    pub const fn rcc_ccipr2(&self) -> &RccCcipr2 {
        &self.rcc_ccipr2
    }
}
#[doc = "RCC_CR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cr`] module"]
#[doc(alias = "RCC_CR")]
pub type RccCr = crate::Reg<rcc_cr::RccCrSpec>;
#[doc = "Clock control register"]
pub mod rcc_cr;
#[doc = "RCC_ICSCR (rw) register accessor: Internal clock sources calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_icscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_icscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_icscr`] module"]
#[doc(alias = "RCC_ICSCR")]
pub type RccIcscr = crate::Reg<rcc_icscr::RccIcscrSpec>;
#[doc = "Internal clock sources calibration register"]
pub mod rcc_icscr;
#[doc = "RCC_CFGR (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cfgr`] module"]
#[doc(alias = "RCC_CFGR")]
pub type RccCfgr = crate::Reg<rcc_cfgr::RccCfgrSpec>;
#[doc = "Clock configuration register"]
pub mod rcc_cfgr;
#[doc = "RCC_PLLCFGR (rw) register accessor: PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pllcfgr`] module"]
#[doc(alias = "RCC_PLLCFGR")]
pub type RccPllcfgr = crate::Reg<rcc_pllcfgr::RccPllcfgrSpec>;
#[doc = "PLL configuration register"]
pub mod rcc_pllcfgr;
#[doc = "RCC_CIER (rw) register accessor: Clock interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cier`] module"]
#[doc(alias = "RCC_CIER")]
pub type RccCier = crate::Reg<rcc_cier::RccCierSpec>;
#[doc = "Clock interrupt enable register"]
pub mod rcc_cier;
#[doc = "RCC_CIFR (r) register accessor: Clock interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cifr`] module"]
#[doc(alias = "RCC_CIFR")]
pub type RccCifr = crate::Reg<rcc_cifr::RccCifrSpec>;
#[doc = "Clock interrupt flag register"]
pub mod rcc_cifr;
#[doc = "RCC_CICR (w) register accessor: Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cicr`] module"]
#[doc(alias = "RCC_CICR")]
pub type RccCicr = crate::Reg<rcc_cicr::RccCicrSpec>;
#[doc = "Clock interrupt clear register"]
pub mod rcc_cicr;
#[doc = "RCC_AHB1RSTR (rw) register accessor: AHB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb1rstr`] module"]
#[doc(alias = "RCC_AHB1RSTR")]
pub type RccAhb1rstr = crate::Reg<rcc_ahb1rstr::RccAhb1rstrSpec>;
#[doc = "AHB1 peripheral reset register"]
pub mod rcc_ahb1rstr;
#[doc = "RCC_AHB2RSTR (rw) register accessor: AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2rstr`] module"]
#[doc(alias = "RCC_AHB2RSTR")]
pub type RccAhb2rstr = crate::Reg<rcc_ahb2rstr::RccAhb2rstrSpec>;
#[doc = "AHB2 peripheral reset register"]
pub mod rcc_ahb2rstr;
#[doc = "RCC_AHB3RSTR (rw) register accessor: AHB3 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3rstr`] module"]
#[doc(alias = "RCC_AHB3RSTR")]
pub type RccAhb3rstr = crate::Reg<rcc_ahb3rstr::RccAhb3rstrSpec>;
#[doc = "AHB3 peripheral reset register"]
pub mod rcc_ahb3rstr;
#[doc = "RCC_APB1RSTR1 (rw) register accessor: APB1 peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1rstr1`] module"]
#[doc(alias = "RCC_APB1RSTR1")]
pub type RccApb1rstr1 = crate::Reg<rcc_apb1rstr1::RccApb1rstr1Spec>;
#[doc = "APB1 peripheral reset register 1"]
pub mod rcc_apb1rstr1;
#[doc = "RCC_APB1RSTR2 (rw) register accessor: APB1 peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1rstr2`] module"]
#[doc(alias = "RCC_APB1RSTR2")]
pub type RccApb1rstr2 = crate::Reg<rcc_apb1rstr2::RccApb1rstr2Spec>;
#[doc = "APB1 peripheral reset register 2"]
pub mod rcc_apb1rstr2;
#[doc = "RCC_APB2RSTR (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2rstr`] module"]
#[doc(alias = "RCC_APB2RSTR")]
pub type RccApb2rstr = crate::Reg<rcc_apb2rstr::RccApb2rstrSpec>;
#[doc = "APB2 peripheral reset register"]
pub mod rcc_apb2rstr;
#[doc = "RCC_AHB1ENR (rw) register accessor: AHB1 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb1enr`] module"]
#[doc(alias = "RCC_AHB1ENR")]
pub type RccAhb1enr = crate::Reg<rcc_ahb1enr::RccAhb1enrSpec>;
#[doc = "AHB1 peripheral clock enable register"]
pub mod rcc_ahb1enr;
#[doc = "RCC_AHB2ENR (rw) register accessor: AHB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2enr`] module"]
#[doc(alias = "RCC_AHB2ENR")]
pub type RccAhb2enr = crate::Reg<rcc_ahb2enr::RccAhb2enrSpec>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod rcc_ahb2enr;
#[doc = "RCC_AHB3ENR (rw) register accessor: AHB3 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3enr`] module"]
#[doc(alias = "RCC_AHB3ENR")]
pub type RccAhb3enr = crate::Reg<rcc_ahb3enr::RccAhb3enrSpec>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod rcc_ahb3enr;
#[doc = "RCC_APB1ENR1 (rw) register accessor: APB1 peripheral clock enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1enr1`] module"]
#[doc(alias = "RCC_APB1ENR1")]
pub type RccApb1enr1 = crate::Reg<rcc_apb1enr1::RccApb1enr1Spec>;
#[doc = "APB1 peripheral clock enable register 1"]
pub mod rcc_apb1enr1;
#[doc = "RCC_APB1ENR2 (rw) register accessor: APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1enr2`] module"]
#[doc(alias = "RCC_APB1ENR2")]
pub type RccApb1enr2 = crate::Reg<rcc_apb1enr2::RccApb1enr2Spec>;
#[doc = "APB1 peripheral clock enable register 2"]
pub mod rcc_apb1enr2;
#[doc = "RCC_APB2ENR (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2enr`] module"]
#[doc(alias = "RCC_APB2ENR")]
pub type RccApb2enr = crate::Reg<rcc_apb2enr::RccApb2enrSpec>;
#[doc = "APB2 peripheral clock enable register"]
pub mod rcc_apb2enr;
#[doc = "RCC_AHB1SMENR (rw) register accessor: AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb1smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb1smenr`] module"]
#[doc(alias = "RCC_AHB1SMENR")]
pub type RccAhb1smenr = crate::Reg<rcc_ahb1smenr::RccAhb1smenrSpec>;
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_ahb1smenr;
#[doc = "RCC_AHB2SMENR (rw) register accessor: AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2smenr`] module"]
#[doc(alias = "RCC_AHB2SMENR")]
pub type RccAhb2smenr = crate::Reg<rcc_ahb2smenr::RccAhb2smenrSpec>;
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_ahb2smenr;
#[doc = "RCC_AHB3SMENR (rw) register accessor: AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb3smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3smenr`] module"]
#[doc(alias = "RCC_AHB3SMENR")]
pub type RccAhb3smenr = crate::Reg<rcc_ahb3smenr::RccAhb3smenrSpec>;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_ahb3smenr;
#[doc = "RCC_APB1SMENR1 (rw) register accessor: APB1 peripheral clocks enable in Sleep and Stop modes register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1smenr1`] module"]
#[doc(alias = "RCC_APB1SMENR1")]
pub type RccApb1smenr1 = crate::Reg<rcc_apb1smenr1::RccApb1smenr1Spec>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
pub mod rcc_apb1smenr1;
#[doc = "RCC_APB1SMENR2 (rw) register accessor: APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1smenr2`] module"]
#[doc(alias = "RCC_APB1SMENR2")]
pub type RccApb1smenr2 = crate::Reg<rcc_apb1smenr2::RccApb1smenr2Spec>;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod rcc_apb1smenr2;
#[doc = "RCC_APB2SMENR (rw) register accessor: APB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb2smenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2smenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2smenr`] module"]
#[doc(alias = "RCC_APB2SMENR")]
pub type RccApb2smenr = crate::Reg<rcc_apb2smenr::RccApb2smenrSpec>;
#[doc = "APB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod rcc_apb2smenr;
#[doc = "RCC_CCIPR (rw) register accessor: Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ccipr`] module"]
#[doc(alias = "RCC_CCIPR")]
pub type RccCcipr = crate::Reg<rcc_ccipr::RccCciprSpec>;
#[doc = "Peripherals independent clock configuration register"]
pub mod rcc_ccipr;
#[doc = "RCC_BDCR (rw) register accessor: RTC domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_bdcr`] module"]
#[doc(alias = "RCC_BDCR")]
pub type RccBdcr = crate::Reg<rcc_bdcr::RccBdcrSpec>;
#[doc = "RTC domain control register"]
pub mod rcc_bdcr;
#[doc = "RCC_CSR (rw) register accessor: Control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_csr`] module"]
#[doc(alias = "RCC_CSR")]
pub type RccCsr = crate::Reg<rcc_csr::RccCsrSpec>;
#[doc = "Control/status register"]
pub mod rcc_csr;
#[doc = "RCC_CRRCR (rw) register accessor: Clock recovery RC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_crrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_crrcr`] module"]
#[doc(alias = "RCC_CRRCR")]
pub type RccCrrcr = crate::Reg<rcc_crrcr::RccCrrcrSpec>;
#[doc = "Clock recovery RC register"]
pub mod rcc_crrcr;
#[doc = "RCC_CCIPR2 (rw) register accessor: Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ccipr2`] module"]
#[doc(alias = "RCC_CCIPR2")]
pub type RccCcipr2 = crate::Reg<rcc_ccipr2::RccCcipr2Spec>;
#[doc = "Peripherals independent clock configuration register"]
pub mod rcc_ccipr2;
