#[doc = "alarm A register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alrmar(pub u32);
impl Alrmar {
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub const fn su(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub fn set_su(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Alarm A seconds mask"]
    #[inline(always)]
    pub const fn msk1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A seconds mask"]
    #[inline(always)]
    pub fn set_msk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub const fn mnu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub fn set_mnu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub const fn mnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub fn set_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Alarm A minutes mask"]
    #[inline(always)]
    pub const fn msk2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A minutes mask"]
    #[inline(always)]
    pub fn set_msk2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub const fn ht(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub fn set_ht(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Alarm A hours mask"]
    #[inline(always)]
    pub const fn msk3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A hours mask"]
    #[inline(always)]
    pub fn set_msk3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Date units or day in BCD format"]
    #[inline(always)]
    pub const fn du(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Date units or day in BCD format"]
    #[inline(always)]
    pub fn set_du(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Week day selection"]
    #[inline(always)]
    pub const fn wdsel(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Week day selection"]
    #[inline(always)]
    pub fn set_wdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Alarm A date mask"]
    #[inline(always)]
    pub const fn msk4(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A date mask"]
    #[inline(always)]
    pub fn set_msk4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Alrmar {
    #[inline(always)]
    fn default() -> Alrmar {
        Alrmar(0)
    }
}
#[doc = "alarm A sub second register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alrmassr(pub u32);
impl Alrmassr {
    #[doc = "Sub seconds value"]
    #[inline(always)]
    pub const fn ss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Sub seconds value"]
    #[inline(always)]
    pub fn set_ss(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub const fn maskss(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn set_maskss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Alrmassr {
    #[inline(always)]
    fn default() -> Alrmassr {
        Alrmassr(0)
    }
}
#[doc = "alarm B register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alrmbr(pub u32);
impl Alrmbr {
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub const fn su(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub fn set_su(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Alarm B seconds mask"]
    #[inline(always)]
    pub const fn msk1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B seconds mask"]
    #[inline(always)]
    pub fn set_msk1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub const fn mnu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub fn set_mnu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub const fn mnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub fn set_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Alarm B minutes mask"]
    #[inline(always)]
    pub const fn msk2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B minutes mask"]
    #[inline(always)]
    pub fn set_msk2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub const fn ht(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub fn set_ht(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Alarm B hours mask"]
    #[inline(always)]
    pub const fn msk3(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B hours mask"]
    #[inline(always)]
    pub fn set_msk3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Date units or day in BCD format"]
    #[inline(always)]
    pub const fn du(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Date units or day in BCD format"]
    #[inline(always)]
    pub fn set_du(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Week day selection"]
    #[inline(always)]
    pub const fn wdsel(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Week day selection"]
    #[inline(always)]
    pub fn set_wdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Alarm B date mask"]
    #[inline(always)]
    pub const fn msk4(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B date mask"]
    #[inline(always)]
    pub fn set_msk4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Alrmbr {
    #[inline(always)]
    fn default() -> Alrmbr {
        Alrmbr(0)
    }
}
#[doc = "alarm B sub second register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alrmbssr(pub u32);
impl Alrmbssr {
    #[doc = "Sub seconds value"]
    #[inline(always)]
    pub const fn ss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Sub seconds value"]
    #[inline(always)]
    pub fn set_ss(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub const fn maskss(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn set_maskss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Alrmbssr {
    #[inline(always)]
    fn default() -> Alrmbssr {
        Alrmbssr(0)
    }
}
#[doc = "calibration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calr(pub u32);
impl Calr {
    #[doc = "Calibration minus"]
    #[inline(always)]
    pub const fn calm(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Calibration minus"]
    #[inline(always)]
    pub fn set_calm(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub const fn calw16(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub fn set_calw16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub const fn calw8(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub fn set_calw8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub const fn calp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub fn set_calp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Calr {
    #[inline(always)]
    fn default() -> Calr {
        Calr(0)
    }
}
#[doc = "control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Wakeup clock selection"]
    #[inline(always)]
    pub const fn wcksel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Wakeup clock selection"]
    #[inline(always)]
    pub fn set_wcksel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Time-stamp event active edge"]
    #[inline(always)]
    pub const fn tsedge(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Time-stamp event active edge"]
    #[inline(always)]
    pub fn set_tsedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub const fn refckon(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn set_refckon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bypass the shadow registers"]
    #[inline(always)]
    pub const fn bypshad(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the shadow registers"]
    #[inline(always)]
    pub fn set_bypshad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Hour format"]
    #[inline(always)]
    pub const fn fmt(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Hour format"]
    #[inline(always)]
    pub fn set_fmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Alarm A enable"]
    #[inline(always)]
    pub const fn alrae(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A enable"]
    #[inline(always)]
    pub fn set_alrae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Alarm B enable"]
    #[inline(always)]
    pub const fn alrbe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B enable"]
    #[inline(always)]
    pub fn set_alrbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Wakeup timer enable"]
    #[inline(always)]
    pub const fn wute(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup timer enable"]
    #[inline(always)]
    pub fn set_wute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Time stamp enable"]
    #[inline(always)]
    pub const fn tse(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Time stamp enable"]
    #[inline(always)]
    pub fn set_tse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Alarm A interrupt enable"]
    #[inline(always)]
    pub const fn alraie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A interrupt enable"]
    #[inline(always)]
    pub fn set_alraie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Alarm B interrupt enable"]
    #[inline(always)]
    pub const fn alrbie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B interrupt enable"]
    #[inline(always)]
    pub fn set_alrbie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Wakeup timer interrupt enable"]
    #[inline(always)]
    pub const fn wutie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn set_wutie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Time-stamp interrupt enable"]
    #[inline(always)]
    pub const fn tsie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn set_tsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Add 1 hour (summer time change)"]
    #[inline(always)]
    pub const fn add1h(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn set_add1h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub const fn sub1h(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn set_sub1h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Backup"]
    #[inline(always)]
    pub const fn bkp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Backup"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Calibration output selection"]
    #[inline(always)]
    pub const fn cosel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration output selection"]
    #[inline(always)]
    pub fn set_cosel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Output polarity"]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Output polarity"]
    #[inline(always)]
    pub fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Output selection"]
    #[inline(always)]
    pub const fn osel(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Output selection"]
    #[inline(always)]
    pub fn set_osel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "Calibration output enable"]
    #[inline(always)]
    pub const fn coe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Calibration output enable"]
    #[inline(always)]
    pub fn set_coe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "timestamp on internal event enable"]
    #[inline(always)]
    pub const fn itse(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "timestamp on internal event enable"]
    #[inline(always)]
    pub fn set_itse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TAMPTS"]
    #[inline(always)]
    pub const fn tampts(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TAMPTS"]
    #[inline(always)]
    pub fn set_tampts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TAMPOE"]
    #[inline(always)]
    pub const fn tampoe(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TAMPOE"]
    #[inline(always)]
    pub fn set_tampoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "TAMPALRM_PU"]
    #[inline(always)]
    pub const fn tampalrm_pu(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TAMPALRM_PU"]
    #[inline(always)]
    pub fn set_tampalrm_pu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TAMPALRM_TYPE"]
    #[inline(always)]
    pub const fn tampalrm_type(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "TAMPALRM_TYPE"]
    #[inline(always)]
    pub fn set_tampalrm_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "OUT2EN"]
    #[inline(always)]
    pub const fn out2en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "OUT2EN"]
    #[inline(always)]
    pub fn set_out2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "date register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub const fn du(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub fn set_du(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub const fn mu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub fn set_mu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub const fn mt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub fn set_mt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Week day units"]
    #[inline(always)]
    pub const fn wdu(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Week day units"]
    #[inline(always)]
    pub fn set_wdu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Year units in BCD format"]
    #[inline(always)]
    pub const fn yu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Year units in BCD format"]
    #[inline(always)]
    pub fn set_yu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Year tens in BCD format"]
    #[inline(always)]
    pub const fn yt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Year tens in BCD format"]
    #[inline(always)]
    pub fn set_yt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
#[doc = "initialization and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icsr(pub u32);
impl Icsr {
    #[doc = "Alarm A write flag"]
    #[inline(always)]
    pub const fn alrawf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm A write flag"]
    #[inline(always)]
    pub fn set_alrawf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Alarm B write flag"]
    #[inline(always)]
    pub const fn alrbwf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm B write flag"]
    #[inline(always)]
    pub fn set_alrbwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wakeup timer write flag"]
    #[inline(always)]
    pub const fn wutwf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup timer write flag"]
    #[inline(always)]
    pub fn set_wutwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shift operation pending"]
    #[inline(always)]
    pub const fn shpf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shift operation pending"]
    #[inline(always)]
    pub fn set_shpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Initialization status flag"]
    #[inline(always)]
    pub const fn inits(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization status flag"]
    #[inline(always)]
    pub fn set_inits(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Registers synchronization flag"]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Registers synchronization flag"]
    #[inline(always)]
    pub fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Initialization flag"]
    #[inline(always)]
    pub const fn initf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization flag"]
    #[inline(always)]
    pub fn set_initf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Initialization mode"]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization mode"]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Recalibration pending Flag"]
    #[inline(always)]
    pub const fn recalpf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Recalibration pending Flag"]
    #[inline(always)]
    pub fn set_recalpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Icsr {
    #[inline(always)]
    fn default() -> Icsr {
        Icsr(0)
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misr(pub u32);
impl Misr {
    #[doc = "ALRAMF"]
    #[inline(always)]
    pub const fn alramf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ALRAMF"]
    #[inline(always)]
    pub fn set_alramf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ALRBMF"]
    #[inline(always)]
    pub const fn alrbmf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ALRBMF"]
    #[inline(always)]
    pub fn set_alrbmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WUTMF"]
    #[inline(always)]
    pub const fn wutmf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WUTMF"]
    #[inline(always)]
    pub fn set_wutmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TSMF"]
    #[inline(always)]
    pub const fn tsmf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TSMF"]
    #[inline(always)]
    pub fn set_tsmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TSOVMF"]
    #[inline(always)]
    pub const fn tsovmf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TSOVMF"]
    #[inline(always)]
    pub fn set_tsovmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ITSMF"]
    #[inline(always)]
    pub const fn itsmf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ITSMF"]
    #[inline(always)]
    pub fn set_itsmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Misr {
    #[inline(always)]
    fn default() -> Misr {
        Misr(0)
    }
}
#[doc = "prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prer(pub u32);
impl Prer {
    #[doc = "Synchronous prescaler factor"]
    #[inline(always)]
    pub const fn prediv_s(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Synchronous prescaler factor"]
    #[inline(always)]
    pub fn set_prediv_s(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Asynchronous prescaler factor"]
    #[inline(always)]
    pub const fn prediv_a(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn set_prediv_a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Prer {
    #[inline(always)]
    fn default() -> Prer {
        Prer(0)
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "CALRAF"]
    #[inline(always)]
    pub const fn calraf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CALRAF"]
    #[inline(always)]
    pub fn set_calraf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CALRBF"]
    #[inline(always)]
    pub const fn calrbf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CALRBF"]
    #[inline(always)]
    pub fn set_calrbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CWUTF"]
    #[inline(always)]
    pub const fn cwutf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CWUTF"]
    #[inline(always)]
    pub fn set_cwutf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTSF"]
    #[inline(always)]
    pub const fn ctsf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTSF"]
    #[inline(always)]
    pub fn set_ctsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CTSOVF"]
    #[inline(always)]
    pub const fn ctsovf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CTSOVF"]
    #[inline(always)]
    pub fn set_ctsovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CITSF"]
    #[inline(always)]
    pub const fn citsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CITSF"]
    #[inline(always)]
    pub fn set_citsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
#[doc = "shift control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftr(pub u32);
impl Shiftr {
    #[doc = "Subtract a fraction of a second"]
    #[inline(always)]
    pub const fn subfs(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Subtract a fraction of a second"]
    #[inline(always)]
    pub fn set_subfs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Add one second"]
    #[inline(always)]
    pub const fn add1s(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Add one second"]
    #[inline(always)]
    pub fn set_add1s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Shiftr {
    #[inline(always)]
    fn default() -> Shiftr {
        Shiftr(0)
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "ALRAF"]
    #[inline(always)]
    pub const fn alraf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ALRAF"]
    #[inline(always)]
    pub fn set_alraf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ALRBF"]
    #[inline(always)]
    pub const fn alrbf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ALRBF"]
    #[inline(always)]
    pub fn set_alrbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WUTF"]
    #[inline(always)]
    pub const fn wutf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WUTF"]
    #[inline(always)]
    pub fn set_wutf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TSF"]
    #[inline(always)]
    pub const fn tsf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TSF"]
    #[inline(always)]
    pub fn set_tsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TSOVF"]
    #[inline(always)]
    pub const fn tsovf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TSOVF"]
    #[inline(always)]
    pub fn set_tsovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ITSF"]
    #[inline(always)]
    pub const fn itsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ITSF"]
    #[inline(always)]
    pub fn set_itsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "sub second register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr(pub u32);
impl Ssr {
    #[doc = "Sub second value"]
    #[inline(always)]
    pub const fn ss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sub second value"]
    #[inline(always)]
    pub fn set_ss(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        Ssr(0)
    }
}
#[doc = "time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr(pub u32);
impl Tr {
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub const fn su(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub fn set_su(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub const fn mnu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub fn set_mnu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub const fn mnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub fn set_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub const fn ht(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub fn set_ht(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Tr {
    #[inline(always)]
    fn default() -> Tr {
        Tr(0)
    }
}
#[doc = "time stamp date register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsdr(pub u32);
impl Tsdr {
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub const fn du(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub fn set_du(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub const fn mu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub fn set_mu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub const fn mt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub fn set_mt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Week day units"]
    #[inline(always)]
    pub const fn wdu(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Week day units"]
    #[inline(always)]
    pub fn set_wdu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
}
impl Default for Tsdr {
    #[inline(always)]
    fn default() -> Tsdr {
        Tsdr(0)
    }
}
#[doc = "timestamp sub second register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsssr(pub u32);
impl Tsssr {
    #[doc = "Sub second value"]
    #[inline(always)]
    pub const fn ss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sub second value"]
    #[inline(always)]
    pub fn set_ss(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tsssr {
    #[inline(always)]
    fn default() -> Tsssr {
        Tsssr(0)
    }
}
#[doc = "time stamp time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstr(pub u32);
impl Tstr {
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub const fn su(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub fn set_su(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub const fn mnu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub fn set_mnu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub const fn mnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub fn set_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub const fn ht(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub fn set_ht(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "AM/PM notation"]
    #[inline(always)]
    pub fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Tstr {
    #[inline(always)]
    fn default() -> Tstr {
        Tstr(0)
    }
}
#[doc = "write protection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wpr(pub u32);
impl Wpr {
    #[doc = "Write protection key"]
    #[inline(always)]
    pub const fn key(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write protection key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Wpr {
    #[inline(always)]
    fn default() -> Wpr {
        Wpr(0)
    }
}
#[doc = "wakeup timer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wutr(pub u32);
impl Wutr {
    #[doc = "Wakeup auto-reload value bits"]
    #[inline(always)]
    pub const fn wut(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Wakeup auto-reload value bits"]
    #[inline(always)]
    pub fn set_wut(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Wutr {
    #[inline(always)]
    fn default() -> Wutr {
        Wutr(0)
    }
}
