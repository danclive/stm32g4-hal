#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwr_cr1: PwrCr1,
    pwr_cr2: PwrCr2,
    pwr_cr3: PwrCr3,
    pwr_cr4: PwrCr4,
    pwr_sr1: PwrSr1,
    pwr_sr2: PwrSr2,
    pwr_scr: PwrScr,
    _reserved7: [u8; 0x04],
    pwr_pucra: PwrPucra,
    pwr_pdcra: PwrPdcra,
    pwr_pucrb: PwrPucrb,
    pwr_pdcrb: PwrPdcrb,
    pwr_pucrc: PwrPucrc,
    pwr_pdcrc: PwrPdcrc,
    pwr_pucrd: PwrPucrd,
    pwr_pdcrd: PwrPdcrd,
    pwr_pucre: PwrPucre,
    pwr_pdcre: PwrPdcre,
    pwr_pucrf: PwrPucrf,
    pwr_pdcrf: PwrPdcrf,
    pwr_pucrg: PwrPucrg,
    pwr_pdcrg: PwrPdcrg,
    _reserved21: [u8; 0x28],
    pwr_cr5: PwrCr5,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    #[inline(always)]
    pub const fn pwr_cr1(&self) -> &PwrCr1 {
        &self.pwr_cr1
    }
    #[doc = "0x04 - Power control register 2"]
    #[inline(always)]
    pub const fn pwr_cr2(&self) -> &PwrCr2 {
        &self.pwr_cr2
    }
    #[doc = "0x08 - Power control register 3"]
    #[inline(always)]
    pub const fn pwr_cr3(&self) -> &PwrCr3 {
        &self.pwr_cr3
    }
    #[doc = "0x0c - Power control register 4"]
    #[inline(always)]
    pub const fn pwr_cr4(&self) -> &PwrCr4 {
        &self.pwr_cr4
    }
    #[doc = "0x10 - Power status register 1"]
    #[inline(always)]
    pub const fn pwr_sr1(&self) -> &PwrSr1 {
        &self.pwr_sr1
    }
    #[doc = "0x14 - Power status register 2"]
    #[inline(always)]
    pub const fn pwr_sr2(&self) -> &PwrSr2 {
        &self.pwr_sr2
    }
    #[doc = "0x18 - Power status clear register"]
    #[inline(always)]
    pub const fn pwr_scr(&self) -> &PwrScr {
        &self.pwr_scr
    }
    #[doc = "0x20 - Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucra(&self) -> &PwrPucra {
        &self.pwr_pucra
    }
    #[doc = "0x24 - Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcra(&self) -> &PwrPdcra {
        &self.pwr_pdcra
    }
    #[doc = "0x28 - Power Port B pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrb(&self) -> &PwrPucrb {
        &self.pwr_pucrb
    }
    #[doc = "0x2c - Power Port B pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrb(&self) -> &PwrPdcrb {
        &self.pwr_pdcrb
    }
    #[doc = "0x30 - Power Port C pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrc(&self) -> &PwrPucrc {
        &self.pwr_pucrc
    }
    #[doc = "0x34 - Power Port C pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrc(&self) -> &PwrPdcrc {
        &self.pwr_pdcrc
    }
    #[doc = "0x38 - Power Port D pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrd(&self) -> &PwrPucrd {
        &self.pwr_pucrd
    }
    #[doc = "0x3c - Power Port D pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrd(&self) -> &PwrPdcrd {
        &self.pwr_pdcrd
    }
    #[doc = "0x40 - Power Port E pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucre(&self) -> &PwrPucre {
        &self.pwr_pucre
    }
    #[doc = "0x44 - Power Port E pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcre(&self) -> &PwrPdcre {
        &self.pwr_pdcre
    }
    #[doc = "0x48 - Power Port F pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrf(&self) -> &PwrPucrf {
        &self.pwr_pucrf
    }
    #[doc = "0x4c - Power Port F pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrf(&self) -> &PwrPdcrf {
        &self.pwr_pdcrf
    }
    #[doc = "0x50 - Power Port G pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrg(&self) -> &PwrPucrg {
        &self.pwr_pucrg
    }
    #[doc = "0x54 - Power Port G pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrg(&self) -> &PwrPdcrg {
        &self.pwr_pdcrg
    }
    #[doc = "0x80 - Power control register"]
    #[inline(always)]
    pub const fn pwr_cr5(&self) -> &PwrCr5 {
        &self.pwr_cr5
    }
}
#[doc = "PWR_CR1 (rw) register accessor: Power control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr1`] module"]
#[doc(alias = "PWR_CR1")]
pub type PwrCr1 = crate::Reg<pwr_cr1::PwrCr1Spec>;
#[doc = "Power control register 1"]
pub mod pwr_cr1;
#[doc = "PWR_CR2 (rw) register accessor: Power control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr2`] module"]
#[doc(alias = "PWR_CR2")]
pub type PwrCr2 = crate::Reg<pwr_cr2::PwrCr2Spec>;
#[doc = "Power control register 2"]
pub mod pwr_cr2;
#[doc = "PWR_CR3 (rw) register accessor: Power control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr3`] module"]
#[doc(alias = "PWR_CR3")]
pub type PwrCr3 = crate::Reg<pwr_cr3::PwrCr3Spec>;
#[doc = "Power control register 3"]
pub mod pwr_cr3;
#[doc = "PWR_CR4 (rw) register accessor: Power control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr4`] module"]
#[doc(alias = "PWR_CR4")]
pub type PwrCr4 = crate::Reg<pwr_cr4::PwrCr4Spec>;
#[doc = "Power control register 4"]
pub mod pwr_cr4;
#[doc = "PWR_SR1 (r) register accessor: Power status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_sr1`] module"]
#[doc(alias = "PWR_SR1")]
pub type PwrSr1 = crate::Reg<pwr_sr1::PwrSr1Spec>;
#[doc = "Power status register 1"]
pub mod pwr_sr1;
#[doc = "PWR_SR2 (r) register accessor: Power status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_sr2`] module"]
#[doc(alias = "PWR_SR2")]
pub type PwrSr2 = crate::Reg<pwr_sr2::PwrSr2Spec>;
#[doc = "Power status register 2"]
pub mod pwr_sr2;
#[doc = "PWR_SCR (w) register accessor: Power status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_scr`] module"]
#[doc(alias = "PWR_SCR")]
pub type PwrScr = crate::Reg<pwr_scr::PwrScrSpec>;
#[doc = "Power status clear register"]
pub mod pwr_scr;
#[doc = "PWR_PUCRA (rw) register accessor: Power Port A pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucra`] module"]
#[doc(alias = "PWR_PUCRA")]
pub type PwrPucra = crate::Reg<pwr_pucra::PwrPucraSpec>;
#[doc = "Power Port A pull-up control register"]
pub mod pwr_pucra;
#[doc = "PWR_PDCRA (rw) register accessor: Power Port A pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcra`] module"]
#[doc(alias = "PWR_PDCRA")]
pub type PwrPdcra = crate::Reg<pwr_pdcra::PwrPdcraSpec>;
#[doc = "Power Port A pull-down control register"]
pub mod pwr_pdcra;
#[doc = "PWR_PUCRB (rw) register accessor: Power Port B pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrb`] module"]
#[doc(alias = "PWR_PUCRB")]
pub type PwrPucrb = crate::Reg<pwr_pucrb::PwrPucrbSpec>;
#[doc = "Power Port B pull-up control register"]
pub mod pwr_pucrb;
#[doc = "PWR_PDCRB (rw) register accessor: Power Port B pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrb`] module"]
#[doc(alias = "PWR_PDCRB")]
pub type PwrPdcrb = crate::Reg<pwr_pdcrb::PwrPdcrbSpec>;
#[doc = "Power Port B pull-down control register"]
pub mod pwr_pdcrb;
#[doc = "PWR_PUCRC (rw) register accessor: Power Port C pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrc`] module"]
#[doc(alias = "PWR_PUCRC")]
pub type PwrPucrc = crate::Reg<pwr_pucrc::PwrPucrcSpec>;
#[doc = "Power Port C pull-up control register"]
pub mod pwr_pucrc;
#[doc = "PWR_PDCRC (rw) register accessor: Power Port C pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrc`] module"]
#[doc(alias = "PWR_PDCRC")]
pub type PwrPdcrc = crate::Reg<pwr_pdcrc::PwrPdcrcSpec>;
#[doc = "Power Port C pull-down control register"]
pub mod pwr_pdcrc;
#[doc = "PWR_PUCRD (rw) register accessor: Power Port D pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrd`] module"]
#[doc(alias = "PWR_PUCRD")]
pub type PwrPucrd = crate::Reg<pwr_pucrd::PwrPucrdSpec>;
#[doc = "Power Port D pull-up control register"]
pub mod pwr_pucrd;
#[doc = "PWR_PDCRD (rw) register accessor: Power Port D pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrd`] module"]
#[doc(alias = "PWR_PDCRD")]
pub type PwrPdcrd = crate::Reg<pwr_pdcrd::PwrPdcrdSpec>;
#[doc = "Power Port D pull-down control register"]
pub mod pwr_pdcrd;
#[doc = "PWR_PUCRE (rw) register accessor: Power Port E pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucre`] module"]
#[doc(alias = "PWR_PUCRE")]
pub type PwrPucre = crate::Reg<pwr_pucre::PwrPucreSpec>;
#[doc = "Power Port E pull-up control register"]
pub mod pwr_pucre;
#[doc = "PWR_PDCRE (rw) register accessor: Power Port E pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcre`] module"]
#[doc(alias = "PWR_PDCRE")]
pub type PwrPdcre = crate::Reg<pwr_pdcre::PwrPdcreSpec>;
#[doc = "Power Port E pull-down control register"]
pub mod pwr_pdcre;
#[doc = "PWR_PUCRF (rw) register accessor: Power Port F pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrf`] module"]
#[doc(alias = "PWR_PUCRF")]
pub type PwrPucrf = crate::Reg<pwr_pucrf::PwrPucrfSpec>;
#[doc = "Power Port F pull-up control register"]
pub mod pwr_pucrf;
#[doc = "PWR_PDCRF (rw) register accessor: Power Port F pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrf`] module"]
#[doc(alias = "PWR_PDCRF")]
pub type PwrPdcrf = crate::Reg<pwr_pdcrf::PwrPdcrfSpec>;
#[doc = "Power Port F pull-down control register"]
pub mod pwr_pdcrf;
#[doc = "PWR_PUCRG (rw) register accessor: Power Port G pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pucrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrg`] module"]
#[doc(alias = "PWR_PUCRG")]
pub type PwrPucrg = crate::Reg<pwr_pucrg::PwrPucrgSpec>;
#[doc = "Power Port G pull-up control register"]
pub mod pwr_pucrg;
#[doc = "PWR_PDCRG (rw) register accessor: Power Port G pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrg`] module"]
#[doc(alias = "PWR_PDCRG")]
pub type PwrPdcrg = crate::Reg<pwr_pdcrg::PwrPdcrgSpec>;
#[doc = "Power Port G pull-down control register"]
pub mod pwr_pdcrg;
#[doc = "PWR_CR5 (rw) register accessor: Power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr5`] module"]
#[doc(alias = "PWR_CR5")]
pub type PwrCr5 = crate::Reg<pwr_cr5::PwrCr5Spec>;
#[doc = "Power control register"]
pub mod pwr_cr5;
