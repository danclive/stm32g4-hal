#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tr: Tr,
    dr: Dr,
    ssr: Ssr,
    icsr: Icsr,
    prer: Prer,
    wutr: Wutr,
    cr: Cr,
    _reserved7: [u8; 0x08],
    wpr: Wpr,
    calr: Calr,
    shiftr: Shiftr,
    tstr: Tstr,
    tsdr: Tsdr,
    tsssr: Tsssr,
    _reserved13: [u8; 0x04],
    alrmar: Alrmar,
    alrmassr: Alrmassr,
    alrmbr: Alrmbr,
    alrmbssr: Alrmbssr,
    sr: Sr,
    misr: Misr,
    _reserved19: [u8; 0x04],
    scr: Scr,
}
impl RegisterBlock {
    #[doc = "0x00 - time register"]
    #[inline(always)]
    pub const fn tr(&self) -> &Tr {
        &self.tr
    }
    #[doc = "0x04 - date register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x08 - sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &Ssr {
        &self.ssr
    }
    #[doc = "0x0c - initialization and status register"]
    #[inline(always)]
    pub const fn icsr(&self) -> &Icsr {
        &self.icsr
    }
    #[doc = "0x10 - prescaler register"]
    #[inline(always)]
    pub const fn prer(&self) -> &Prer {
        &self.prer
    }
    #[doc = "0x14 - wakeup timer register"]
    #[inline(always)]
    pub const fn wutr(&self) -> &Wutr {
        &self.wutr
    }
    #[doc = "0x18 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &Wpr {
        &self.wpr
    }
    #[doc = "0x28 - calibration register"]
    #[inline(always)]
    pub const fn calr(&self) -> &Calr {
        &self.calr
    }
    #[doc = "0x2c - shift control register"]
    #[inline(always)]
    pub const fn shiftr(&self) -> &Shiftr {
        &self.shiftr
    }
    #[doc = "0x30 - time stamp time register"]
    #[inline(always)]
    pub const fn tstr(&self) -> &Tstr {
        &self.tstr
    }
    #[doc = "0x34 - time stamp date register"]
    #[inline(always)]
    pub const fn tsdr(&self) -> &Tsdr {
        &self.tsdr
    }
    #[doc = "0x38 - timestamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(&self) -> &Tsssr {
        &self.tsssr
    }
    #[doc = "0x40 - alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &Alrmar {
        &self.alrmar
    }
    #[doc = "0x44 - alarm A sub second register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &Alrmassr {
        &self.alrmassr
    }
    #[doc = "0x48 - alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &Alrmbr {
        &self.alrmbr
    }
    #[doc = "0x4c - alarm B sub second register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &Alrmbssr {
        &self.alrmbssr
    }
    #[doc = "0x50 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x54 - status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &Misr {
        &self.misr
    }
    #[doc = "0x5c - status register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
}
#[doc = "TR (rw) register accessor: time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
#[doc(alias = "TR")]
pub type Tr = crate::Reg<tr::TrSpec>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "date register"]
pub mod dr;
#[doc = "SSR (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
#[doc(alias = "SSR")]
pub type Ssr = crate::Reg<ssr::SsrSpec>;
#[doc = "sub second register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`]
module"]
#[doc(alias = "ICSR")]
pub type Icsr = crate::Reg<icsr::IcsrSpec>;
#[doc = "initialization and status register"]
pub mod icsr;
#[doc = "PRER (rw) register accessor: prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`]
module"]
#[doc(alias = "PRER")]
pub type Prer = crate::Reg<prer::PrerSpec>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`]
module"]
#[doc(alias = "WUTR")]
pub type Wutr = crate::Reg<wutr::WutrSpec>;
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
#[doc = "WPR (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
#[doc(alias = "WPR")]
pub type Wpr = crate::Reg<wpr::WprSpec>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
#[doc(alias = "CALR")]
pub type Calr = crate::Reg<calr::CalrSpec>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`]
module"]
#[doc(alias = "SHIFTR")]
pub type Shiftr = crate::Reg<shiftr::ShiftrSpec>;
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: time stamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstr`]
module"]
#[doc(alias = "TSTR")]
pub type Tstr = crate::Reg<tstr::TstrSpec>;
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: time stamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdr`]
module"]
#[doc(alias = "TSDR")]
pub type Tsdr = crate::Reg<tsdr::TsdrSpec>;
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsssr`]
module"]
#[doc(alias = "TSSSR")]
pub type Tsssr = crate::Reg<tsssr::TsssrSpec>;
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "ALRMAR (rw) register accessor: alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmar`]
module"]
#[doc(alias = "ALRMAR")]
pub type Alrmar = crate::Reg<alrmar::AlrmarSpec>;
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "ALRMASSR (rw) register accessor: alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmassr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmassr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmassr`]
module"]
#[doc(alias = "ALRMASSR")]
pub type Alrmassr = crate::Reg<alrmassr::AlrmassrSpec>;
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "ALRMBR (rw) register accessor: alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbr`]
module"]
#[doc(alias = "ALRMBR")]
pub type Alrmbr = crate::Reg<alrmbr::AlrmbrSpec>;
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "ALRMBSSR (rw) register accessor: alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmbssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmbssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmbssr`]
module"]
#[doc(alias = "ALRMBSSR")]
pub type Alrmbssr = crate::Reg<alrmbssr::AlrmbssrSpec>;
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "MISR (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
#[doc(alias = "MISR")]
pub type Misr = crate::Reg<misr::MisrSpec>;
#[doc = "status register"]
pub mod misr;
#[doc = "SCR (w) register accessor: status register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "status register"]
pub mod scr;
