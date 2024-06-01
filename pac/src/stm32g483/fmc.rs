#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcr1: Bcr1,
    btr1: Btr1,
    bcr2: Bcr2,
    btr2: Btr2,
    bcr3: Bcr3,
    btr3: Btr3,
    bcr4: Bcr4,
    btr4: Btr4,
    pcscntr: Pcscntr,
    _reserved9: [u8; 0x5c],
    pcr: Pcr,
    sr: Sr,
    pmem: Pmem,
    patt: Patt,
    _reserved13: [u8; 0x04],
    eccr: Eccr,
    _reserved14: [u8; 0x6c],
    bwtr1: Bwtr1,
    _reserved15: [u8; 0x04],
    bwtr2: Bwtr2,
    _reserved16: [u8; 0x04],
    bwtr3: Bwtr3,
    _reserved17: [u8; 0x04],
    bwtr4: Bwtr4,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bcr1(&self) -> &Bcr1 {
        &self.bcr1
    }
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    #[inline(always)]
    pub const fn btr1(&self) -> &Btr1 {
        &self.btr1
    }
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    #[inline(always)]
    pub const fn bcr2(&self) -> &Bcr2 {
        &self.bcr2
    }
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    #[inline(always)]
    pub const fn btr2(&self) -> &Btr2 {
        &self.btr2
    }
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    #[inline(always)]
    pub const fn bcr3(&self) -> &Bcr3 {
        &self.bcr3
    }
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    #[inline(always)]
    pub const fn btr3(&self) -> &Btr3 {
        &self.btr3
    }
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    #[inline(always)]
    pub const fn bcr4(&self) -> &Bcr4 {
        &self.bcr4
    }
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    #[inline(always)]
    pub const fn btr4(&self) -> &Btr4 {
        &self.btr4
    }
    #[doc = "0x20 - PSRAM chip select counter register"]
    #[inline(always)]
    pub const fn pcscntr(&self) -> &Pcscntr {
        &self.pcscntr
    }
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
    }
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x88 - Common memory space timing register 3"]
    #[inline(always)]
    pub const fn pmem(&self) -> &Pmem {
        &self.pmem
    }
    #[doc = "0x8c - Attribute memory space timing register 3"]
    #[inline(always)]
    pub const fn patt(&self) -> &Patt {
        &self.patt
    }
    #[doc = "0x94 - ECC result register 3"]
    #[inline(always)]
    pub const fn eccr(&self) -> &Eccr {
        &self.eccr
    }
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    #[inline(always)]
    pub const fn bwtr1(&self) -> &Bwtr1 {
        &self.bwtr1
    }
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    #[inline(always)]
    pub const fn bwtr2(&self) -> &Bwtr2 {
        &self.bwtr2
    }
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    #[inline(always)]
    pub const fn bwtr3(&self) -> &Bwtr3 {
        &self.bwtr3
    }
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    #[inline(always)]
    pub const fn bwtr4(&self) -> &Bwtr4 {
        &self.bwtr4
    }
}
#[doc = "BCR1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr1`]
module"]
#[doc(alias = "BCR1")]
pub type Bcr1 = crate::Reg<bcr1::Bcr1Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR1 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr1`]
module"]
#[doc(alias = "BTR1")]
pub type Btr1 = crate::Reg<btr1::Btr1Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "BCR2 (rw) register accessor: SRAM/NOR-Flash chip-select control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr2`]
module"]
#[doc(alias = "BCR2")]
pub type Bcr2 = crate::Reg<bcr2::Bcr2Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "BTR2 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr2`]
module"]
#[doc(alias = "BTR2")]
pub type Btr2 = crate::Reg<btr2::Btr2Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "BCR3 (rw) register accessor: SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr3`]
module"]
#[doc(alias = "BCR3")]
pub type Bcr3 = crate::Reg<bcr3::Bcr3Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "BTR3 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr3`]
module"]
#[doc(alias = "BTR3")]
pub type Btr3 = crate::Reg<btr3::Btr3Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "BCR4 (rw) register accessor: SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr4`]
module"]
#[doc(alias = "BCR4")]
pub type Bcr4 = crate::Reg<bcr4::Bcr4Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "BTR4 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr4`]
module"]
#[doc(alias = "BTR4")]
pub type Btr4 = crate::Reg<btr4::Btr4Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PCSCNTR (rw) register accessor: PSRAM chip select counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcscntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcscntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcscntr`]
module"]
#[doc(alias = "PCSCNTR")]
pub type Pcscntr = crate::Reg<pcscntr::PcscntrSpec>;
#[doc = "PSRAM chip select counter register"]
pub mod pcscntr;
#[doc = "PCR (rw) register accessor: PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr;
#[doc = "SR (rw) register accessor: FIFO status and interrupt register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "FIFO status and interrupt register 3"]
pub mod sr;
#[doc = "PMEM (rw) register accessor: Common memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmem`]
module"]
#[doc(alias = "PMEM")]
pub type Pmem = crate::Reg<pmem::PmemSpec>;
#[doc = "Common memory space timing register 3"]
pub mod pmem;
#[doc = "PATT (rw) register accessor: Attribute memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt`]
module"]
#[doc(alias = "PATT")]
pub type Patt = crate::Reg<patt::PattSpec>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt;
#[doc = "ECCR (r) register accessor: ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`]
module"]
#[doc(alias = "ECCR")]
pub type Eccr = crate::Reg<eccr::EccrSpec>;
#[doc = "ECC result register 3"]
pub mod eccr;
#[doc = "BWTR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr1`]
module"]
#[doc(alias = "BWTR1")]
pub type Bwtr1 = crate::Reg<bwtr1::Bwtr1Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "BWTR2 (rw) register accessor: SRAM/NOR-Flash write timing registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr2`]
module"]
#[doc(alias = "BWTR2")]
pub type Bwtr2 = crate::Reg<bwtr2::Bwtr2Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "BWTR3 (rw) register accessor: SRAM/NOR-Flash write timing registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr3`]
module"]
#[doc(alias = "BWTR3")]
pub type Bwtr3 = crate::Reg<bwtr3::Bwtr3Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "BWTR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr4`]
module"]
#[doc(alias = "BWTR4")]
pub type Bwtr4 = crate::Reg<bwtr4::Bwtr4Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
