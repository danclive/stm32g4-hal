#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp_c1csr: CompC1csr,
    comp_c2csr: CompC2csr,
    comp_c3csr: CompC3csr,
    comp_c4csr: CompC4csr,
    comp_c5csr: CompC5csr,
    comp_c6csr: CompC6csr,
    comp_c7csr: CompC7csr,
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
    #[doc = "0x10 - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c5csr(&self) -> &CompC5csr {
        &self.comp_c5csr
    }
    #[doc = "0x14 - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c6csr(&self) -> &CompC6csr {
        &self.comp_c6csr
    }
    #[doc = "0x18 - Comparator control/status register"]
    #[inline(always)]
    pub const fn comp_c7csr(&self) -> &CompC7csr {
        &self.comp_c7csr
    }
}
#[doc = "COMP_C1CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c1csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c1csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c1csr`]
module"]
#[doc(alias = "COMP_C1CSR")]
pub type CompC1csr = crate::Reg<comp_c1csr::CompC1csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c1csr;
#[doc = "COMP_C2CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c2csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c2csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c2csr`]
module"]
#[doc(alias = "COMP_C2CSR")]
pub type CompC2csr = crate::Reg<comp_c2csr::CompC2csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c2csr;
#[doc = "COMP_C3CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c3csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c3csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c3csr`]
module"]
#[doc(alias = "COMP_C3CSR")]
pub type CompC3csr = crate::Reg<comp_c3csr::CompC3csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c3csr;
#[doc = "COMP_C4CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c4csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c4csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c4csr`]
module"]
#[doc(alias = "COMP_C4CSR")]
pub type CompC4csr = crate::Reg<comp_c4csr::CompC4csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c4csr;
#[doc = "COMP_C5CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c5csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c5csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c5csr`]
module"]
#[doc(alias = "COMP_C5CSR")]
pub type CompC5csr = crate::Reg<comp_c5csr::CompC5csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c5csr;
#[doc = "COMP_C6CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c6csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c6csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c6csr`]
module"]
#[doc(alias = "COMP_C6CSR")]
pub type CompC6csr = crate::Reg<comp_c6csr::CompC6csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c6csr;
#[doc = "COMP_C7CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_c7csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_c7csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_c7csr`]
module"]
#[doc(alias = "COMP_C7CSR")]
pub type CompC7csr = crate::Reg<comp_c7csr::CompC7csrSpec>;
#[doc = "Comparator control/status register"]
pub mod comp_c7csr;
