#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    memrmp: Memrmp,
    cfgr1: Cfgr1,
    exticr1: Exticr1,
    exticr2: Exticr2,
    exticr3: Exticr3,
    exticr4: Exticr4,
    scsr: Scsr,
    cfgr2: Cfgr2,
    swpr: Swpr,
    skr: Skr,
}
impl RegisterBlock {
    #[doc = "0x00 - Remap Memory register"]
    #[inline(always)]
    pub const fn memrmp(&self) -> &Memrmp {
        &self.memrmp
    }
    #[doc = "0x04 - peripheral mode configuration register"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &Exticr1 {
        &self.exticr1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &Exticr2 {
        &self.exticr2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &Exticr3 {
        &self.exticr3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn exticr4(&self) -> &Exticr4 {
        &self.exticr4
    }
    #[doc = "0x18 - CCM SRAM control and status register"]
    #[inline(always)]
    pub const fn scsr(&self) -> &Scsr {
        &self.scsr
    }
    #[doc = "0x1c - configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x20 - SRAM Write protection register 1"]
    #[inline(always)]
    pub const fn swpr(&self) -> &Swpr {
        &self.swpr
    }
    #[doc = "0x24 - SRAM2 Key Register"]
    #[inline(always)]
    pub const fn skr(&self) -> &Skr {
        &self.skr
    }
}
#[doc = "MEMRMP (rw) register accessor: Remap Memory register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memrmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memrmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memrmp`]
module"]
#[doc(alias = "MEMRMP")]
pub type Memrmp = crate::Reg<memrmp::MemrmpSpec>;
#[doc = "Remap Memory register"]
pub mod memrmp;
#[doc = "CFGR1 (rw) register accessor: peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "peripheral mode configuration register"]
pub mod cfgr1;
#[doc = "EXTICR1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`]
module"]
#[doc(alias = "EXTICR1")]
pub type Exticr1 = crate::Reg<exticr1::Exticr1Spec>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`]
module"]
#[doc(alias = "EXTICR2")]
pub type Exticr2 = crate::Reg<exticr2::Exticr2Spec>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`]
module"]
#[doc(alias = "EXTICR3")]
pub type Exticr3 = crate::Reg<exticr3::Exticr3Spec>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`]
module"]
#[doc(alias = "EXTICR4")]
pub type Exticr4 = crate::Reg<exticr4::Exticr4Spec>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SCSR (rw) register accessor: CCM SRAM control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`]
module"]
#[doc(alias = "SCSR")]
pub type Scsr = crate::Reg<scsr::ScsrSpec>;
#[doc = "CCM SRAM control and status register"]
pub mod scsr;
#[doc = "CFGR2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "SWPR (rw) register accessor: SRAM Write protection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpr`]
module"]
#[doc(alias = "SWPR")]
pub type Swpr = crate::Reg<swpr::SwprSpec>;
#[doc = "SRAM Write protection register 1"]
pub mod swpr;
#[doc = "SKR (w) register accessor: SRAM2 Key Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@skr`]
module"]
#[doc(alias = "SKR")]
pub type Skr = crate::Reg<skr::SkrSpec>;
#[doc = "SRAM2 Key Register"]
pub mod skr;
