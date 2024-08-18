#[doc = "VREF_BUF Calibration Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrefbufCcr(pub u32);
impl VrefbufCcr {
    #[doc = "Trimming code"]
    #[inline(always)]
    pub const fn trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Trimming code"]
    #[inline(always)]
    pub fn set_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for VrefbufCcr {
    #[inline(always)]
    fn default() -> VrefbufCcr {
        VrefbufCcr(0)
    }
}
#[doc = "VREF_BUF Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrefbufCsr(pub u32);
impl VrefbufCsr {
    #[doc = "Enable Voltage Reference"]
    #[inline(always)]
    pub const fn envr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Voltage Reference"]
    #[inline(always)]
    pub fn set_envr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "High impedence mode for the VREF_BUF"]
    #[inline(always)]
    pub fn set_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Voltage reference buffer ready"]
    #[inline(always)]
    pub const fn vrr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage reference buffer ready"]
    #[inline(always)]
    pub fn set_vrr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Voltage reference scale"]
    #[inline(always)]
    pub const fn vrs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Voltage reference scale"]
    #[inline(always)]
    pub fn set_vrs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for VrefbufCsr {
    #[inline(always)]
    fn default() -> VrefbufCsr {
        VrefbufCsr(0)
    }
}
