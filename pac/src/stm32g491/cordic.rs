#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    wdata: Wdata,
    rdata: Rdata,
}
impl RegisterBlock {
    #[doc = "0x00 - CORDIC Control Status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - FMAC Write Data register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    #[doc = "0x08 - FMAC Read Data register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &Rdata {
        &self.rdata
    }
}
#[doc = "CSR (rw) register accessor: CORDIC Control Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "CORDIC Control Status register"]
pub mod csr;
#[doc = "WDATA (rw) register accessor: FMAC Write Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`]
module"]
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WdataSpec>;
#[doc = "FMAC Write Data register"]
pub mod wdata;
#[doc = "RDATA (r) register accessor: FMAC Read Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
#[doc(alias = "RDATA")]
pub type Rdata = crate::Reg<rdata::RdataSpec>;
#[doc = "FMAC Read Data register"]
pub mod rdata;
