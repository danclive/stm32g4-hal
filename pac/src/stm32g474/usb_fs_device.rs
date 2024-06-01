#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ep0r: Ep0r,
    ep1r: Ep1r,
    ep2r: Ep2r,
    ep3r: Ep3r,
    ep4r: Ep4r,
    ep5r: Ep5r,
    ep6r: Ep6r,
    ep7r: Ep7r,
    _reserved8: [u8; 0x20],
    cntr: Cntr,
    istr: Istr,
    fnr: Fnr,
    daddr: Daddr,
    btable: Btable,
}
impl RegisterBlock {
    #[doc = "0x00 - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep0r(&self) -> &Ep0r {
        &self.ep0r
    }
    #[doc = "0x04 - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep1r(&self) -> &Ep1r {
        &self.ep1r
    }
    #[doc = "0x08 - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep2r(&self) -> &Ep2r {
        &self.ep2r
    }
    #[doc = "0x0c - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep3r(&self) -> &Ep3r {
        &self.ep3r
    }
    #[doc = "0x10 - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep4r(&self) -> &Ep4r {
        &self.ep4r
    }
    #[doc = "0x14 - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep5r(&self) -> &Ep5r {
        &self.ep5r
    }
    #[doc = "0x18 - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep6r(&self) -> &Ep6r {
        &self.ep6r
    }
    #[doc = "0x1c - USB endpoint n register"]
    #[inline(always)]
    pub const fn ep7r(&self) -> &Ep7r {
        &self.ep7r
    }
    #[doc = "0x40 - USB control register"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x44 - USB interrupt status register"]
    #[inline(always)]
    pub const fn istr(&self) -> &Istr {
        &self.istr
    }
    #[doc = "0x48 - USB frame number register"]
    #[inline(always)]
    pub const fn fnr(&self) -> &Fnr {
        &self.fnr
    }
    #[doc = "0x4c - USB device address"]
    #[inline(always)]
    pub const fn daddr(&self) -> &Daddr {
        &self.daddr
    }
    #[doc = "0x50 - Buffer table address"]
    #[inline(always)]
    pub const fn btable(&self) -> &Btable {
        &self.btable
    }
}
#[doc = "EP0R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0r`]
module"]
#[doc(alias = "EP0R")]
pub type Ep0r = crate::Reg<ep0r::Ep0rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep0r;
#[doc = "EP1R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1r`]
module"]
#[doc(alias = "EP1R")]
pub type Ep1r = crate::Reg<ep1r::Ep1rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep1r;
#[doc = "EP2R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2r`]
module"]
#[doc(alias = "EP2R")]
pub type Ep2r = crate::Reg<ep2r::Ep2rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep2r;
#[doc = "EP3R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3r`]
module"]
#[doc(alias = "EP3R")]
pub type Ep3r = crate::Reg<ep3r::Ep3rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep3r;
#[doc = "EP4R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4r`]
module"]
#[doc(alias = "EP4R")]
pub type Ep4r = crate::Reg<ep4r::Ep4rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep4r;
#[doc = "EP5R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5r`]
module"]
#[doc(alias = "EP5R")]
pub type Ep5r = crate::Reg<ep5r::Ep5rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep5r;
#[doc = "EP6R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6r`]
module"]
#[doc(alias = "EP6R")]
pub type Ep6r = crate::Reg<ep6r::Ep6rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep6r;
#[doc = "EP7R (rw) register accessor: USB endpoint n register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7r`]
module"]
#[doc(alias = "EP7R")]
pub type Ep7r = crate::Reg<ep7r::Ep7rSpec>;
#[doc = "USB endpoint n register"]
pub mod ep7r;
#[doc = "CNTR (rw) register accessor: USB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "USB control register"]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`]
module"]
#[doc(alias = "ISTR")]
pub type Istr = crate::Reg<istr::IstrSpec>;
#[doc = "USB interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: USB frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`]
module"]
#[doc(alias = "FNR")]
pub type Fnr = crate::Reg<fnr::FnrSpec>;
#[doc = "USB frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: USB device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
#[doc(alias = "DADDR")]
pub type Daddr = crate::Reg<daddr::DaddrSpec>;
#[doc = "USB device address"]
pub mod daddr;
#[doc = "BTABLE (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btable`]
module"]
#[doc(alias = "BTABLE")]
pub type Btable = crate::Reg<btable::BtableSpec>;
#[doc = "Buffer table address"]
pub mod btable;
