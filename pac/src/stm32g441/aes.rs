#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    sr: Sr,
    dinr: Dinr,
    doutr: Doutr,
    keyr0: Keyr0,
    keyr1: Keyr1,
    keyr2: Keyr2,
    keyr3: Keyr3,
    ivr0: Ivr0,
    ivr1: Ivr1,
    ivr2: Ivr2,
    ivr3: Ivr3,
    keyr4: Keyr4,
    keyr5: Keyr5,
    keyr6: Keyr6,
    keyr7: Keyr7,
    susp0r: Susp0r,
    susp1r: Susp1r,
    susp2r: Susp2r,
    susp3r: Susp3r,
    susp4r: Susp4r,
    susp5r: Susp5r,
    susp6r: Susp6r,
    susp7r: Susp7r,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x08 - data input register"]
    #[inline(always)]
    pub const fn dinr(&self) -> &Dinr {
        &self.dinr
    }
    #[doc = "0x0c - data output register"]
    #[inline(always)]
    pub const fn doutr(&self) -> &Doutr {
        &self.doutr
    }
    #[doc = "0x10 - key register 0"]
    #[inline(always)]
    pub const fn keyr0(&self) -> &Keyr0 {
        &self.keyr0
    }
    #[doc = "0x14 - key register 1"]
    #[inline(always)]
    pub const fn keyr1(&self) -> &Keyr1 {
        &self.keyr1
    }
    #[doc = "0x18 - key register 2"]
    #[inline(always)]
    pub const fn keyr2(&self) -> &Keyr2 {
        &self.keyr2
    }
    #[doc = "0x1c - key register 3"]
    #[inline(always)]
    pub const fn keyr3(&self) -> &Keyr3 {
        &self.keyr3
    }
    #[doc = "0x20 - initialization vector register 0"]
    #[inline(always)]
    pub const fn ivr0(&self) -> &Ivr0 {
        &self.ivr0
    }
    #[doc = "0x24 - initialization vector register 1"]
    #[inline(always)]
    pub const fn ivr1(&self) -> &Ivr1 {
        &self.ivr1
    }
    #[doc = "0x28 - initialization vector register 2"]
    #[inline(always)]
    pub const fn ivr2(&self) -> &Ivr2 {
        &self.ivr2
    }
    #[doc = "0x2c - initialization vector register 3"]
    #[inline(always)]
    pub const fn ivr3(&self) -> &Ivr3 {
        &self.ivr3
    }
    #[doc = "0x30 - key register 4"]
    #[inline(always)]
    pub const fn keyr4(&self) -> &Keyr4 {
        &self.keyr4
    }
    #[doc = "0x34 - key register 5"]
    #[inline(always)]
    pub const fn keyr5(&self) -> &Keyr5 {
        &self.keyr5
    }
    #[doc = "0x38 - key register 6"]
    #[inline(always)]
    pub const fn keyr6(&self) -> &Keyr6 {
        &self.keyr6
    }
    #[doc = "0x3c - key register 7"]
    #[inline(always)]
    pub const fn keyr7(&self) -> &Keyr7 {
        &self.keyr7
    }
    #[doc = "0x40 - suspend registers"]
    #[inline(always)]
    pub const fn susp0r(&self) -> &Susp0r {
        &self.susp0r
    }
    #[doc = "0x44 - suspend registers"]
    #[inline(always)]
    pub const fn susp1r(&self) -> &Susp1r {
        &self.susp1r
    }
    #[doc = "0x48 - suspend registers"]
    #[inline(always)]
    pub const fn susp2r(&self) -> &Susp2r {
        &self.susp2r
    }
    #[doc = "0x4c - suspend registers"]
    #[inline(always)]
    pub const fn susp3r(&self) -> &Susp3r {
        &self.susp3r
    }
    #[doc = "0x50 - suspend registers"]
    #[inline(always)]
    pub const fn susp4r(&self) -> &Susp4r {
        &self.susp4r
    }
    #[doc = "0x54 - suspend registers"]
    #[inline(always)]
    pub const fn susp5r(&self) -> &Susp5r {
        &self.susp5r
    }
    #[doc = "0x58 - suspend registers"]
    #[inline(always)]
    pub const fn susp6r(&self) -> &Susp6r {
        &self.susp6r
    }
    #[doc = "0x5c - suspend registers"]
    #[inline(always)]
    pub const fn susp7r(&self) -> &Susp7r {
        &self.susp7r
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "DINR (rw) register accessor: data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`]
module"]
#[doc(alias = "DINR")]
pub type Dinr = crate::Reg<dinr::DinrSpec>;
#[doc = "data input register"]
pub mod dinr;
#[doc = "DOUTR (r) register accessor: data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`]
module"]
#[doc(alias = "DOUTR")]
pub type Doutr = crate::Reg<doutr::DoutrSpec>;
#[doc = "data output register"]
pub mod doutr;
#[doc = "KEYR0 (rw) register accessor: key register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr0`]
module"]
#[doc(alias = "KEYR0")]
pub type Keyr0 = crate::Reg<keyr0::Keyr0Spec>;
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "KEYR1 (rw) register accessor: key register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr1`]
module"]
#[doc(alias = "KEYR1")]
pub type Keyr1 = crate::Reg<keyr1::Keyr1Spec>;
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "KEYR2 (rw) register accessor: key register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr2`]
module"]
#[doc(alias = "KEYR2")]
pub type Keyr2 = crate::Reg<keyr2::Keyr2Spec>;
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "KEYR3 (rw) register accessor: key register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr3`]
module"]
#[doc(alias = "KEYR3")]
pub type Keyr3 = crate::Reg<keyr3::Keyr3Spec>;
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "IVR0 (rw) register accessor: initialization vector register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr0`]
module"]
#[doc(alias = "IVR0")]
pub type Ivr0 = crate::Reg<ivr0::Ivr0Spec>;
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 (rw) register accessor: initialization vector register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr1`]
module"]
#[doc(alias = "IVR1")]
pub type Ivr1 = crate::Reg<ivr1::Ivr1Spec>;
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 (rw) register accessor: initialization vector register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr2`]
module"]
#[doc(alias = "IVR2")]
pub type Ivr2 = crate::Reg<ivr2::Ivr2Spec>;
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 (rw) register accessor: initialization vector register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivr3`]
module"]
#[doc(alias = "IVR3")]
pub type Ivr3 = crate::Reg<ivr3::Ivr3Spec>;
#[doc = "initialization vector register 3"]
pub mod ivr3;
#[doc = "KEYR4 (rw) register accessor: key register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr4`]
module"]
#[doc(alias = "KEYR4")]
pub type Keyr4 = crate::Reg<keyr4::Keyr4Spec>;
#[doc = "key register 4"]
pub mod keyr4;
#[doc = "KEYR5 (rw) register accessor: key register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr5`]
module"]
#[doc(alias = "KEYR5")]
pub type Keyr5 = crate::Reg<keyr5::Keyr5Spec>;
#[doc = "key register 5"]
pub mod keyr5;
#[doc = "KEYR6 (rw) register accessor: key register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr6`]
module"]
#[doc(alias = "KEYR6")]
pub type Keyr6 = crate::Reg<keyr6::Keyr6Spec>;
#[doc = "key register 6"]
pub mod keyr6;
#[doc = "KEYR7 (rw) register accessor: key register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr7`]
module"]
#[doc(alias = "KEYR7")]
pub type Keyr7 = crate::Reg<keyr7::Keyr7Spec>;
#[doc = "key register 7"]
pub mod keyr7;
#[doc = "SUSP0R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp0r`]
module"]
#[doc(alias = "SUSP0R")]
pub type Susp0r = crate::Reg<susp0r::Susp0rSpec>;
#[doc = "suspend registers"]
pub mod susp0r;
#[doc = "SUSP1R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp1r`]
module"]
#[doc(alias = "SUSP1R")]
pub type Susp1r = crate::Reg<susp1r::Susp1rSpec>;
#[doc = "suspend registers"]
pub mod susp1r;
#[doc = "SUSP2R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp2r`]
module"]
#[doc(alias = "SUSP2R")]
pub type Susp2r = crate::Reg<susp2r::Susp2rSpec>;
#[doc = "suspend registers"]
pub mod susp2r;
#[doc = "SUSP3R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp3r`]
module"]
#[doc(alias = "SUSP3R")]
pub type Susp3r = crate::Reg<susp3r::Susp3rSpec>;
#[doc = "suspend registers"]
pub mod susp3r;
#[doc = "SUSP4R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp4r`]
module"]
#[doc(alias = "SUSP4R")]
pub type Susp4r = crate::Reg<susp4r::Susp4rSpec>;
#[doc = "suspend registers"]
pub mod susp4r;
#[doc = "SUSP5R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp5r`]
module"]
#[doc(alias = "SUSP5R")]
pub type Susp5r = crate::Reg<susp5r::Susp5rSpec>;
#[doc = "suspend registers"]
pub mod susp5r;
#[doc = "SUSP6R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp6r`]
module"]
#[doc(alias = "SUSP6R")]
pub type Susp6r = crate::Reg<susp6r::Susp6rSpec>;
#[doc = "suspend registers"]
pub mod susp6r;
#[doc = "SUSP7R (rw) register accessor: suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp7r`]
module"]
#[doc(alias = "SUSP7R")]
pub type Susp7r = crate::Reg<susp7r::Susp7rSpec>;
#[doc = "suspend registers"]
pub mod susp7r;
