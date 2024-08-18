#[doc = "DAC calibration control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCcr(pub u32);
impl DacCcr {
    #[doc = "DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub const fn otrim1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub fn set_otrim1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub const fn otrim2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub fn set_otrim2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for DacCcr {
    #[inline(always)]
    fn default() -> DacCcr {
        DacCcr(0)
    }
}
#[doc = "DAC control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCr(pub u32);
impl DacCr {
    #[doc = "DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub const fn en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1."]
    #[inline(always)]
    pub fn set_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC channel1 trigger enable"]
    #[inline(always)]
    pub const fn ten1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn set_ten1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub const fn tsel1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn set_tsel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub const fn wave1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled)."]
    #[inline(always)]
    pub fn set_wave1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub const fn mamp1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn set_mamp1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub const fn dmaen1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn set_dmaen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub const fn dmaudrie1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn set_dmaudrie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub const fn cen1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn set_cen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
    #[inline(always)]
    pub const fn en2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2."]
    #[inline(always)]
    pub fn set_en2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DAC channel2 trigger enable"]
    #[inline(always)]
    pub const fn ten2(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn set_ten2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
    #[inline(always)]
    pub const fn tsel2(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)."]
    #[inline(always)]
    pub fn set_tsel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
    #[inline(always)]
    pub const fn wave2(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)"]
    #[inline(always)]
    pub fn set_wave2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub const fn mamp2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\\[11:0\\] of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn set_mamp2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DAC channel2 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub const fn dmaen2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 DMA enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn set_dmaen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub const fn dmaudrie2(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn set_dmaudrie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub const fn cen2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored."]
    #[inline(always)]
    pub fn set_cen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for DacCr {
    #[inline(always)]
    fn default() -> DacCr {
        DacCr(0)
    }
}
#[doc = "DAC channel1 12-bit left aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr12l1(pub u32);
impl DacDhr12l1 {
    #[doc = "DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dhr(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "DAC channel1 12-bit left-aligned data B"]
    #[inline(always)]
    pub const fn dacc1dhrb(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 12-bit left-aligned data B"]
    #[inline(always)]
    pub fn set_dacc1dhrb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for DacDhr12l1 {
    #[inline(always)]
    fn default() -> DacDhr12l1 {
        DacDhr12l1(0)
    }
}
#[doc = "DAC channel2 12-bit left aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr12l2(pub u32);
impl DacDhr12l2 {
    #[doc = "DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dhr(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "DAC channel2 12-bit left-aligned data B"]
    #[inline(always)]
    pub const fn dacc2dhrb(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 12-bit left-aligned data B"]
    #[inline(always)]
    pub fn set_dacc2dhrb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for DacDhr12l2 {
    #[inline(always)]
    fn default() -> DacDhr12l2 {
        DacDhr12l2(0)
    }
}
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr12ld(pub u32);
impl DacDhr12ld {
    #[doc = "DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dhr(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dhr(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for DacDhr12ld {
    #[inline(always)]
    fn default() -> DacDhr12ld {
        DacDhr12ld(0)
    }
}
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr12r1(pub u32);
impl DacDhr12r1 {
    #[doc = "DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dhr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC channel1 12-bit right-aligned data B"]
    #[inline(always)]
    pub const fn dacc1dhrb(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 12-bit right-aligned data B"]
    #[inline(always)]
    pub fn set_dacc1dhrb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for DacDhr12r1 {
    #[inline(always)]
    fn default() -> DacDhr12r1 {
        DacDhr12r1(0)
    }
}
#[doc = "DAC channel2 12-bit right aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr12r2(pub u32);
impl DacDhr12r2 {
    #[doc = "DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dhr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub const fn dacc2dhrb(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn set_dacc2dhrb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for DacDhr12r2 {
    #[inline(always)]
    fn default() -> DacDhr12r2 {
        DacDhr12r2(0)
    }
}
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr12rd(pub u32);
impl DacDhr12rd {
    #[doc = "DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dhr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dhr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dhr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for DacDhr12rd {
    #[inline(always)]
    fn default() -> DacDhr12rd {
        DacDhr12rd(0)
    }
}
#[doc = "DAC channel1 8-bit right aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr8r1(pub u32);
impl DacDhr8r1 {
    #[doc = "DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dhr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub const fn dacc1dhrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn set_dacc1dhrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for DacDhr8r1 {
    #[inline(always)]
    fn default() -> DacDhr8r1 {
        DacDhr8r1(0)
    }
}
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr8r2(pub u32);
impl DacDhr8r2 {
    #[doc = "DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dhr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub const fn dacc2dhrb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn set_dacc2dhrb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for DacDhr8r2 {
    #[inline(always)]
    fn default() -> DacDhr8r2 {
        DacDhr8r2(0)
    }
}
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDhr8rd(pub u32);
impl DacDhr8rd {
    #[doc = "DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dhr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dhr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for DacDhr8rd {
    #[inline(always)]
    fn default() -> DacDhr8rd {
        DacDhr8rd(0)
    }
}
#[doc = "DAC channel1 data output register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDor1(pub u32);
impl DacDor1 {
    #[doc = "DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
    #[inline(always)]
    pub const fn dacc1dor(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
    #[inline(always)]
    pub fn set_dacc1dor(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC channel1 data output"]
    #[inline(always)]
    pub const fn dacc1dorb(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel1 data output"]
    #[inline(always)]
    pub fn set_dacc1dorb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for DacDor1 {
    #[inline(always)]
    fn default() -> DacDor1 {
        DacDor1(0)
    }
}
#[doc = "DAC channel2 data output register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacDor2(pub u32);
impl DacDor2 {
    #[doc = "DAC channel2 data output These bits are read-only, they contain data output for DAC channel2."]
    #[inline(always)]
    pub const fn dacc2dor(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 data output These bits are read-only, they contain data output for DAC channel2."]
    #[inline(always)]
    pub fn set_dacc2dor(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC channel2 data output"]
    #[inline(always)]
    pub const fn dacc2dorb(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC channel2 data output"]
    #[inline(always)]
    pub fn set_dacc2dorb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for DacDor2 {
    #[inline(always)]
    fn default() -> DacDor2 {
        DacDor2(0)
    }
}
#[doc = "DAC mode control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacMcr(pub u32);
impl DacMcr {
    #[doc = "DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub const fn mode1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn set_mode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub const fn dmadouble1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn set_dmadouble1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable signed format for DAC channel1"]
    #[inline(always)]
    pub const fn sinformat1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn set_sinformat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High frequency interface mode selection"]
    #[inline(always)]
    pub const fn hfsel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "High frequency interface mode selection"]
    #[inline(always)]
    pub fn set_hfsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub const fn mode2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn set_mode2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub const fn dmadouble2(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn set_dmadouble2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable signed format for DAC channel2"]
    #[inline(always)]
    pub const fn sinformat2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn set_sinformat2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for DacMcr {
    #[inline(always)]
    fn default() -> DacMcr {
        DacMcr(0)
    }
}
#[doc = "DAC Sample and Hold hold time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacShhr(pub u32);
impl DacShhr {
    #[doc = "DAC Channel 1 hold Time (only valid in sample &amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub const fn thold1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "DAC Channel 1 hold Time (only valid in sample &amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn set_thold1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "DAC Channel 2 hold time (only valid in sample &amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub const fn thold2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "DAC Channel 2 hold time (only valid in sample &amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn set_thold2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for DacShhr {
    #[inline(always)]
    fn default() -> DacShhr {
        DacShhr(0)
    }
}
#[doc = "DAC Sample and Hold refresh time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacShrr(pub u32);
impl DacShrr {
    #[doc = "DAC Channel 1 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub const fn trefresh1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DAC Channel 1 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn set_trefresh1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DAC Channel 2 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub const fn trefresh2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "DAC Channel 2 refresh Time (only valid in sample &amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn set_trefresh2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for DacShrr {
    #[inline(always)]
    fn default() -> DacShrr {
        DacShrr(0)
    }
}
#[doc = "DAC Sample and Hold sample time register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacShsr1(pub u32);
impl DacShsr1 {
    #[doc = "DAC Channel 1 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub const fn tsample1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "DAC Channel 1 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, If BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn set_tsample1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DacShsr1 {
    #[inline(always)]
    fn default() -> DacShsr1 {
        DacShsr1(0)
    }
}
#[doc = "DAC Sample and Hold sample time register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacShsr2(pub u32);
impl DacShsr2 {
    #[doc = "DAC Channel 2 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, if BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub const fn tsample2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "DAC Channel 2 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, if BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn set_tsample2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DacShsr2 {
    #[inline(always)]
    fn default() -> DacShsr2 {
        DacShsr2(0)
    }
}
#[doc = "DAC status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacSr(pub u32);
impl DacSr {
    #[doc = "DAC channel1 ready status bit"]
    #[inline(always)]
    pub const fn dac1rdy(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 ready status bit"]
    #[inline(always)]
    pub fn set_dac1rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DAC channel1 output register status bit"]
    #[inline(always)]
    pub const fn dorstat1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 output register status bit"]
    #[inline(always)]
    pub fn set_dorstat1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub const fn dmaudr1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn set_dmaudr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub const fn cal_flag1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn set_cal_flag1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
    #[inline(always)]
    pub const fn bwst1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
    #[inline(always)]
    pub fn set_bwst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DAC channel 2 ready status bit"]
    #[inline(always)]
    pub const fn dac2rdy(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel 2 ready status bit"]
    #[inline(always)]
    pub fn set_dac2rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DAC channel 2 output register status bit"]
    #[inline(always)]
    pub const fn dorstat2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel 2 output register status bit"]
    #[inline(always)]
    pub fn set_dorstat2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub const fn dmaudr2(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn set_dmaudr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub const fn cal_flag2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn set_cal_flag2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub const fn bwst2(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub fn set_bwst2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DacSr {
    #[inline(always)]
    fn default() -> DacSr {
        DacSr(0)
    }
}
#[doc = "Sawtooth Mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacStmodr(pub u32);
impl DacStmodr {
    #[doc = "DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub const fn strsttrigsel1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn set_strsttrigsel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub const fn stinctrigsel1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn set_stinctrigsel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub const fn strsttrigsel2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn set_strsttrigsel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub const fn stinctrigsel2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn set_stinctrigsel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DacStmodr {
    #[inline(always)]
    fn default() -> DacStmodr {
        DacStmodr(0)
    }
}
#[doc = "Sawtooth register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacStr1(pub u32);
impl DacStr1 {
    #[doc = "DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub const fn strstdata1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC Channel 1 Sawtooth reset value"]
    #[inline(always)]
    pub fn set_strstdata1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub const fn stdir1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel1 Sawtooth direction setting"]
    #[inline(always)]
    pub fn set_stdir1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub const fn stincdata1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "DAC CH1 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn set_stincdata1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DacStr1 {
    #[inline(always)]
    fn default() -> DacStr1 {
        DacStr1(0)
    }
}
#[doc = "Sawtooth register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacStr2(pub u32);
impl DacStr2 {
    #[doc = "DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub const fn strstdata2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "DAC Channel 2 Sawtooth reset value"]
    #[inline(always)]
    pub fn set_strstdata2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub const fn stdir2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Channel2 Sawtooth direction setting"]
    #[inline(always)]
    pub fn set_stdir2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub const fn stincdata2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "DAC CH2 Sawtooth increment value (12.4 bit format)"]
    #[inline(always)]
    pub fn set_stincdata2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DacStr2 {
    #[inline(always)]
    fn default() -> DacStr2 {
        DacStr2(0)
    }
}
#[doc = "DAC software trigger register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacSwtrgr(pub u32);
impl DacSwtrgr {
    #[doc = "DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    pub const fn swtrig1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    pub fn set_swtrig1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
    #[inline(always)]
    pub const fn swtrig2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
    #[inline(always)]
    pub fn set_swtrig2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DAC channel1 software trigger B"]
    #[inline(always)]
    pub const fn swtrigb1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel1 software trigger B"]
    #[inline(always)]
    pub fn set_swtrigb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DAC channel2 software trigger B"]
    #[inline(always)]
    pub const fn swtrigb2(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DAC channel2 software trigger B"]
    #[inline(always)]
    pub fn set_swtrigb2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for DacSwtrgr {
    #[inline(always)]
    fn default() -> DacSwtrgr {
        DacSwtrgr(0)
    }
}
