#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    x1bufcfg: X1bufcfg,
    x2bufcfg: X2bufcfg,
    ybufcfg: Ybufcfg,
    param: Param,
    cr: Cr,
    sr: Sr,
    wdata: Wdata,
    rdata: Rdata,
}
impl RegisterBlock {
    #[doc = "0x00 - FMAC X1 Buffer Configuration register"]
    #[inline(always)]
    pub const fn x1bufcfg(&self) -> &X1bufcfg {
        &self.x1bufcfg
    }
    #[doc = "0x04 - FMAC X2 Buffer Configuration register"]
    #[inline(always)]
    pub const fn x2bufcfg(&self) -> &X2bufcfg {
        &self.x2bufcfg
    }
    #[doc = "0x08 - FMAC Y Buffer Configuration register"]
    #[inline(always)]
    pub const fn ybufcfg(&self) -> &Ybufcfg {
        &self.ybufcfg
    }
    #[doc = "0x0c - FMAC Parameter register"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x10 - FMAC Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x14 - FMAC Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x18 - FMAC Write Data register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    #[doc = "0x1c - FMAC Read Data register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &Rdata {
        &self.rdata
    }
}
#[doc = "X1BUFCFG (rw) register accessor: FMAC X1 Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`x1bufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x1bufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x1bufcfg`] module"]
#[doc(alias = "X1BUFCFG")]
pub type X1bufcfg = crate::Reg<x1bufcfg::X1bufcfgSpec>;
#[doc = "FMAC X1 Buffer Configuration register"]
pub mod x1bufcfg;
#[doc = "X2BUFCFG (rw) register accessor: FMAC X2 Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`x2bufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x2bufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x2bufcfg`] module"]
#[doc(alias = "X2BUFCFG")]
pub type X2bufcfg = crate::Reg<x2bufcfg::X2bufcfgSpec>;
#[doc = "FMAC X2 Buffer Configuration register"]
pub mod x2bufcfg;
#[doc = "YBUFCFG (rw) register accessor: FMAC Y Buffer Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ybufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ybufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ybufcfg`] module"]
#[doc(alias = "YBUFCFG")]
pub type Ybufcfg = crate::Reg<ybufcfg::YbufcfgSpec>;
#[doc = "FMAC Y Buffer Configuration register"]
pub mod ybufcfg;
#[doc = "PARAM (rw) register accessor: FMAC Parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`] module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "FMAC Parameter register"]
pub mod param;
#[doc = "CR (rw) register accessor: FMAC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "FMAC Control register"]
pub mod cr;
#[doc = "SR (r) register accessor: FMAC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "FMAC Status register"]
pub mod sr;
#[doc = "WDATA (w) register accessor: FMAC Write Data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`] module"]
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WdataSpec>;
#[doc = "FMAC Write Data register"]
pub mod wdata;
#[doc = "RDATA (r) register accessor: FMAC Read Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`] module"]
#[doc(alias = "RDATA")]
pub type Rdata = crate::Reg<rdata::RdataSpec>;
#[doc = "FMAC Read Data register"]
pub mod rdata;
