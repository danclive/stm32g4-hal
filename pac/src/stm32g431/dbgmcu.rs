#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    cr: Cr,
    apb1l_fz: Apb1lFz,
    apb1h_fz: Apb1hFz,
    apb2_fz: Apb2Fz,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - Debug MCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - APB Low Freeze Register 1"]
    #[inline(always)]
    pub const fn apb1l_fz(&self) -> &Apb1lFz {
        &self.apb1l_fz
    }
    #[doc = "0x0c - APB Low Freeze Register 2"]
    #[inline(always)]
    pub const fn apb1h_fz(&self) -> &Apb1hFz {
        &self.apb1h_fz
    }
    #[doc = "0x10 - APB High Freeze Register"]
    #[inline(always)]
    pub const fn apb2_fz(&self) -> &Apb2Fz {
        &self.apb2_fz
    }
}
#[doc = "IDCODE (r) register accessor: MCU Device ID Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: Debug MCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1L_FZ (rw) register accessor: APB Low Freeze Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1l_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1l_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1l_fz`] module"]
#[doc(alias = "APB1L_FZ")]
pub type Apb1lFz = crate::Reg<apb1l_fz::Apb1lFzSpec>;
#[doc = "APB Low Freeze Register 1"]
pub mod apb1l_fz;
#[doc = "APB1H_FZ (rw) register accessor: APB Low Freeze Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1h_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1h_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1h_fz`] module"]
#[doc(alias = "APB1H_FZ")]
pub type Apb1hFz = crate::Reg<apb1h_fz::Apb1hFzSpec>;
#[doc = "APB Low Freeze Register 2"]
pub mod apb1h_fz;
#[doc = "APB2_FZ (rw) register accessor: APB High Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2_fz`] module"]
#[doc(alias = "APB2_FZ")]
pub type Apb2Fz = crate::Reg<apb2_fz::Apb2FzSpec>;
#[doc = "APB High Freeze Register"]
pub mod apb2_fz;
