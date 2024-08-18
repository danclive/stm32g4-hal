#[doc = "Autoreload Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "Auto reload value"]
    #[inline(always)]
    pub const fn arr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Auto reload value"]
    #[inline(always)]
    pub fn set_arr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Arr {
    #[inline(always)]
    fn default() -> Arr {
        Arr(0)
    }
}
#[doc = "Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc = "Clock selector"]
    #[inline(always)]
    pub const fn cksel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock selector"]
    #[inline(always)]
    pub fn set_cksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Polarity"]
    #[inline(always)]
    pub const fn ckpol(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Clock Polarity"]
    #[inline(always)]
    pub fn set_ckpol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Configurable digital filter for external clock"]
    #[inline(always)]
    pub const fn ckflt(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Configurable digital filter for external clock"]
    #[inline(always)]
    pub fn set_ckflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Configurable digital filter for trigger"]
    #[inline(always)]
    pub const fn trgflt(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Configurable digital filter for trigger"]
    #[inline(always)]
    pub fn set_trgflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Clock prescaler"]
    #[inline(always)]
    pub const fn presc(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Clock prescaler"]
    #[inline(always)]
    pub fn set_presc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Trigger selector"]
    #[inline(always)]
    pub const fn trigsel(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger selector"]
    #[inline(always)]
    pub fn set_trigsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "Trigger enable and polarity"]
    #[inline(always)]
    pub const fn trigen(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "Trigger enable and polarity"]
    #[inline(always)]
    pub fn set_trigen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "Timeout enable"]
    #[inline(always)]
    pub const fn timout(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout enable"]
    #[inline(always)]
    pub fn set_timout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Waveform shape"]
    #[inline(always)]
    pub const fn wave(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform shape"]
    #[inline(always)]
    pub fn set_wave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Waveform shape polarity"]
    #[inline(always)]
    pub const fn wavpol(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform shape polarity"]
    #[inline(always)]
    pub fn set_wavpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Registers update mode"]
    #[inline(always)]
    pub const fn preload(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Registers update mode"]
    #[inline(always)]
    pub fn set_preload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "counter mode enabled"]
    #[inline(always)]
    pub const fn countmode(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "counter mode enabled"]
    #[inline(always)]
    pub fn set_countmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Encoder mode enable"]
    #[inline(always)]
    pub const fn enc(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Encoder mode enable"]
    #[inline(always)]
    pub fn set_enc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Cfgr {
    #[inline(always)]
    fn default() -> Cfgr {
        Cfgr(0)
    }
}
#[doc = "Compare Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp(pub u32);
impl Cmp {
    #[doc = "Compare value"]
    #[inline(always)]
    pub const fn cmp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare value"]
    #[inline(always)]
    pub fn set_cmp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cmp {
    #[inline(always)]
    fn default() -> Cmp {
        Cmp(0)
    }
}
#[doc = "Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "LPTIM Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM Enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM start in single mode"]
    #[inline(always)]
    pub const fn sngstrt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM start in single mode"]
    #[inline(always)]
    pub fn set_sngstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer start in continuous mode"]
    #[inline(always)]
    pub const fn cntstrt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Timer start in continuous mode"]
    #[inline(always)]
    pub fn set_cntstrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "COUNTRST"]
    #[inline(always)]
    pub const fn countrst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTRST"]
    #[inline(always)]
    pub fn set_countrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RSTARE"]
    #[inline(always)]
    pub const fn rstare(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RSTARE"]
    #[inline(always)]
    pub fn set_rstare(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "compare match Clear Flag"]
    #[inline(always)]
    pub const fn cmpmcf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "compare match Clear Flag"]
    #[inline(always)]
    pub fn set_cmpmcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autoreload match Clear Flag"]
    #[inline(always)]
    pub const fn arrmcf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn set_arrmcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub const fn exttrigcf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn set_exttrigcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare register update OK Clear Flag"]
    #[inline(always)]
    pub const fn cmpokcf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn set_cmpokcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub const fn arrokcf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn set_arrokcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direction change to UP Clear Flag"]
    #[inline(always)]
    pub const fn upcf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to UP Clear Flag"]
    #[inline(always)]
    pub fn set_upcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Direction change to down Clear Flag"]
    #[inline(always)]
    pub const fn downcf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn set_downcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Compare match Interrupt Enable"]
    #[inline(always)]
    pub const fn cmpmie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn set_cmpmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub const fn arrmie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn set_arrmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub const fn exttrigie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn set_exttrigie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub const fn cmpokie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn set_cmpokie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub const fn arrokie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn set_arrokie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub const fn upie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn set_upie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub const fn downie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn set_downie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
#[doc = "Interrupt and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Compare match"]
    #[inline(always)]
    pub const fn cmpm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare match"]
    #[inline(always)]
    pub fn set_cmpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autoreload match"]
    #[inline(always)]
    pub const fn arrm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload match"]
    #[inline(always)]
    pub fn set_arrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger edge event"]
    #[inline(always)]
    pub const fn exttrig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger edge event"]
    #[inline(always)]
    pub fn set_exttrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare register update OK"]
    #[inline(always)]
    pub const fn cmpok(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register update OK"]
    #[inline(always)]
    pub fn set_cmpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Autoreload register update OK"]
    #[inline(always)]
    pub const fn arrok(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload register update OK"]
    #[inline(always)]
    pub fn set_arrok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Counter direction change down to up"]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Counter direction change down to up"]
    #[inline(always)]
    pub fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter direction change up to down"]
    #[inline(always)]
    pub const fn down(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Counter direction change up to down"]
    #[inline(always)]
    pub fn set_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
#[doc = "option register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Or(pub u32);
impl Or {
    #[doc = "IN1"]
    #[inline(always)]
    pub const fn in1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn set_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub const fn in2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn set_in2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IN1_2_1"]
    #[inline(always)]
    pub const fn in1_2_1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "IN1_2_1"]
    #[inline(always)]
    pub fn set_in1_2_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "IN2_2_1"]
    #[inline(always)]
    pub const fn in2_2_1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "IN2_2_1"]
    #[inline(always)]
    pub fn set_in2_2_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Or {
    #[inline(always)]
    fn default() -> Or {
        Or(0)
    }
}
