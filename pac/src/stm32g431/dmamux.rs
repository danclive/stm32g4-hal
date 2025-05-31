#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c0cr: C0cr,
    c1cr: C1cr,
    c2cr: C2cr,
    c3cr: C3cr,
    c4cr: C4cr,
    c5cr: C5cr,
    c6cr: C6cr,
    c7cr: C7cr,
    c8cr: C8cr,
    c9cr: C9cr,
    c10cr: C10cr,
    c11cr: C11cr,
    c12cr: C12cr,
    c13cr: C13cr,
    c14cr: C14cr,
    c15cr: C15cr,
    _reserved16: [u8; 0x40],
    csr: Csr,
    cfr: Cfr,
    _reserved18: [u8; 0x78],
    rg0cr: Rg0cr,
    rg1cr: Rg1cr,
    rg2cr: Rg2cr,
    rg3cr: Rg3cr,
    _reserved22: [u8; 0x30],
    rgsr: Rgsr,
    rgcfr: Rgcfr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0cr {
        &self.c0cr
    }
    #[doc = "0x04 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1cr {
        &self.c1cr
    }
    #[doc = "0x08 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2cr {
        &self.c2cr
    }
    #[doc = "0x0c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3cr {
        &self.c3cr
    }
    #[doc = "0x10 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4cr {
        &self.c4cr
    }
    #[doc = "0x14 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5cr {
        &self.c5cr
    }
    #[doc = "0x18 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6cr {
        &self.c6cr
    }
    #[doc = "0x1c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7cr {
        &self.c7cr
    }
    #[doc = "0x20 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c8cr(&self) -> &C8cr {
        &self.c8cr
    }
    #[doc = "0x24 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c9cr(&self) -> &C9cr {
        &self.c9cr
    }
    #[doc = "0x28 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c10cr(&self) -> &C10cr {
        &self.c10cr
    }
    #[doc = "0x2c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c11cr(&self) -> &C11cr {
        &self.c11cr
    }
    #[doc = "0x30 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c12cr(&self) -> &C12cr {
        &self.c12cr
    }
    #[doc = "0x34 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c13cr(&self) -> &C13cr {
        &self.c13cr
    }
    #[doc = "0x38 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c14cr(&self) -> &C14cr {
        &self.c14cr
    }
    #[doc = "0x3c - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn c15cr(&self) -> &C15cr {
        &self.c15cr
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn cfr(&self) -> &Cfr {
        &self.cfr
    }
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg0cr(&self) -> &Rg0cr {
        &self.rg0cr
    }
    #[doc = "0x104 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg1cr(&self) -> &Rg1cr {
        &self.rg1cr
    }
    #[doc = "0x108 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg2cr(&self) -> &Rg2cr {
        &self.rg2cr
    }
    #[doc = "0x10c - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rg3cr(&self) -> &Rg3cr {
        &self.rg3cr
    }
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    #[inline(always)]
    pub const fn rgsr(&self) -> &Rgsr {
        &self.rgsr
    }
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    #[inline(always)]
    pub const fn rgcfr(&self) -> &Rgcfr {
        &self.rgcfr
    }
}
#[doc = "C0CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0cr`] module"]
#[doc(alias = "C0CR")]
pub type C0cr = crate::Reg<c0cr::C0crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c0cr;
#[doc = "C1CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cr`] module"]
#[doc(alias = "C1CR")]
pub type C1cr = crate::Reg<c1cr::C1crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c1cr;
#[doc = "C2CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cr`] module"]
#[doc(alias = "C2CR")]
pub type C2cr = crate::Reg<c2cr::C2crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c2cr;
#[doc = "C3CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3cr`] module"]
#[doc(alias = "C3CR")]
pub type C3cr = crate::Reg<c3cr::C3crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c3cr;
#[doc = "C4CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4cr`] module"]
#[doc(alias = "C4CR")]
pub type C4cr = crate::Reg<c4cr::C4crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c4cr;
#[doc = "C5CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5cr`] module"]
#[doc(alias = "C5CR")]
pub type C5cr = crate::Reg<c5cr::C5crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c5cr;
#[doc = "C6CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6cr`] module"]
#[doc(alias = "C6CR")]
pub type C6cr = crate::Reg<c6cr::C6crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c6cr;
#[doc = "C7CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7cr`] module"]
#[doc(alias = "C7CR")]
pub type C7cr = crate::Reg<c7cr::C7crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c7cr;
#[doc = "C8CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c8cr`] module"]
#[doc(alias = "C8CR")]
pub type C8cr = crate::Reg<c8cr::C8crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c8cr;
#[doc = "C9CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c9cr`] module"]
#[doc(alias = "C9CR")]
pub type C9cr = crate::Reg<c9cr::C9crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c9cr;
#[doc = "C10CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c10cr`] module"]
#[doc(alias = "C10CR")]
pub type C10cr = crate::Reg<c10cr::C10crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c10cr;
#[doc = "C11CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c11cr`] module"]
#[doc(alias = "C11CR")]
pub type C11cr = crate::Reg<c11cr::C11crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c11cr;
#[doc = "C12CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c12cr`] module"]
#[doc(alias = "C12CR")]
pub type C12cr = crate::Reg<c12cr::C12crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c12cr;
#[doc = "C13CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c13cr`] module"]
#[doc(alias = "C13CR")]
pub type C13cr = crate::Reg<c13cr::C13crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c13cr;
#[doc = "C14CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c14cr`] module"]
#[doc(alias = "C14CR")]
pub type C14cr = crate::Reg<c14cr::C14crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c14cr;
#[doc = "C15CR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c15cr`] module"]
#[doc(alias = "C15CR")]
pub type C15cr = crate::Reg<c15cr::C15crSpec>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod c15cr;
#[doc = "RG0CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg0cr`] module"]
#[doc(alias = "RG0CR")]
pub type Rg0cr = crate::Reg<rg0cr::Rg0crSpec>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg0cr;
#[doc = "RG1CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg1cr`] module"]
#[doc(alias = "RG1CR")]
pub type Rg1cr = crate::Reg<rg1cr::Rg1crSpec>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg1cr;
#[doc = "RG2CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg2cr`] module"]
#[doc(alias = "RG2CR")]
pub type Rg2cr = crate::Reg<rg2cr::Rg2crSpec>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg2cr;
#[doc = "RG3CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg3cr`] module"]
#[doc(alias = "RG3CR")]
pub type Rg3cr = crate::Reg<rg3cr::Rg3crSpec>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg3cr;
#[doc = "RGSR (r) register accessor: DMAMux - DMA request generator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`] module"]
#[doc(alias = "RGSR")]
pub type Rgsr = crate::Reg<rgsr::RgsrSpec>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcfr`] module"]
#[doc(alias = "RGCFR")]
pub type Rgcfr = crate::Reg<rgcfr::RgcfrSpec>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
#[doc = "CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`] module"]
#[doc(alias = "CFR")]
pub type Cfr = crate::Reg<cfr::CfrSpec>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
