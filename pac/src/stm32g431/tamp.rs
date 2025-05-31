#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    _reserved2: [u8; 0x04],
    fltcr: Fltcr,
    _reserved3: [u8; 0x1c],
    ier: Ier,
    sr: Sr,
    misr: Misr,
    _reserved6: [u8; 0x04],
    scr: Scr,
    _reserved7: [u8; 0xc0],
    bkp0r: Bkp0r,
    bkp1r: Bkp1r,
    bkp2r: Bkp2r,
    bkp3r: Bkp3r,
    bkp4r: Bkp4r,
    bkp5r: Bkp5r,
    bkp6r: Bkp6r,
    bkp7r: Bkp7r,
    bkp8r: Bkp8r,
    bkp9r: Bkp9r,
    bkp10r: Bkp10r,
    bkp11r: Bkp11r,
    bkp12r: Bkp12r,
    bkp13r: Bkp13r,
    bkp14r: Bkp14r,
    bkp15r: Bkp15r,
    bkp16r: Bkp16r,
    bkp17r: Bkp17r,
    bkp18r: Bkp18r,
    bkp19r: Bkp19r,
    bkp20r: Bkp20r,
    bkp21r: Bkp21r,
    bkp22r: Bkp22r,
    bkp23r: Bkp23r,
    bkp24r: Bkp24r,
    bkp25r: Bkp25r,
    bkp26r: Bkp26r,
    bkp27r: Bkp27r,
    bkp28r: Bkp28r,
    bkp29r: Bkp29r,
    bkp30r: Bkp30r,
    bkp31r: Bkp31r,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x0c - TAMP filter control register"]
    #[inline(always)]
    pub const fn fltcr(&self) -> &Fltcr {
        &self.fltcr
    }
    #[doc = "0x2c - TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x30 - TAMP status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x34 - TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &Misr {
        &self.misr
    }
    #[doc = "0x3c - TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x100 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp0r(&self) -> &Bkp0r {
        &self.bkp0r
    }
    #[doc = "0x104 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp1r(&self) -> &Bkp1r {
        &self.bkp1r
    }
    #[doc = "0x108 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp2r(&self) -> &Bkp2r {
        &self.bkp2r
    }
    #[doc = "0x10c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp3r(&self) -> &Bkp3r {
        &self.bkp3r
    }
    #[doc = "0x110 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp4r(&self) -> &Bkp4r {
        &self.bkp4r
    }
    #[doc = "0x114 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp5r(&self) -> &Bkp5r {
        &self.bkp5r
    }
    #[doc = "0x118 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp6r(&self) -> &Bkp6r {
        &self.bkp6r
    }
    #[doc = "0x11c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp7r(&self) -> &Bkp7r {
        &self.bkp7r
    }
    #[doc = "0x120 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp8r(&self) -> &Bkp8r {
        &self.bkp8r
    }
    #[doc = "0x124 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp9r(&self) -> &Bkp9r {
        &self.bkp9r
    }
    #[doc = "0x128 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp10r(&self) -> &Bkp10r {
        &self.bkp10r
    }
    #[doc = "0x12c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp11r(&self) -> &Bkp11r {
        &self.bkp11r
    }
    #[doc = "0x130 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp12r(&self) -> &Bkp12r {
        &self.bkp12r
    }
    #[doc = "0x134 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp13r(&self) -> &Bkp13r {
        &self.bkp13r
    }
    #[doc = "0x138 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp14r(&self) -> &Bkp14r {
        &self.bkp14r
    }
    #[doc = "0x13c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp15r(&self) -> &Bkp15r {
        &self.bkp15r
    }
    #[doc = "0x140 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp16r(&self) -> &Bkp16r {
        &self.bkp16r
    }
    #[doc = "0x144 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp17r(&self) -> &Bkp17r {
        &self.bkp17r
    }
    #[doc = "0x148 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp18r(&self) -> &Bkp18r {
        &self.bkp18r
    }
    #[doc = "0x14c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp19r(&self) -> &Bkp19r {
        &self.bkp19r
    }
    #[doc = "0x150 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp20r(&self) -> &Bkp20r {
        &self.bkp20r
    }
    #[doc = "0x154 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp21r(&self) -> &Bkp21r {
        &self.bkp21r
    }
    #[doc = "0x158 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp22r(&self) -> &Bkp22r {
        &self.bkp22r
    }
    #[doc = "0x15c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp23r(&self) -> &Bkp23r {
        &self.bkp23r
    }
    #[doc = "0x160 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp24r(&self) -> &Bkp24r {
        &self.bkp24r
    }
    #[doc = "0x164 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp25r(&self) -> &Bkp25r {
        &self.bkp25r
    }
    #[doc = "0x168 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp26r(&self) -> &Bkp26r {
        &self.bkp26r
    }
    #[doc = "0x16c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp27r(&self) -> &Bkp27r {
        &self.bkp27r
    }
    #[doc = "0x170 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp28r(&self) -> &Bkp28r {
        &self.bkp28r
    }
    #[doc = "0x174 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp29r(&self) -> &Bkp29r {
        &self.bkp29r
    }
    #[doc = "0x178 - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp30r(&self) -> &Bkp30r {
        &self.bkp30r
    }
    #[doc = "0x17c - TAMP backup register"]
    #[inline(always)]
    pub const fn bkp31r(&self) -> &Bkp31r {
        &self.bkp31r
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "FLTCR (rw) register accessor: TAMP filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltcr`] module"]
#[doc(alias = "FLTCR")]
pub type Fltcr = crate::Reg<fltcr::FltcrSpec>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "IER (rw) register accessor: TAMP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "SR (r) register accessor: TAMP status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: TAMP masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`] module"]
#[doc(alias = "MISR")]
pub type Misr = crate::Reg<misr::MisrSpec>;
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "SCR (rw) register accessor: TAMP status clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "BKP0R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp0r`] module"]
#[doc(alias = "BKP0R")]
pub type Bkp0r = crate::Reg<bkp0r::Bkp0rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp0r;
#[doc = "BKP1R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp1r`] module"]
#[doc(alias = "BKP1R")]
pub type Bkp1r = crate::Reg<bkp1r::Bkp1rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp1r;
#[doc = "BKP2R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp2r`] module"]
#[doc(alias = "BKP2R")]
pub type Bkp2r = crate::Reg<bkp2r::Bkp2rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp2r;
#[doc = "BKP3R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp3r`] module"]
#[doc(alias = "BKP3R")]
pub type Bkp3r = crate::Reg<bkp3r::Bkp3rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp3r;
#[doc = "BKP4R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp4r`] module"]
#[doc(alias = "BKP4R")]
pub type Bkp4r = crate::Reg<bkp4r::Bkp4rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp4r;
#[doc = "BKP5R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp5r`] module"]
#[doc(alias = "BKP5R")]
pub type Bkp5r = crate::Reg<bkp5r::Bkp5rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp5r;
#[doc = "BKP6R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp6r`] module"]
#[doc(alias = "BKP6R")]
pub type Bkp6r = crate::Reg<bkp6r::Bkp6rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp6r;
#[doc = "BKP7R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp7r`] module"]
#[doc(alias = "BKP7R")]
pub type Bkp7r = crate::Reg<bkp7r::Bkp7rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp7r;
#[doc = "BKP8R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp8r`] module"]
#[doc(alias = "BKP8R")]
pub type Bkp8r = crate::Reg<bkp8r::Bkp8rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp8r;
#[doc = "BKP9R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp9r`] module"]
#[doc(alias = "BKP9R")]
pub type Bkp9r = crate::Reg<bkp9r::Bkp9rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp9r;
#[doc = "BKP10R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp10r`] module"]
#[doc(alias = "BKP10R")]
pub type Bkp10r = crate::Reg<bkp10r::Bkp10rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp10r;
#[doc = "BKP11R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp11r`] module"]
#[doc(alias = "BKP11R")]
pub type Bkp11r = crate::Reg<bkp11r::Bkp11rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp11r;
#[doc = "BKP12R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp12r`] module"]
#[doc(alias = "BKP12R")]
pub type Bkp12r = crate::Reg<bkp12r::Bkp12rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp12r;
#[doc = "BKP13R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp13r`] module"]
#[doc(alias = "BKP13R")]
pub type Bkp13r = crate::Reg<bkp13r::Bkp13rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp13r;
#[doc = "BKP14R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp14r`] module"]
#[doc(alias = "BKP14R")]
pub type Bkp14r = crate::Reg<bkp14r::Bkp14rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp14r;
#[doc = "BKP15R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp15r`] module"]
#[doc(alias = "BKP15R")]
pub type Bkp15r = crate::Reg<bkp15r::Bkp15rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp15r;
#[doc = "BKP16R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp16r`] module"]
#[doc(alias = "BKP16R")]
pub type Bkp16r = crate::Reg<bkp16r::Bkp16rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp16r;
#[doc = "BKP17R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp17r`] module"]
#[doc(alias = "BKP17R")]
pub type Bkp17r = crate::Reg<bkp17r::Bkp17rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp17r;
#[doc = "BKP18R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp18r`] module"]
#[doc(alias = "BKP18R")]
pub type Bkp18r = crate::Reg<bkp18r::Bkp18rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp18r;
#[doc = "BKP19R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp19r`] module"]
#[doc(alias = "BKP19R")]
pub type Bkp19r = crate::Reg<bkp19r::Bkp19rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp19r;
#[doc = "BKP20R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp20r`] module"]
#[doc(alias = "BKP20R")]
pub type Bkp20r = crate::Reg<bkp20r::Bkp20rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp20r;
#[doc = "BKP21R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp21r`] module"]
#[doc(alias = "BKP21R")]
pub type Bkp21r = crate::Reg<bkp21r::Bkp21rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp21r;
#[doc = "BKP22R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp22r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp22r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp22r`] module"]
#[doc(alias = "BKP22R")]
pub type Bkp22r = crate::Reg<bkp22r::Bkp22rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp22r;
#[doc = "BKP23R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp23r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp23r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp23r`] module"]
#[doc(alias = "BKP23R")]
pub type Bkp23r = crate::Reg<bkp23r::Bkp23rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp23r;
#[doc = "BKP24R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp24r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp24r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp24r`] module"]
#[doc(alias = "BKP24R")]
pub type Bkp24r = crate::Reg<bkp24r::Bkp24rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp24r;
#[doc = "BKP25R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp25r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp25r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp25r`] module"]
#[doc(alias = "BKP25R")]
pub type Bkp25r = crate::Reg<bkp25r::Bkp25rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp25r;
#[doc = "BKP26R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp26r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp26r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp26r`] module"]
#[doc(alias = "BKP26R")]
pub type Bkp26r = crate::Reg<bkp26r::Bkp26rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp26r;
#[doc = "BKP27R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp27r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp27r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp27r`] module"]
#[doc(alias = "BKP27R")]
pub type Bkp27r = crate::Reg<bkp27r::Bkp27rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp27r;
#[doc = "BKP28R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp28r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp28r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp28r`] module"]
#[doc(alias = "BKP28R")]
pub type Bkp28r = crate::Reg<bkp28r::Bkp28rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp28r;
#[doc = "BKP29R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp29r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp29r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp29r`] module"]
#[doc(alias = "BKP29R")]
pub type Bkp29r = crate::Reg<bkp29r::Bkp29rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp29r;
#[doc = "BKP30R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp30r`] module"]
#[doc(alias = "BKP30R")]
pub type Bkp30r = crate::Reg<bkp30r::Bkp30rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp30r;
#[doc = "BKP31R (rw) register accessor: TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkp31r`] module"]
#[doc(alias = "BKP31R")]
pub type Bkp31r = crate::Reg<bkp31r::Bkp31rSpec>;
#[doc = "TAMP backup register"]
pub mod bkp31r;
