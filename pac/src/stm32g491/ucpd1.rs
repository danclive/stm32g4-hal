#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg1: Cfg1,
    cfg2: Cfg2,
    _reserved2: [u8; 0x04],
    cr: Cr,
    imr: Imr,
    sr: Sr,
    icr: Icr,
    tx_ordset: TxOrdset,
    tx_paysz: TxPaysz,
    txdr: Txdr,
    rx_ordset: RxOrdset,
    rx_paysz: RxPaysz,
    rxdr: Rxdr,
    rx_ordext1: RxOrdext1,
    rx_ordext2: RxOrdext2,
}
impl RegisterBlock {
    #[doc = "0x00 - UCPD configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x04 - UCPD configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x0c - UCPD configuration register 2"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10 - UCPD Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x14 - UCPD Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x18 - UCPD Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x1c - UCPD Tx Ordered Set Type Register"]
    #[inline(always)]
    pub const fn tx_ordset(&self) -> &TxOrdset {
        &self.tx_ordset
    }
    #[doc = "0x20 - UCPD Tx Paysize Register"]
    #[inline(always)]
    pub const fn tx_paysz(&self) -> &TxPaysz {
        &self.tx_paysz
    }
    #[doc = "0x24 - UCPD Tx Data Register"]
    #[inline(always)]
    pub const fn txdr(&self) -> &Txdr {
        &self.txdr
    }
    #[doc = "0x28 - UCPD Rx Ordered Set Register"]
    #[inline(always)]
    pub const fn rx_ordset(&self) -> &RxOrdset {
        &self.rx_ordset
    }
    #[doc = "0x2c - UCPD Rx Paysize Register"]
    #[inline(always)]
    pub const fn rx_paysz(&self) -> &RxPaysz {
        &self.rx_paysz
    }
    #[doc = "0x30 - UCPD Rx Data Register"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &Rxdr {
        &self.rxdr
    }
    #[doc = "0x34 - UCPD Rx Ordered Set Extension Register 1"]
    #[inline(always)]
    pub const fn rx_ordext1(&self) -> &RxOrdext1 {
        &self.rx_ordext1
    }
    #[doc = "0x38 - UCPD Rx Ordered Set Extension Register 2"]
    #[inline(always)]
    pub const fn rx_ordext2(&self) -> &RxOrdext2 {
        &self.rx_ordext2
    }
}
#[doc = "CFG1 (rw) register accessor: UCPD configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "UCPD configuration register 1"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: UCPD configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "UCPD configuration register 2"]
pub mod cfg2;
#[doc = "CR (rw) register accessor: UCPD configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "UCPD configuration register 2"]
pub mod cr;
#[doc = "IMR (rw) register accessor: UCPD Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "UCPD Interrupt Mask Register"]
pub mod imr;
#[doc = "SR (rw) register accessor: UCPD Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "UCPD Status Register"]
pub mod sr;
#[doc = "ICR (rw) register accessor: UCPD Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "UCPD Interrupt Clear Register"]
pub mod icr;
#[doc = "TX_ORDSET (rw) register accessor: UCPD Tx Ordered Set Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ordset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ordset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ordset`]
module"]
#[doc(alias = "TX_ORDSET")]
pub type TxOrdset = crate::Reg<tx_ordset::TxOrdsetSpec>;
#[doc = "UCPD Tx Ordered Set Type Register"]
pub mod tx_ordset;
#[doc = "TX_PAYSZ (rw) register accessor: UCPD Tx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_paysz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_paysz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_paysz`]
module"]
#[doc(alias = "TX_PAYSZ")]
pub type TxPaysz = crate::Reg<tx_paysz::TxPayszSpec>;
#[doc = "UCPD Tx Paysize Register"]
pub mod tx_paysz;
#[doc = "TXDR (rw) register accessor: UCPD Tx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
#[doc(alias = "TXDR")]
pub type Txdr = crate::Reg<txdr::TxdrSpec>;
#[doc = "UCPD Tx Data Register"]
pub mod txdr;
#[doc = "RX_ORDSET (r) register accessor: UCPD Rx Ordered Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordset`]
module"]
#[doc(alias = "RX_ORDSET")]
pub type RxOrdset = crate::Reg<rx_ordset::RxOrdsetSpec>;
#[doc = "UCPD Rx Ordered Set Register"]
pub mod rx_ordset;
#[doc = "RX_PAYSZ (r) register accessor: UCPD Rx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_paysz::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_paysz`]
module"]
#[doc(alias = "RX_PAYSZ")]
pub type RxPaysz = crate::Reg<rx_paysz::RxPayszSpec>;
#[doc = "UCPD Rx Paysize Register"]
pub mod rx_paysz;
#[doc = "RXDR (r) register accessor: UCPD Rx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
#[doc(alias = "RXDR")]
pub type Rxdr = crate::Reg<rxdr::RxdrSpec>;
#[doc = "UCPD Rx Data Register"]
pub mod rxdr;
#[doc = "RX_ORDEXT1 (rw) register accessor: UCPD Rx Ordered Set Extension Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordext1`]
module"]
#[doc(alias = "RX_ORDEXT1")]
pub type RxOrdext1 = crate::Reg<rx_ordext1::RxOrdext1Spec>;
#[doc = "UCPD Rx Ordered Set Extension Register 1"]
pub mod rx_ordext1;
#[doc = "RX_ORDEXT2 (rw) register accessor: UCPD Rx Ordered Set Extension Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordext2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordext2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ordext2`]
module"]
#[doc(alias = "RX_ORDEXT2")]
pub type RxOrdext2 = crate::Reg<rx_ordext2::RxOrdext2Spec>;
#[doc = "UCPD Rx Ordered Set Extension Register 2"]
pub mod rx_ordext2;
