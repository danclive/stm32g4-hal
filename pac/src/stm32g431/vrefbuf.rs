#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vrefbuf_csr: VrefbufCsr,
    vrefbuf_ccr: VrefbufCcr,
}
impl RegisterBlock {
    #[doc = "0x00 - VREF_BUF Control and Status Register"]
    #[inline(always)]
    pub const fn vrefbuf_csr(&self) -> &VrefbufCsr {
        &self.vrefbuf_csr
    }
    #[doc = "0x04 - VREF_BUF Calibration Control Register"]
    #[inline(always)]
    pub const fn vrefbuf_ccr(&self) -> &VrefbufCcr {
        &self.vrefbuf_ccr
    }
}
#[doc = "VREFBUF_CSR (rw) register accessor: VREF_BUF Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefbuf_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrefbuf_csr`]
module"]
#[doc(alias = "VREFBUF_CSR")]
pub type VrefbufCsr = crate::Reg<vrefbuf_csr::VrefbufCsrSpec>;
#[doc = "VREF_BUF Control and Status Register"]
pub mod vrefbuf_csr;
#[doc = "VREFBUF_CCR (rw) register accessor: VREF_BUF Calibration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefbuf_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrefbuf_ccr`]
module"]
#[doc(alias = "VREFBUF_CCR")]
pub type VrefbufCcr = crate::Reg<vrefbuf_ccr::VrefbufCcrSpec>;
#[doc = "VREF_BUF Calibration Control Register"]
pub mod vrefbuf_ccr;
