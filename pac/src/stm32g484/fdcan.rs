#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fdcan_crel: FdcanCrel,
    fdcan_endn: FdcanEndn,
    _reserved2: [u8; 0x04],
    fdcan_dbtp: FdcanDbtp,
    fdcan_test: FdcanTest,
    fdcan_rwd: FdcanRwd,
    fdcan_cccr: FdcanCccr,
    fdcan_nbtp: FdcanNbtp,
    fdcan_tscc: FdcanTscc,
    fdcan_tscv: FdcanTscv,
    fdcan_tocc: FdcanTocc,
    fdcan_tocv: FdcanTocv,
    _reserved11: [u8; 0x10],
    fdcan_ecr: FdcanEcr,
    fdcan_psr: FdcanPsr,
    fdcan_tdcr: FdcanTdcr,
    _reserved14: [u8; 0x04],
    fdcan_ir: FdcanIr,
    fdcan_ie: FdcanIe,
    fdcan_ils: FdcanIls,
    fdcan_ile: FdcanIle,
    _reserved18: [u8; 0x20],
    fdcan_rxgfc: FdcanRxgfc,
    fdcan_xidam: FdcanXidam,
    fdcan_hpms: FdcanHpms,
    _reserved21: [u8; 0x04],
    fdcan_rxf0s: FdcanRxf0s,
    fdcan_rxf0a: FdcanRxf0a,
    fdcan_rxf1s: FdcanRxf1s,
    fdcan_rxf1a: FdcanRxf1a,
    _reserved25: [u8; 0x20],
    fdcan_txbc: FdcanTxbc,
    fdcan_txfqs: FdcanTxfqs,
    fdcan_txbrp: FdcanTxbrp,
    fdcan_txbar: FdcanTxbar,
    fdcan_txbcr: FdcanTxbcr,
    fdcan_txbto: FdcanTxbto,
    fdcan_txbcf: FdcanTxbcf,
    fdcan_txbtie: FdcanTxbtie,
    fdcan_txbcie: FdcanTxbcie,
    fdcan_txefs: FdcanTxefs,
    fdcan_txefa: FdcanTxefa,
    _reserved36: [u8; 0x14],
    fdcan_ckdiv: FdcanCkdiv,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN core release register"]
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FdcanCrel {
        &self.fdcan_crel
    }
    #[doc = "0x04 - FDCAN endian register"]
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FdcanEndn {
        &self.fdcan_endn
    }
    #[doc = "0x0c - FDCAN data bit timing and prescaler register"]
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FdcanDbtp {
        &self.fdcan_dbtp
    }
    #[doc = "0x10 - FDCAN test register"]
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FdcanTest {
        &self.fdcan_test
    }
    #[doc = "0x14 - FDCAN RAM watchdog register"]
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FdcanRwd {
        &self.fdcan_rwd
    }
    #[doc = "0x18 - FDCAN CC control register"]
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FdcanCccr {
        &self.fdcan_cccr
    }
    #[doc = "0x1c - FDCAN nominal bit timing and prescaler register"]
    #[inline(always)]
    pub const fn fdcan_nbtp(&self) -> &FdcanNbtp {
        &self.fdcan_nbtp
    }
    #[doc = "0x20 - FDCAN timestamp counter configuration register"]
    #[inline(always)]
    pub const fn fdcan_tscc(&self) -> &FdcanTscc {
        &self.fdcan_tscc
    }
    #[doc = "0x24 - FDCAN timestamp counter value register"]
    #[inline(always)]
    pub const fn fdcan_tscv(&self) -> &FdcanTscv {
        &self.fdcan_tscv
    }
    #[doc = "0x28 - FDCAN timeout counter configuration register"]
    #[inline(always)]
    pub const fn fdcan_tocc(&self) -> &FdcanTocc {
        &self.fdcan_tocc
    }
    #[doc = "0x2c - FDCAN timeout counter value register"]
    #[inline(always)]
    pub const fn fdcan_tocv(&self) -> &FdcanTocv {
        &self.fdcan_tocv
    }
    #[doc = "0x40 - FDCAN error counter register"]
    #[inline(always)]
    pub const fn fdcan_ecr(&self) -> &FdcanEcr {
        &self.fdcan_ecr
    }
    #[doc = "0x44 - FDCAN protocol status register"]
    #[inline(always)]
    pub const fn fdcan_psr(&self) -> &FdcanPsr {
        &self.fdcan_psr
    }
    #[doc = "0x48 - FDCAN transmitter delay compensation register"]
    #[inline(always)]
    pub const fn fdcan_tdcr(&self) -> &FdcanTdcr {
        &self.fdcan_tdcr
    }
    #[doc = "0x50 - FDCAN interrupt register"]
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FdcanIr {
        &self.fdcan_ir
    }
    #[doc = "0x54 - FDCAN interrupt enable register"]
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FdcanIe {
        &self.fdcan_ie
    }
    #[doc = "0x58 - FDCAN interrupt line select register"]
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FdcanIls {
        &self.fdcan_ils
    }
    #[doc = "0x5c - FDCAN interrupt line enable register"]
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FdcanIle {
        &self.fdcan_ile
    }
    #[doc = "0x80 - FDCAN global filter configuration register"]
    #[inline(always)]
    pub const fn fdcan_rxgfc(&self) -> &FdcanRxgfc {
        &self.fdcan_rxgfc
    }
    #[doc = "0x84 - FDCAN extended ID and mask register"]
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FdcanXidam {
        &self.fdcan_xidam
    }
    #[doc = "0x88 - FDCAN high-priority message status register"]
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FdcanHpms {
        &self.fdcan_hpms
    }
    #[doc = "0x90 - FDCAN Rx FIFO 0 status register"]
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FdcanRxf0s {
        &self.fdcan_rxf0s
    }
    #[doc = "0x94 - CAN Rx FIFO 0 acknowledge register"]
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FdcanRxf0a {
        &self.fdcan_rxf0a
    }
    #[doc = "0x98 - FDCAN Rx FIFO 1 status register"]
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FdcanRxf1s {
        &self.fdcan_rxf1s
    }
    #[doc = "0x9c - FDCAN Rx FIFO 1 acknowledge register"]
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FdcanRxf1a {
        &self.fdcan_rxf1a
    }
    #[doc = "0xc0 - FDCAN Tx buffer configuration register"]
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FdcanTxbc {
        &self.fdcan_txbc
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/queue status register"]
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FdcanTxfqs {
        &self.fdcan_txfqs
    }
    #[doc = "0xc8 - FDCAN Tx buffer request pending register"]
    #[inline(always)]
    pub const fn fdcan_txbrp(&self) -> &FdcanTxbrp {
        &self.fdcan_txbrp
    }
    #[doc = "0xcc - FDCAN Tx buffer add request register"]
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FdcanTxbar {
        &self.fdcan_txbar
    }
    #[doc = "0xd0 - FDCAN Tx buffer cancellation request register"]
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FdcanTxbcr {
        &self.fdcan_txbcr
    }
    #[doc = "0xd4 - FDCAN Tx buffer transmission occurred register"]
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FdcanTxbto {
        &self.fdcan_txbto
    }
    #[doc = "0xd8 - FDCAN Tx buffer cancellation finished register"]
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FdcanTxbcf {
        &self.fdcan_txbcf
    }
    #[doc = "0xdc - FDCAN Tx buffer transmission interrupt enable register"]
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FdcanTxbtie {
        &self.fdcan_txbtie
    }
    #[doc = "0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register"]
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FdcanTxbcie {
        &self.fdcan_txbcie
    }
    #[doc = "0xe4 - FDCAN Tx event FIFO status register"]
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FdcanTxefs {
        &self.fdcan_txefs
    }
    #[doc = "0xe8 - FDCAN Tx event FIFO acknowledge register"]
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FdcanTxefa {
        &self.fdcan_txefa
    }
    #[doc = "0x100 - FDCAN CFG clock divider register"]
    #[inline(always)]
    pub const fn fdcan_ckdiv(&self) -> &FdcanCkdiv {
        &self.fdcan_ckdiv
    }
}
#[doc = "FDCAN_CREL (r) register accessor: FDCAN core release register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_crel::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_crel`]
module"]
#[doc(alias = "FDCAN_CREL")]
pub type FdcanCrel = crate::Reg<fdcan_crel::FdcanCrelSpec>;
#[doc = "FDCAN core release register"]
pub mod fdcan_crel;
#[doc = "FDCAN_ENDN (r) register accessor: FDCAN endian register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_endn::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_endn`]
module"]
#[doc(alias = "FDCAN_ENDN")]
pub type FdcanEndn = crate::Reg<fdcan_endn::FdcanEndnSpec>;
#[doc = "FDCAN endian register"]
pub mod fdcan_endn;
#[doc = "FDCAN_DBTP (rw) register accessor: FDCAN data bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_dbtp`]
module"]
#[doc(alias = "FDCAN_DBTP")]
pub type FdcanDbtp = crate::Reg<fdcan_dbtp::FdcanDbtpSpec>;
#[doc = "FDCAN data bit timing and prescaler register"]
pub mod fdcan_dbtp;
#[doc = "FDCAN_TEST (rw) register accessor: FDCAN test register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_test`]
module"]
#[doc(alias = "FDCAN_TEST")]
pub type FdcanTest = crate::Reg<fdcan_test::FdcanTestSpec>;
#[doc = "FDCAN test register"]
pub mod fdcan_test;
#[doc = "FDCAN_RWD (rw) register accessor: FDCAN RAM watchdog register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rwd`]
module"]
#[doc(alias = "FDCAN_RWD")]
pub type FdcanRwd = crate::Reg<fdcan_rwd::FdcanRwdSpec>;
#[doc = "FDCAN RAM watchdog register"]
pub mod fdcan_rwd;
#[doc = "FDCAN_CCCR (rw) register accessor: FDCAN CC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_cccr`]
module"]
#[doc(alias = "FDCAN_CCCR")]
pub type FdcanCccr = crate::Reg<fdcan_cccr::FdcanCccrSpec>;
#[doc = "FDCAN CC control register"]
pub mod fdcan_cccr;
#[doc = "FDCAN_NBTP (rw) register accessor: FDCAN nominal bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_nbtp`]
module"]
#[doc(alias = "FDCAN_NBTP")]
pub type FdcanNbtp = crate::Reg<fdcan_nbtp::FdcanNbtpSpec>;
#[doc = "FDCAN nominal bit timing and prescaler register"]
pub mod fdcan_nbtp;
#[doc = "FDCAN_TSCC (rw) register accessor: FDCAN timestamp counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscc`]
module"]
#[doc(alias = "FDCAN_TSCC")]
pub type FdcanTscc = crate::Reg<fdcan_tscc::FdcanTsccSpec>;
#[doc = "FDCAN timestamp counter configuration register"]
pub mod fdcan_tscc;
#[doc = "FDCAN_TSCV (rw) register accessor: FDCAN timestamp counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscv`]
module"]
#[doc(alias = "FDCAN_TSCV")]
pub type FdcanTscv = crate::Reg<fdcan_tscv::FdcanTscvSpec>;
#[doc = "FDCAN timestamp counter value register"]
pub mod fdcan_tscv;
#[doc = "FDCAN_TOCC (rw) register accessor: FDCAN timeout counter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocc`]
module"]
#[doc(alias = "FDCAN_TOCC")]
pub type FdcanTocc = crate::Reg<fdcan_tocc::FdcanToccSpec>;
#[doc = "FDCAN timeout counter configuration register"]
pub mod fdcan_tocc;
#[doc = "FDCAN_TOCV (rw) register accessor: FDCAN timeout counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocv`]
module"]
#[doc(alias = "FDCAN_TOCV")]
pub type FdcanTocv = crate::Reg<fdcan_tocv::FdcanTocvSpec>;
#[doc = "FDCAN timeout counter value register"]
pub mod fdcan_tocv;
#[doc = "FDCAN_ECR (rw) register accessor: FDCAN error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ecr`]
module"]
#[doc(alias = "FDCAN_ECR")]
pub type FdcanEcr = crate::Reg<fdcan_ecr::FdcanEcrSpec>;
#[doc = "FDCAN error counter register"]
pub mod fdcan_ecr;
#[doc = "FDCAN_PSR (rw) register accessor: FDCAN protocol status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_psr`]
module"]
#[doc(alias = "FDCAN_PSR")]
pub type FdcanPsr = crate::Reg<fdcan_psr::FdcanPsrSpec>;
#[doc = "FDCAN protocol status register"]
pub mod fdcan_psr;
#[doc = "FDCAN_TDCR (rw) register accessor: FDCAN transmitter delay compensation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tdcr`]
module"]
#[doc(alias = "FDCAN_TDCR")]
pub type FdcanTdcr = crate::Reg<fdcan_tdcr::FdcanTdcrSpec>;
#[doc = "FDCAN transmitter delay compensation register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN_IR (rw) register accessor: FDCAN interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ir`]
module"]
#[doc(alias = "FDCAN_IR")]
pub type FdcanIr = crate::Reg<fdcan_ir::FdcanIrSpec>;
#[doc = "FDCAN interrupt register"]
pub mod fdcan_ir;
#[doc = "FDCAN_IE (rw) register accessor: FDCAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ie`]
module"]
#[doc(alias = "FDCAN_IE")]
pub type FdcanIe = crate::Reg<fdcan_ie::FdcanIeSpec>;
#[doc = "FDCAN interrupt enable register"]
pub mod fdcan_ie;
#[doc = "FDCAN_ILS (rw) register accessor: FDCAN interrupt line select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ils`]
module"]
#[doc(alias = "FDCAN_ILS")]
pub type FdcanIls = crate::Reg<fdcan_ils::FdcanIlsSpec>;
#[doc = "FDCAN interrupt line select register"]
pub mod fdcan_ils;
#[doc = "FDCAN_ILE (rw) register accessor: FDCAN interrupt line enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ile`]
module"]
#[doc(alias = "FDCAN_ILE")]
pub type FdcanIle = crate::Reg<fdcan_ile::FdcanIleSpec>;
#[doc = "FDCAN interrupt line enable register"]
pub mod fdcan_ile;
#[doc = "FDCAN_RXGFC (rw) register accessor: FDCAN global filter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxgfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxgfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxgfc`]
module"]
#[doc(alias = "FDCAN_RXGFC")]
pub type FdcanRxgfc = crate::Reg<fdcan_rxgfc::FdcanRxgfcSpec>;
#[doc = "FDCAN global filter configuration register"]
pub mod fdcan_rxgfc;
#[doc = "FDCAN_XIDAM (rw) register accessor: FDCAN extended ID and mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidam`]
module"]
#[doc(alias = "FDCAN_XIDAM")]
pub type FdcanXidam = crate::Reg<fdcan_xidam::FdcanXidamSpec>;
#[doc = "FDCAN extended ID and mask register"]
pub mod fdcan_xidam;
#[doc = "FDCAN_HPMS (r) register accessor: FDCAN high-priority message status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_hpms::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_hpms`]
module"]
#[doc(alias = "FDCAN_HPMS")]
pub type FdcanHpms = crate::Reg<fdcan_hpms::FdcanHpmsSpec>;
#[doc = "FDCAN high-priority message status register"]
pub mod fdcan_hpms;
#[doc = "FDCAN_RXF0S (r) register accessor: FDCAN Rx FIFO 0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0s`]
module"]
#[doc(alias = "FDCAN_RXF0S")]
pub type FdcanRxf0s = crate::Reg<fdcan_rxf0s::FdcanRxf0sSpec>;
#[doc = "FDCAN Rx FIFO 0 status register"]
pub mod fdcan_rxf0s;
#[doc = "FDCAN_RXF0A (rw) register accessor: CAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0a`]
module"]
#[doc(alias = "FDCAN_RXF0A")]
pub type FdcanRxf0a = crate::Reg<fdcan_rxf0a::FdcanRxf0aSpec>;
#[doc = "CAN Rx FIFO 0 acknowledge register"]
pub mod fdcan_rxf0a;
#[doc = "FDCAN_RXF1S (r) register accessor: FDCAN Rx FIFO 1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1s::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1s`]
module"]
#[doc(alias = "FDCAN_RXF1S")]
pub type FdcanRxf1s = crate::Reg<fdcan_rxf1s::FdcanRxf1sSpec>;
#[doc = "FDCAN Rx FIFO 1 status register"]
pub mod fdcan_rxf1s;
#[doc = "FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1a`]
module"]
#[doc(alias = "FDCAN_RXF1A")]
pub type FdcanRxf1a = crate::Reg<fdcan_rxf1a::FdcanRxf1aSpec>;
#[doc = "FDCAN Rx FIFO 1 acknowledge register"]
pub mod fdcan_rxf1a;
#[doc = "FDCAN_TXBC (rw) register accessor: FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbc`]
module"]
#[doc(alias = "FDCAN_TXBC")]
pub type FdcanTxbc = crate::Reg<fdcan_txbc::FdcanTxbcSpec>;
#[doc = "FDCAN Tx buffer configuration register"]
pub mod fdcan_txbc;
#[doc = "FDCAN_TXFQS (r) register accessor: FDCAN Tx FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txfqs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txfqs`]
module"]
#[doc(alias = "FDCAN_TXFQS")]
pub type FdcanTxfqs = crate::Reg<fdcan_txfqs::FdcanTxfqsSpec>;
#[doc = "FDCAN Tx FIFO/queue status register"]
pub mod fdcan_txfqs;
#[doc = "FDCAN_TXBRP (r) register accessor: FDCAN Tx buffer request pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbrp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbrp`]
module"]
#[doc(alias = "FDCAN_TXBRP")]
pub type FdcanTxbrp = crate::Reg<fdcan_txbrp::FdcanTxbrpSpec>;
#[doc = "FDCAN Tx buffer request pending register"]
pub mod fdcan_txbrp;
#[doc = "FDCAN_TXBAR (rw) register accessor: FDCAN Tx buffer add request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbar`]
module"]
#[doc(alias = "FDCAN_TXBAR")]
pub type FdcanTxbar = crate::Reg<fdcan_txbar::FdcanTxbarSpec>;
#[doc = "FDCAN Tx buffer add request register"]
pub mod fdcan_txbar;
#[doc = "FDCAN_TXBCR (rw) register accessor: FDCAN Tx buffer cancellation request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcr`]
module"]
#[doc(alias = "FDCAN_TXBCR")]
pub type FdcanTxbcr = crate::Reg<fdcan_txbcr::FdcanTxbcrSpec>;
#[doc = "FDCAN Tx buffer cancellation request register"]
pub mod fdcan_txbcr;
#[doc = "FDCAN_TXBTO (r) register accessor: FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbto`]
module"]
#[doc(alias = "FDCAN_TXBTO")]
pub type FdcanTxbto = crate::Reg<fdcan_txbto::FdcanTxbtoSpec>;
#[doc = "FDCAN Tx buffer transmission occurred register"]
pub mod fdcan_txbto;
#[doc = "FDCAN_TXBCF (r) register accessor: FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcf`]
module"]
#[doc(alias = "FDCAN_TXBCF")]
pub type FdcanTxbcf = crate::Reg<fdcan_txbcf::FdcanTxbcfSpec>;
#[doc = "FDCAN Tx buffer cancellation finished register"]
pub mod fdcan_txbcf;
#[doc = "FDCAN_TXBTIE (rw) register accessor: FDCAN Tx buffer transmission interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbtie`]
module"]
#[doc(alias = "FDCAN_TXBTIE")]
pub type FdcanTxbtie = crate::Reg<fdcan_txbtie::FdcanTxbtieSpec>;
#[doc = "FDCAN Tx buffer transmission interrupt enable register"]
pub mod fdcan_txbtie;
#[doc = "FDCAN_TXBCIE (rw) register accessor: FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcie`]
module"]
#[doc(alias = "FDCAN_TXBCIE")]
pub type FdcanTxbcie = crate::Reg<fdcan_txbcie::FdcanTxbcieSpec>;
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register"]
pub mod fdcan_txbcie;
#[doc = "FDCAN_TXEFS (r) register accessor: FDCAN Tx event FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefs`]
module"]
#[doc(alias = "FDCAN_TXEFS")]
pub type FdcanTxefs = crate::Reg<fdcan_txefs::FdcanTxefsSpec>;
#[doc = "FDCAN Tx event FIFO status register"]
pub mod fdcan_txefs;
#[doc = "FDCAN_TXEFA (rw) register accessor: FDCAN Tx event FIFO acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefa`]
module"]
#[doc(alias = "FDCAN_TXEFA")]
pub type FdcanTxefa = crate::Reg<fdcan_txefa::FdcanTxefaSpec>;
#[doc = "FDCAN Tx event FIFO acknowledge register"]
pub mod fdcan_txefa;
#[doc = "FDCAN_CKDIV (rw) register accessor: FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ckdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ckdiv`]
module"]
#[doc(alias = "FDCAN_CKDIV")]
pub type FdcanCkdiv = crate::Reg<fdcan_ckdiv::FdcanCkdivSpec>;
#[doc = "FDCAN CFG clock divider register"]
pub mod fdcan_ckdiv;
