#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcr: Mcr,
    misr: Misr,
    micr: Micr,
    mdier: Mdier,
    mcntr: Mcntr,
    mper: Mper,
    mrep: Mrep,
    mcmp1r: Mcmp1r,
    _reserved8: [u8; 0x04],
    mcmp2r: Mcmp2r,
    mcmp3r: Mcmp3r,
    mcmp4r: Mcmp4r,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Timer Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x04 - Master Timer Interrupt Status Register"]
    #[inline(always)]
    pub const fn misr(&self) -> &Misr {
        &self.misr
    }
    #[doc = "0x08 - Master Timer Interrupt Clear Register"]
    #[inline(always)]
    pub const fn micr(&self) -> &Micr {
        &self.micr
    }
    #[doc = "0x0c - HRTIM Master Timer DMA / Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mdier(&self) -> &Mdier {
        &self.mdier
    }
    #[doc = "0x10 - Master Timer Counter Register"]
    #[inline(always)]
    pub const fn mcntr(&self) -> &Mcntr {
        &self.mcntr
    }
    #[doc = "0x14 - Master Timer Period Register"]
    #[inline(always)]
    pub const fn mper(&self) -> &Mper {
        &self.mper
    }
    #[doc = "0x18 - Master Timer Repetition Register"]
    #[inline(always)]
    pub const fn mrep(&self) -> &Mrep {
        &self.mrep
    }
    #[doc = "0x1c - Master Timer Compare 1 Register"]
    #[inline(always)]
    pub const fn mcmp1r(&self) -> &Mcmp1r {
        &self.mcmp1r
    }
    #[doc = "0x24 - Master Timer Compare 2 Register"]
    #[inline(always)]
    pub const fn mcmp2r(&self) -> &Mcmp2r {
        &self.mcmp2r
    }
    #[doc = "0x28 - Master Timer Compare 3 Register"]
    #[inline(always)]
    pub const fn mcmp3r(&self) -> &Mcmp3r {
        &self.mcmp3r
    }
    #[doc = "0x2c - Master Timer Compare 4 Register"]
    #[inline(always)]
    pub const fn mcmp4r(&self) -> &Mcmp4r {
        &self.mcmp4r
    }
}
#[doc = "MCR (rw) register accessor: Master Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Master Timer Control Register"]
pub mod mcr;
#[doc = "MISR (r) register accessor: Master Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
#[doc(alias = "MISR")]
pub type Misr = crate::Reg<misr::MisrSpec>;
#[doc = "Master Timer Interrupt Status Register"]
pub mod misr;
#[doc = "MICR (w) register accessor: Master Timer Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@micr`]
module"]
#[doc(alias = "MICR")]
pub type Micr = crate::Reg<micr::MicrSpec>;
#[doc = "Master Timer Interrupt Clear Register"]
pub mod micr;
#[doc = "MDIER (rw) register accessor: HRTIM Master Timer DMA / Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdier`]
module"]
#[doc(alias = "MDIER")]
pub type Mdier = crate::Reg<mdier::MdierSpec>;
#[doc = "HRTIM Master Timer DMA / Interrupt Enable Register"]
pub mod mdier;
#[doc = "MCNTR (rw) register accessor: Master Timer Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcntr`]
module"]
#[doc(alias = "MCNTR")]
pub type Mcntr = crate::Reg<mcntr::McntrSpec>;
#[doc = "Master Timer Counter Register"]
pub mod mcntr;
#[doc = "MPER (rw) register accessor: Master Timer Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mper`]
module"]
#[doc(alias = "MPER")]
pub type Mper = crate::Reg<mper::MperSpec>;
#[doc = "Master Timer Period Register"]
pub mod mper;
#[doc = "MREP (rw) register accessor: Master Timer Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrep`]
module"]
#[doc(alias = "MREP")]
pub type Mrep = crate::Reg<mrep::MrepSpec>;
#[doc = "Master Timer Repetition Register"]
pub mod mrep;
#[doc = "MCMP1R (rw) register accessor: Master Timer Compare 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmp1r`]
module"]
#[doc(alias = "MCMP1R")]
pub type Mcmp1r = crate::Reg<mcmp1r::Mcmp1rSpec>;
#[doc = "Master Timer Compare 1 Register"]
pub mod mcmp1r;
#[doc = "MCMP2R (rw) register accessor: Master Timer Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmp2r`]
module"]
#[doc(alias = "MCMP2R")]
pub type Mcmp2r = crate::Reg<mcmp2r::Mcmp2rSpec>;
#[doc = "Master Timer Compare 2 Register"]
pub mod mcmp2r;
#[doc = "MCMP3R (rw) register accessor: Master Timer Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmp3r`]
module"]
#[doc(alias = "MCMP3R")]
pub type Mcmp3r = crate::Reg<mcmp3r::Mcmp3rSpec>;
#[doc = "Master Timer Compare 3 Register"]
pub mod mcmp3r;
#[doc = "MCMP4R (rw) register accessor: Master Timer Compare 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmp4r`]
module"]
#[doc(alias = "MCMP4R")]
pub type Mcmp4r = crate::Reg<mcmp4r::Mcmp4rSpec>;
#[doc = "Master Timer Compare 4 Register"]
pub mod mcmp4r;
