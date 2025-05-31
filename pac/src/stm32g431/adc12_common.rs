#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    _reserved1: [u8; 0x04],
    ccr: Ccr,
    cdr: Cdr,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Common status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x08 - ADC common control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x0c - ADC common regular data register for dual and triple modes"]
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
}
#[doc = "CSR (r) register accessor: ADC Common status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: ADC common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "CDR (r) register accessor: ADC common regular data register for dual and triple modes\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`] module"]
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
#[doc = "ADC common regular data register for dual and triple modes"]
pub mod cdr;
