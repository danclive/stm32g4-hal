#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp_c1csr: CompC1csr,
    comp_c2csr: CompC2csr,
    comp_c3csr: CompC3csr,
    comp_c4csr: CompC4csr,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c1csr(&self) -> &CompC1csr {
        &self.comp_c1csr
    }
    #[doc = "0x04 - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c2csr(&self) -> &CompC2csr {
        &self.comp_c2csr
    }
    #[doc = "0x08 - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c3csr(&self) -> &CompC3csr {
        &self.comp_c3csr
    }
    #[doc = "0x0c - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c4csr(&self) -> &CompC4csr {
        &self.comp_c4csr
    }
}
#[doc = "COMP_C1CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_c1csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_c1csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c1csr`]
module"]
#[doc(alias = "COMP_C1CSR")]
pub type CompC1csr = crate::Reg<comp_c1csr::CompC1csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c1csr;
#[doc = "COMP_C2CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_c2csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_c2csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c2csr`]
module"]
#[doc(alias = "COMP_C2CSR")]
pub type CompC2csr = crate::Reg<comp_c2csr::CompC2csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c2csr;
#[doc = "COMP_C3CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_c3csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_c3csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c3csr`]
module"]
#[doc(alias = "COMP_C3CSR")]
pub type CompC3csr = crate::Reg<comp_c3csr::CompC3csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c3csr;
#[doc = "COMP_C4CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_c4csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_c4csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c4csr`]
module"]
#[doc(alias = "COMP_C4CSR")]
pub type CompC4csr = crate::Reg<comp_c4csr::CompC4csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c4csr;
