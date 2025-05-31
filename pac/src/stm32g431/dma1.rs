#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ifcr: Ifcr,
    ccr1: Ccr1,
    cndtr1: Cndtr1,
    cpar1: Cpar1,
    cmar1: Cmar1,
    _reserved6: [u8; 0x04],
    ccr2: Ccr2,
    cndtr2: Cndtr2,
    cpar2: Cpar2,
    cmar2: Cmar2,
    _reserved10: [u8; 0x04],
    ccr3: Ccr3,
    cndtr3: Cndtr3,
    cpar3: Cpar3,
    cmar3: Cmar3,
    _reserved14: [u8; 0x04],
    ccr4: Ccr4,
    cndtr4: Cndtr4,
    cpar4: Cpar4,
    cmar4: Cmar4,
    _reserved18: [u8; 0x04],
    ccr5: Ccr5,
    cndtr5: Cndtr5,
    cpar5: Cpar5,
    cmar5: Cmar5,
    _reserved22: [u8; 0x04],
    ccr6: Ccr6,
    cndtr6: Cndtr6,
    cpar6: Cpar6,
    cmar6: Cmar6,
    _reserved26: [u8; 0x04],
    ccr7: Ccr7,
    cndtr7: Cndtr7,
    cpar7: Cpar7,
    cmar7: Cmar7,
    _reserved30: [u8; 0x04],
    ccr8: Ccr8,
    cndtr8: Cndtr8,
    cpar8: Cpar8,
    cmar8: Cmar8,
}
impl RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - DMA interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x08 - DMA channel 1 configuration register"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x0c - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr1(&self) -> &Cndtr1 {
        &self.cndtr1
    }
    #[doc = "0x10 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar1(&self) -> &Cpar1 {
        &self.cpar1
    }
    #[doc = "0x14 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar1(&self) -> &Cmar1 {
        &self.cmar1
    }
    #[doc = "0x1c - DMA channel 2 configuration register"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x20 - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr2(&self) -> &Cndtr2 {
        &self.cndtr2
    }
    #[doc = "0x24 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar2(&self) -> &Cpar2 {
        &self.cpar2
    }
    #[doc = "0x28 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar2(&self) -> &Cmar2 {
        &self.cmar2
    }
    #[doc = "0x30 - DMA channel 3 configuration register"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x34 - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr3(&self) -> &Cndtr3 {
        &self.cndtr3
    }
    #[doc = "0x38 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar3(&self) -> &Cpar3 {
        &self.cpar3
    }
    #[doc = "0x3c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar3(&self) -> &Cmar3 {
        &self.cmar3
    }
    #[doc = "0x44 - DMA channel 3 configuration register"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x48 - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr4(&self) -> &Cndtr4 {
        &self.cndtr4
    }
    #[doc = "0x4c - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar4(&self) -> &Cpar4 {
        &self.cpar4
    }
    #[doc = "0x50 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar4(&self) -> &Cmar4 {
        &self.cmar4
    }
    #[doc = "0x58 - DMA channel 4 configuration register"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    #[doc = "0x5c - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr5(&self) -> &Cndtr5 {
        &self.cndtr5
    }
    #[doc = "0x60 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar5(&self) -> &Cpar5 {
        &self.cpar5
    }
    #[doc = "0x64 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar5(&self) -> &Cmar5 {
        &self.cmar5
    }
    #[doc = "0x6c - DMA channel 5 configuration register"]
    #[inline(always)]
    pub const fn ccr6(&self) -> &Ccr6 {
        &self.ccr6
    }
    #[doc = "0x70 - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr6(&self) -> &Cndtr6 {
        &self.cndtr6
    }
    #[doc = "0x74 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar6(&self) -> &Cpar6 {
        &self.cpar6
    }
    #[doc = "0x78 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar6(&self) -> &Cmar6 {
        &self.cmar6
    }
    #[doc = "0x80 - DMA channel 6 configuration register"]
    #[inline(always)]
    pub const fn ccr7(&self) -> &Ccr7 {
        &self.ccr7
    }
    #[doc = "0x84 - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr7(&self) -> &Cndtr7 {
        &self.cndtr7
    }
    #[doc = "0x88 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar7(&self) -> &Cpar7 {
        &self.cpar7
    }
    #[doc = "0x8c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar7(&self) -> &Cmar7 {
        &self.cmar7
    }
    #[doc = "0x94 - DMA channel 7 configuration register"]
    #[inline(always)]
    pub const fn ccr8(&self) -> &Ccr8 {
        &self.ccr8
    }
    #[doc = "0x98 - channel x number of data to transfer register"]
    #[inline(always)]
    pub const fn cndtr8(&self) -> &Cndtr8 {
        &self.cndtr8
    }
    #[doc = "0x9c - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn cpar8(&self) -> &Cpar8 {
        &self.cpar8
    }
    #[doc = "0xa0 - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn cmar8(&self) -> &Cmar8 {
        &self.cmar8
    }
}
#[doc = "ISR (r) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: DMA channel 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "DMA channel 1 configuration register"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: DMA channel 2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "DMA channel 2 configuration register"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: DMA channel 3 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "DMA channel 3 configuration register"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: DMA channel 3 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::Ccr4Spec>;
#[doc = "DMA channel 3 configuration register"]
pub mod ccr4;
#[doc = "CCR5 (rw) register accessor: DMA channel 4 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`] module"]
#[doc(alias = "CCR5")]
pub type Ccr5 = crate::Reg<ccr5::Ccr5Spec>;
#[doc = "DMA channel 4 configuration register"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: DMA channel 5 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr6`] module"]
#[doc(alias = "CCR6")]
pub type Ccr6 = crate::Reg<ccr6::Ccr6Spec>;
#[doc = "DMA channel 5 configuration register"]
pub mod ccr6;
#[doc = "CCR7 (rw) register accessor: DMA channel 6 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr7`] module"]
#[doc(alias = "CCR7")]
pub type Ccr7 = crate::Reg<ccr7::Ccr7Spec>;
#[doc = "DMA channel 6 configuration register"]
pub mod ccr7;
#[doc = "CCR8 (rw) register accessor: DMA channel 7 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr8`] module"]
#[doc(alias = "CCR8")]
pub type Ccr8 = crate::Reg<ccr8::Ccr8Spec>;
#[doc = "DMA channel 7 configuration register"]
pub mod ccr8;
#[doc = "CNDTR1 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`] module"]
#[doc(alias = "CNDTR1")]
pub type Cndtr1 = crate::Reg<cndtr1::Cndtr1Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr1;
#[doc = "CNDTR2 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr2`] module"]
#[doc(alias = "CNDTR2")]
pub type Cndtr2 = crate::Reg<cndtr2::Cndtr2Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr2;
#[doc = "CNDTR3 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr3`] module"]
#[doc(alias = "CNDTR3")]
pub type Cndtr3 = crate::Reg<cndtr3::Cndtr3Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr3;
#[doc = "CNDTR4 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr4`] module"]
#[doc(alias = "CNDTR4")]
pub type Cndtr4 = crate::Reg<cndtr4::Cndtr4Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr4;
#[doc = "CNDTR5 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr5`] module"]
#[doc(alias = "CNDTR5")]
pub type Cndtr5 = crate::Reg<cndtr5::Cndtr5Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr5;
#[doc = "CNDTR6 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr6`] module"]
#[doc(alias = "CNDTR6")]
pub type Cndtr6 = crate::Reg<cndtr6::Cndtr6Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr6;
#[doc = "CNDTR7 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr7`] module"]
#[doc(alias = "CNDTR7")]
pub type Cndtr7 = crate::Reg<cndtr7::Cndtr7Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr7;
#[doc = "CNDTR8 (rw) register accessor: channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr8`] module"]
#[doc(alias = "CNDTR8")]
pub type Cndtr8 = crate::Reg<cndtr8::Cndtr8Spec>;
#[doc = "channel x number of data to transfer register"]
pub mod cndtr8;
#[doc = "CPAR1 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`] module"]
#[doc(alias = "CPAR1")]
pub type Cpar1 = crate::Reg<cpar1::Cpar1Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar1;
#[doc = "CPAR2 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar2`] module"]
#[doc(alias = "CPAR2")]
pub type Cpar2 = crate::Reg<cpar2::Cpar2Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar2;
#[doc = "CPAR3 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar3`] module"]
#[doc(alias = "CPAR3")]
pub type Cpar3 = crate::Reg<cpar3::Cpar3Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar3;
#[doc = "CPAR4 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar4`] module"]
#[doc(alias = "CPAR4")]
pub type Cpar4 = crate::Reg<cpar4::Cpar4Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar4;
#[doc = "CPAR5 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar5`] module"]
#[doc(alias = "CPAR5")]
pub type Cpar5 = crate::Reg<cpar5::Cpar5Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar5;
#[doc = "CPAR6 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar6`] module"]
#[doc(alias = "CPAR6")]
pub type Cpar6 = crate::Reg<cpar6::Cpar6Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar6;
#[doc = "CPAR7 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar7`] module"]
#[doc(alias = "CPAR7")]
pub type Cpar7 = crate::Reg<cpar7::Cpar7Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar7;
#[doc = "CPAR8 (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar8`] module"]
#[doc(alias = "CPAR8")]
pub type Cpar8 = crate::Reg<cpar8::Cpar8Spec>;
#[doc = "DMA channel x peripheral address register"]
pub mod cpar8;
#[doc = "CMAR1 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`] module"]
#[doc(alias = "CMAR1")]
pub type Cmar1 = crate::Reg<cmar1::Cmar1Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar1;
#[doc = "CMAR2 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar2`] module"]
#[doc(alias = "CMAR2")]
pub type Cmar2 = crate::Reg<cmar2::Cmar2Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar2;
#[doc = "CMAR3 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar3`] module"]
#[doc(alias = "CMAR3")]
pub type Cmar3 = crate::Reg<cmar3::Cmar3Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar3;
#[doc = "CMAR4 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar4`] module"]
#[doc(alias = "CMAR4")]
pub type Cmar4 = crate::Reg<cmar4::Cmar4Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar4;
#[doc = "CMAR5 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar5`] module"]
#[doc(alias = "CMAR5")]
pub type Cmar5 = crate::Reg<cmar5::Cmar5Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar5;
#[doc = "CMAR6 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar6`] module"]
#[doc(alias = "CMAR6")]
pub type Cmar6 = crate::Reg<cmar6::Cmar6Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar6;
#[doc = "CMAR7 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar7`] module"]
#[doc(alias = "CMAR7")]
pub type Cmar7 = crate::Reg<cmar7::Cmar7Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar7;
#[doc = "CMAR8 (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar8`] module"]
#[doc(alias = "CMAR8")]
pub type Cmar8 = crate::Reg<cmar8::Cmar8Spec>;
#[doc = "DMA channel x memory address register"]
pub mod cmar8;
