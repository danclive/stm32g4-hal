#[doc = "APB Low Freeze Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb1hFz(pub u32);
impl Apb1hFz {
    #[doc = "DBG_I2C4_STOP"]
    #[inline(always)]
    pub const fn dbg_i2c4_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DBG_I2C4_STOP"]
    #[inline(always)]
    pub fn set_dbg_i2c4_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Apb1hFz {
    #[inline(always)]
    fn default() -> Apb1hFz {
        Apb1hFz(0)
    }
}
#[doc = "APB Low Freeze Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb1lFz(pub u32);
impl Apb1lFz {
    #[doc = "Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub const fn dbg_timer2_stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn set_dbg_timer2_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim3_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim3_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim4_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim4_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim5_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim5_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub const fn dbg_timer6_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn set_dbg_timer6_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim7_stop(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim7_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub const fn dbg_rtc_stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn set_dbg_rtc_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub const fn dbg_wwdg_stop(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn set_dbg_wwdg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub const fn dbg_iwdg_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_i2c1_stop(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_i2c1_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_i2c2_stop(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_i2c2_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "I2C3 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_i2c3_stop(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "I2C3 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_i2c3_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_lptimer_stop(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_lptimer_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Apb1lFz {
    #[inline(always)]
    fn default() -> Apb1lFz {
        Apb1lFz(0)
    }
}
#[doc = "APB High Freeze Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apb2Fz(pub u32);
impl Apb2Fz {
    #[doc = "TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim1_stop(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim1_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim8_stop(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim8_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim15_stop(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim15_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim16_stop(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim16_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim17_stop(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim17_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub const fn dbg_tim20_stop(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub fn set_dbg_tim20_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub const fn dbg_hrtim0_stop(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn set_dbg_hrtim0_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub const fn dbg_hrtim1_stop(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn set_dbg_hrtim1_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub const fn dbg_hrtim2_stop(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn set_dbg_hrtim2_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub const fn dbg_hrtim3_stop(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn set_dbg_hrtim3_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Apb2Fz {
    #[inline(always)]
    fn default() -> Apb2Fz {
        Apb2Fz(0)
    }
}
#[doc = "Debug MCU Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Debug Sleep Mode"]
    #[inline(always)]
    pub const fn dbg_sleep(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Sleep Mode"]
    #[inline(always)]
    pub fn set_dbg_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Debug Stop Mode"]
    #[inline(always)]
    pub const fn dbg_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Stop Mode"]
    #[inline(always)]
    pub fn set_dbg_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug Standby Mode"]
    #[inline(always)]
    pub const fn dbg_standby(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Standby Mode"]
    #[inline(always)]
    pub fn set_dbg_standby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Trace pin assignment control"]
    #[inline(always)]
    pub const fn trace_ioen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Trace pin assignment control"]
    #[inline(always)]
    pub fn set_trace_ioen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trace pin assignment control"]
    #[inline(always)]
    pub const fn trace_mode(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Trace pin assignment control"]
    #[inline(always)]
    pub fn set_trace_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "MCU Device ID Code Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idcode(pub u32);
impl Idcode {
    #[doc = "Device Identifier"]
    #[inline(always)]
    pub const fn dev_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device Identifier"]
    #[inline(always)]
    pub fn set_dev_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Revision Identifier"]
    #[inline(always)]
    pub const fn rev_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Revision Identifier"]
    #[inline(always)]
    pub fn set_rev_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Idcode {
    #[inline(always)]
    fn default() -> Idcode {
        Idcode(0)
    }
}
