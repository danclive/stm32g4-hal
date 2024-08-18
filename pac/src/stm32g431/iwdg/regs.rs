#[doc = "Key register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kr(pub u32);
impl Kr {
    #[doc = "Key value (write only, read 0x0000)"]
    #[inline(always)]
    pub const fn key(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Key value (write only, read 0x0000)"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Kr {
    #[inline(always)]
    fn default() -> Kr {
        Kr(0)
    }
}
#[doc = "Prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescaler divider"]
    #[inline(always)]
    pub const fn pr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Prescaler divider"]
    #[inline(always)]
    pub fn set_pr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
#[doc = "Reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlr(pub u32);
impl Rlr {
    #[doc = "Watchdog counter reload value"]
    #[inline(always)]
    pub const fn rl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Watchdog counter reload value"]
    #[inline(always)]
    pub fn set_rl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Rlr {
    #[inline(always)]
    fn default() -> Rlr {
        Rlr(0)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Watchdog prescaler value update"]
    #[inline(always)]
    pub const fn pvu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog prescaler value update"]
    #[inline(always)]
    pub fn set_pvu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog counter reload value update"]
    #[inline(always)]
    pub const fn rvu(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog counter reload value update"]
    #[inline(always)]
    pub fn set_rvu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Watchdog counter window value update"]
    #[inline(always)]
    pub const fn wvu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog counter window value update"]
    #[inline(always)]
    pub fn set_wvu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "Window register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Winr(pub u32);
impl Winr {
    #[doc = "Watchdog counter window value"]
    #[inline(always)]
    pub const fn win(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Watchdog counter window value"]
    #[inline(always)]
    pub fn set_win(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Winr {
    #[inline(always)]
    fn default() -> Winr {
        Winr(0)
    }
}
