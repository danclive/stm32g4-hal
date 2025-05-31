#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp1_csr: Opamp1Csr,
    opamp2_csr: Opamp2Csr,
    opamp3_csr: Opamp3Csr,
    _reserved3: [u8; 0x0c],
    opamp1_tcmr: Opamp1Tcmr,
    opamp2_tcmr: Opamp2Tcmr,
    opamp3_tcmr: Opamp3Tcmr,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &Opamp1Csr {
        &self.opamp1_csr
    }
    #[doc = "0x04 - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &Opamp2Csr {
        &self.opamp2_csr
    }
    #[doc = "0x08 - OPAMP3 control/status register"]
    #[inline(always)]
    pub const fn opamp3_csr(&self) -> &Opamp3Csr {
        &self.opamp3_csr
    }
    #[doc = "0x18 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_tcmr(&self) -> &Opamp1Tcmr {
        &self.opamp1_tcmr
    }
    #[doc = "0x1c - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_tcmr(&self) -> &Opamp2Tcmr {
        &self.opamp2_tcmr
    }
    #[doc = "0x20 - OPAMP3 control/status register"]
    #[inline(always)]
    pub const fn opamp3_tcmr(&self) -> &Opamp3Tcmr {
        &self.opamp3_tcmr
    }
}
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`] module"]
#[doc(alias = "OPAMP1_CSR")]
pub type Opamp1Csr = crate::Reg<opamp1_csr::Opamp1CsrSpec>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP2_CSR (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`] module"]
#[doc(alias = "OPAMP2_CSR")]
pub type Opamp2Csr = crate::Reg<opamp2_csr::Opamp2CsrSpec>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: OPAMP3 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp3_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp3_csr`] module"]
#[doc(alias = "OPAMP3_CSR")]
pub type Opamp3Csr = crate::Reg<opamp3_csr::Opamp3CsrSpec>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_csr;
#[doc = "OPAMP1_TCMR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_tcmr`] module"]
#[doc(alias = "OPAMP1_TCMR")]
pub type Opamp1Tcmr = crate::Reg<opamp1_tcmr::Opamp1TcmrSpec>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_tcmr;
#[doc = "OPAMP2_TCMR (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_tcmr`] module"]
#[doc(alias = "OPAMP2_TCMR")]
pub type Opamp2Tcmr = crate::Reg<opamp2_tcmr::Opamp2TcmrSpec>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_tcmr;
#[doc = "OPAMP3_TCMR (rw) register accessor: OPAMP3 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp3_tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp3_tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp3_tcmr`] module"]
#[doc(alias = "OPAMP3_TCMR")]
pub type Opamp3Tcmr = crate::Reg<opamp3_tcmr::Opamp3TcmrSpec>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_tcmr;
