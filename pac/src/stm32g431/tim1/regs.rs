#[doc = "TIM alternate function option register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Af1(pub u32);
impl Af1 {
    #[doc = "BRK BKIN input enable"]
    #[inline(always)]
    pub const fn bkine(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BRK BKIN input enable"]
    #[inline(always)]
    pub fn set_bkine(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BRK COMP1 enable"]
    #[inline(always)]
    pub const fn bkcmp1e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP1 enable"]
    #[inline(always)]
    pub fn set_bkcmp1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BRK COMP2 enable"]
    #[inline(always)]
    pub const fn bkcmp2e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP2 enable"]
    #[inline(always)]
    pub fn set_bkcmp2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BRK COMP3 enable"]
    #[inline(always)]
    pub const fn bkcmp3e(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP3 enable"]
    #[inline(always)]
    pub fn set_bkcmp3e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "BRK COMP4 enable"]
    #[inline(always)]
    pub const fn bkcmp4e(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP4 enable"]
    #[inline(always)]
    pub fn set_bkcmp4e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BRK COMP5 enable"]
    #[inline(always)]
    pub const fn bkcmp5e(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP5 enable"]
    #[inline(always)]
    pub fn set_bkcmp5e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "BRK COMP6 enable"]
    #[inline(always)]
    pub const fn bkcmp6e(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP6 enable"]
    #[inline(always)]
    pub fn set_bkcmp6e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "BRK COMP7 enable"]
    #[inline(always)]
    pub const fn bkcmp7e(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP7 enable"]
    #[inline(always)]
    pub fn set_bkcmp7e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "BRK BKIN input polarity"]
    #[inline(always)]
    pub const fn bkinp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "BRK BKIN input polarity"]
    #[inline(always)]
    pub fn set_bkinp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "BRK COMP1 input polarity"]
    #[inline(always)]
    pub const fn bkcmp1p(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn set_bkcmp1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "BRK COMP2 input polarity"]
    #[inline(always)]
    pub const fn bkcmp2p(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn set_bkcmp2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BRK COMP3 input polarity"]
    #[inline(always)]
    pub const fn bkcmp3p(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP3 input polarity"]
    #[inline(always)]
    pub fn set_bkcmp3p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "BRK COMP4 input polarity"]
    #[inline(always)]
    pub const fn bkcmp4p(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "BRK COMP4 input polarity"]
    #[inline(always)]
    pub fn set_bkcmp4p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ETR source selection"]
    #[inline(always)]
    pub const fn etrsel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x0f;
        val as u8
    }
    #[doc = "ETR source selection"]
    #[inline(always)]
    pub fn set_etrsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
    }
}
impl Default for Af1 {
    #[inline(always)]
    fn default() -> Af1 {
        Af1(0)
    }
}
#[doc = "TIM alternate function option register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Af2(pub u32);
impl Af2 {
    #[doc = "BRK BKIN input enable"]
    #[inline(always)]
    pub const fn bkine(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BRK BKIN input enable"]
    #[inline(always)]
    pub fn set_bkine(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BRK2 COMP1 enable"]
    #[inline(always)]
    pub const fn bk2cmp1e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn set_bk2cmp1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BRK2 COMP2 enable"]
    #[inline(always)]
    pub const fn bk2cmp2e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn set_bk2cmp2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BRK2 COMP3 enable"]
    #[inline(always)]
    pub const fn bk2cmp3e(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn set_bk2cmp3e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "BRK2 COMP4 enable"]
    #[inline(always)]
    pub const fn bk2cmp4e(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn set_bk2cmp4e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BRK2 COMP5 enable"]
    #[inline(always)]
    pub const fn bk2cmp5e(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn set_bk2cmp5e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "BRK2 COMP6 enable"]
    #[inline(always)]
    pub const fn bk2cmp6e(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn set_bk2cmp6e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "BRK2 COMP7 enable"]
    #[inline(always)]
    pub const fn bk2cmp7e(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn set_bk2cmp7e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "BRK2 BKIN input polarity"]
    #[inline(always)]
    pub const fn bk2inp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn set_bk2inp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub const fn bk2cmp1p(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn set_bk2cmp1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub const fn bk2cmp2p(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn set_bk2cmp2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub const fn bk2cmp3p(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn set_bk2cmp3p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub const fn bk2cmp4p(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn set_bk2cmp4p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "OCREF_CLR source selection"]
    #[inline(always)]
    pub const fn ocrsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "OCREF_CLR source selection"]
    #[inline(always)]
    pub fn set_ocrsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Af2 {
    #[inline(always)]
    fn default() -> Af2 {
        Af2(0)
    }
}
#[doc = "auto-reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "Auto-reload value"]
    #[inline(always)]
    pub const fn arr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Auto-reload value"]
    #[inline(always)]
    pub fn set_arr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Arr {
    #[inline(always)]
    fn default() -> Arr {
        Arr(0)
    }
}
#[doc = "break and dead-time register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtr(pub u32);
impl Bdtr {
    #[doc = "Dead-time generator setup"]
    #[inline(always)]
    pub const fn dtg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Dead-time generator setup"]
    #[inline(always)]
    pub fn set_dtg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Lock configuration"]
    #[inline(always)]
    pub const fn lock(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Lock configuration"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Off-state selection for Idle mode"]
    #[inline(always)]
    pub const fn ossi(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn set_ossi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Off-state selection for Run mode"]
    #[inline(always)]
    pub const fn ossr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Off-state selection for Run mode"]
    #[inline(always)]
    pub fn set_ossr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Break enable"]
    #[inline(always)]
    pub const fn bke(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Break enable"]
    #[inline(always)]
    pub fn set_bke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Break polarity"]
    #[inline(always)]
    pub const fn bkp(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Break polarity"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Automatic output enable"]
    #[inline(always)]
    pub const fn aoe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic output enable"]
    #[inline(always)]
    pub fn set_aoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Main output enable"]
    #[inline(always)]
    pub const fn moe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Main output enable"]
    #[inline(always)]
    pub fn set_moe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Break filter"]
    #[inline(always)]
    pub const fn bkf(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Break filter"]
    #[inline(always)]
    pub fn set_bkf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Break 2 filter"]
    #[inline(always)]
    pub const fn bk2f(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Break 2 filter"]
    #[inline(always)]
    pub fn set_bk2f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Break 2 Enable"]
    #[inline(always)]
    pub const fn bk2e(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 Enable"]
    #[inline(always)]
    pub fn set_bk2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Break 2 polarity"]
    #[inline(always)]
    pub const fn bk2p(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 polarity"]
    #[inline(always)]
    pub fn set_bk2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "BKDSRM"]
    #[inline(always)]
    pub const fn bkdsrm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "BKDSRM"]
    #[inline(always)]
    pub fn set_bkdsrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "BK2DSRM"]
    #[inline(always)]
    pub const fn bk2dsrm(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "BK2DSRM"]
    #[inline(always)]
    pub fn set_bk2dsrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "BKBID"]
    #[inline(always)]
    pub const fn bkbid(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "BKBID"]
    #[inline(always)]
    pub fn set_bkbid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "BK2ID"]
    #[inline(always)]
    pub const fn bk2id(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "BK2ID"]
    #[inline(always)]
    pub fn set_bk2id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Bdtr {
    #[inline(always)]
    fn default() -> Bdtr {
        Bdtr(0)
    }
}
#[doc = "capture/compare enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccer(pub u32);
impl Ccer {
    #[doc = "Capture/Compare 1 output enable"]
    #[inline(always)]
    pub const fn cc1e(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn set_cc1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub const fn cc1p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn set_cc1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub const fn cc1ne(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn set_cc1ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub const fn cc1np(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn set_cc1np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 2 output enable"]
    #[inline(always)]
    pub const fn cc2e(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn set_cc2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub const fn cc2p(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn set_cc2p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub const fn cc2ne(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn set_cc2ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub const fn cc2np(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn set_cc2np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 3 output enable"]
    #[inline(always)]
    pub const fn cc3e(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn set_cc3e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub const fn cc3p(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn set_cc3p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub const fn cc3ne(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn set_cc3ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub const fn cc3np(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn set_cc3np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture/Compare 4 output enable"]
    #[inline(always)]
    pub const fn cc4e(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn set_cc4e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub const fn cc4p(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn set_cc4p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Capture/Compare 4 complementary output enable"]
    #[inline(always)]
    pub const fn cc4ne(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 complementary output enable"]
    #[inline(always)]
    pub fn set_cc4ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Capture/Compare 4 complementary output polarity"]
    #[inline(always)]
    pub const fn cc4np(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 complementary output polarity"]
    #[inline(always)]
    pub fn set_cc4np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Capture/Compare 5 output enable"]
    #[inline(always)]
    pub const fn cc5e(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 output enable"]
    #[inline(always)]
    pub fn set_cc5e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Capture/Compare 5 output polarity"]
    #[inline(always)]
    pub const fn cc5p(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 5 output polarity"]
    #[inline(always)]
    pub fn set_cc5p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Capture/Compare 6 output enable"]
    #[inline(always)]
    pub const fn cc6e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 6 output enable"]
    #[inline(always)]
    pub fn set_cc6e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Capture/Compare 6 output polarity"]
    #[inline(always)]
    pub const fn cc6p(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 6 output polarity"]
    #[inline(always)]
    pub fn set_cc6p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ccer {
    #[inline(always)]
    fn default() -> Ccer {
        Ccer(0)
    }
}
#[doc = "capture/compare mode register 1 (input mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1Input(pub u32);
impl Ccmr1Input {
    #[doc = "Capture/Compare 1 selection"]
    #[inline(always)]
    pub const fn cc1s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn set_cc1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input capture 1 prescaler"]
    #[inline(always)]
    pub const fn ic1psc(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 prescaler"]
    #[inline(always)]
    pub fn set_ic1psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Input capture 1 filter"]
    #[inline(always)]
    pub const fn ic1f(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 1 filter"]
    #[inline(always)]
    pub fn set_ic1f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Capture/Compare 2 selection"]
    #[inline(always)]
    pub const fn cc2s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn set_cc2s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Input capture 2 prescaler"]
    #[inline(always)]
    pub const fn ic2psc(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 2 prescaler"]
    #[inline(always)]
    pub fn set_ic2psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Input capture 2 filter"]
    #[inline(always)]
    pub const fn ic2f(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 2 filter"]
    #[inline(always)]
    pub fn set_ic2f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Ccmr1Input {
    #[inline(always)]
    fn default() -> Ccmr1Input {
        Ccmr1Input(0)
    }
}
#[doc = "capture/compare mode register 1 (output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
    #[doc = "Capture/Compare 1 selection"]
    #[inline(always)]
    pub const fn cc1s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn set_cc1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output Compare 1 fast enable"]
    #[inline(always)]
    pub const fn oc1fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn set_oc1fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output Compare 1 preload enable"]
    #[inline(always)]
    pub const fn oc1pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn set_oc1pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output Compare 1 mode"]
    #[inline(always)]
    pub const fn oc1m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output Compare 1 mode"]
    #[inline(always)]
    pub fn set_oc1m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Output Compare 1 clear enable"]
    #[inline(always)]
    pub const fn oc1ce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn set_oc1ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 2 selection"]
    #[inline(always)]
    pub const fn cc2s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn set_cc2s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Output Compare 2 fast enable"]
    #[inline(always)]
    pub const fn oc2fe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 2 fast enable"]
    #[inline(always)]
    pub fn set_oc2fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output Compare 2 preload enable"]
    #[inline(always)]
    pub const fn oc2pe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 2 preload enable"]
    #[inline(always)]
    pub fn set_oc2pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output Compare 2 mode"]
    #[inline(always)]
    pub const fn oc2m(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Output Compare 2 mode"]
    #[inline(always)]
    pub fn set_oc2m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Output Compare 2 clear enable"]
    #[inline(always)]
    pub const fn oc2ce(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 2 clear enable"]
    #[inline(always)]
    pub fn set_oc2ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub const fn oc1m_3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn set_oc1m_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub const fn oc2m_3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 2 mode - bit 3"]
    #[inline(always)]
    pub fn set_oc2m_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccmr1Output {
    #[inline(always)]
    fn default() -> Ccmr1Output {
        Ccmr1Output(0)
    }
}
#[doc = "capture/compare mode register 2 (input mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr2Input(pub u32);
impl Ccmr2Input {
    #[doc = "Capture/compare 3 selection"]
    #[inline(always)]
    pub const fn cc3s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/compare 3 selection"]
    #[inline(always)]
    pub fn set_cc3s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input capture 3 prescaler"]
    #[inline(always)]
    pub const fn ic3psc(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 3 prescaler"]
    #[inline(always)]
    pub fn set_ic3psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Input capture 3 filter"]
    #[inline(always)]
    pub const fn ic3f(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 3 filter"]
    #[inline(always)]
    pub fn set_ic3f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Capture/Compare 4 selection"]
    #[inline(always)]
    pub const fn cc4s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn set_cc4s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Input capture 4 prescaler"]
    #[inline(always)]
    pub const fn ic4psc(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 4 prescaler"]
    #[inline(always)]
    pub fn set_ic4psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Input capture 4 filter"]
    #[inline(always)]
    pub const fn ic4f(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 4 filter"]
    #[inline(always)]
    pub fn set_ic4f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Ccmr2Input {
    #[inline(always)]
    fn default() -> Ccmr2Input {
        Ccmr2Input(0)
    }
}
#[doc = "capture/compare mode register 2 (output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr2Output(pub u32);
impl Ccmr2Output {
    #[doc = "Capture/Compare 3 selection"]
    #[inline(always)]
    pub const fn cc3s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn set_cc3s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output compare 3 fast enable"]
    #[inline(always)]
    pub const fn oc3fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 fast enable"]
    #[inline(always)]
    pub fn set_oc3fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output compare 3 preload enable"]
    #[inline(always)]
    pub const fn oc3pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 preload enable"]
    #[inline(always)]
    pub fn set_oc3pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output compare 3 mode"]
    #[inline(always)]
    pub const fn oc3m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 3 mode"]
    #[inline(always)]
    pub fn set_oc3m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Output compare 3 clear enable"]
    #[inline(always)]
    pub const fn oc3ce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 3 clear enable"]
    #[inline(always)]
    pub fn set_oc3ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Capture/Compare 4 selection"]
    #[inline(always)]
    pub const fn cc4s(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn set_cc4s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Output compare 4 fast enable"]
    #[inline(always)]
    pub const fn oc4fe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 fast enable"]
    #[inline(always)]
    pub fn set_oc4fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output compare 4 preload enable"]
    #[inline(always)]
    pub const fn oc4pe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 preload enable"]
    #[inline(always)]
    pub fn set_oc4pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output compare 4 mode"]
    #[inline(always)]
    pub const fn oc4m(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 4 mode"]
    #[inline(always)]
    pub fn set_oc4m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Output compare 4 clear enable"]
    #[inline(always)]
    pub const fn oc4ce(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 4 clear enable"]
    #[inline(always)]
    pub fn set_oc4ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Output Compare 3 mode - bit 3"]
    #[inline(always)]
    pub const fn oc3m_3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 3 mode - bit 3"]
    #[inline(always)]
    pub fn set_oc3m_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Output Compare 4 mode - bit 3"]
    #[inline(always)]
    pub const fn oc4m_3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 4 mode - bit 3"]
    #[inline(always)]
    pub fn set_oc4m_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccmr2Output {
    #[inline(always)]
    fn default() -> Ccmr2Output {
        Ccmr2Output(0)
    }
}
#[doc = "capture/compare mode register 2 (output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr3Output(pub u32);
impl Ccmr3Output {
    #[doc = "Output compare 5 fast enable"]
    #[inline(always)]
    pub const fn oc5fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 fast enable"]
    #[inline(always)]
    pub fn set_oc5fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output compare 5 preload enable"]
    #[inline(always)]
    pub const fn oc5pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 preload enable"]
    #[inline(always)]
    pub fn set_oc5pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output compare 5 mode"]
    #[inline(always)]
    pub const fn oc5m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 5 mode"]
    #[inline(always)]
    pub fn set_oc5m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Output compare 5 clear enable"]
    #[inline(always)]
    pub const fn oc5ce(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 5 clear enable"]
    #[inline(always)]
    pub fn set_oc5ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Output compare 6 fast enable"]
    #[inline(always)]
    pub const fn oc6fe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 6 fast enable"]
    #[inline(always)]
    pub fn set_oc6fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output compare 6 preload enable"]
    #[inline(always)]
    pub const fn oc6pe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 6 preload enable"]
    #[inline(always)]
    pub fn set_oc6pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output compare 6 mode"]
    #[inline(always)]
    pub const fn oc6m(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 6 mode"]
    #[inline(always)]
    pub fn set_oc6m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Output compare 6 clear enable"]
    #[inline(always)]
    pub const fn oc6ce(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 6 clear enable"]
    #[inline(always)]
    pub fn set_oc6ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub const fn oc5m_bit3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Output Compare 5 mode bit 3"]
    #[inline(always)]
    pub fn set_oc5m_bit3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub const fn oc6m_bit3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Output Compare 6 mode bit 3"]
    #[inline(always)]
    pub fn set_oc6m_bit3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ccmr3Output {
    #[inline(always)]
    fn default() -> Ccmr3Output {
        Ccmr3Output(0)
    }
}
#[doc = "capture/compare register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Capture/Compare 1 value"]
    #[inline(always)]
    pub const fn ccr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Capture/Compare 1 value"]
    #[inline(always)]
    pub fn set_ccr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
#[doc = "capture/compare register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "Capture/Compare 2 value"]
    #[inline(always)]
    pub const fn ccr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Capture/Compare 2 value"]
    #[inline(always)]
    pub fn set_ccr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        Ccr2(0)
    }
}
#[doc = "capture/compare register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub const fn ccr3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub fn set_ccr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        Ccr3(0)
    }
}
#[doc = "capture/compare register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub const fn ccr4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub fn set_ccr4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        Ccr4(0)
    }
}
#[doc = "capture/compare register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub const fn ccr5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub fn set_ccr5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Group Channel 5 and Channel 1"]
    #[inline(always)]
    pub const fn gc5c1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Group Channel 5 and Channel 1"]
    #[inline(always)]
    pub fn set_gc5c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Group Channel 5 and Channel 2"]
    #[inline(always)]
    pub const fn gc5c2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Group Channel 5 and Channel 2"]
    #[inline(always)]
    pub fn set_gc5c2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Group Channel 5 and Channel 3"]
    #[inline(always)]
    pub const fn gc5c3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Group Channel 5 and Channel 3"]
    #[inline(always)]
    pub fn set_gc5c3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ccr5 {
    #[inline(always)]
    fn default() -> Ccr5 {
        Ccr5(0)
    }
}
#[doc = "capture/compare register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr6(pub u32);
impl Ccr6 {
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub const fn ccr6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Capture/Compare value"]
    #[inline(always)]
    pub fn set_ccr6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Ccr6 {
    #[inline(always)]
    fn default() -> Ccr6 {
        Ccr6(0)
    }
}
#[doc = "counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "counter value"]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "counter value"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "UIFCPY"]
    #[inline(always)]
    pub const fn uifcpy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "UIFCPY"]
    #[inline(always)]
    pub fn set_uifcpy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
#[doc = "control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Counter enable"]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable"]
    #[inline(always)]
    pub fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Update disable"]
    #[inline(always)]
    pub const fn udis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update disable"]
    #[inline(always)]
    pub fn set_udis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Update request source"]
    #[inline(always)]
    pub const fn urs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Update request source"]
    #[inline(always)]
    pub fn set_urs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "One-pulse mode"]
    #[inline(always)]
    pub const fn opm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "One-pulse mode"]
    #[inline(always)]
    pub fn set_opm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Center-aligned mode selection"]
    #[inline(always)]
    pub const fn cms(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Center-aligned mode selection"]
    #[inline(always)]
    pub fn set_cms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Auto-reload preload enable"]
    #[inline(always)]
    pub const fn arpe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-reload preload enable"]
    #[inline(always)]
    pub fn set_arpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clock division"]
    #[inline(always)]
    pub const fn ckd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Clock division"]
    #[inline(always)]
    pub fn set_ckd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "UIF status bit remapping"]
    #[inline(always)]
    pub const fn uifremap(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "UIF status bit remapping"]
    #[inline(always)]
    pub fn set_uifremap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Dithering Enable"]
    #[inline(always)]
    pub const fn dithen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Dithering Enable"]
    #[inline(always)]
    pub fn set_dithen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
#[doc = "control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "Capture/compare preloaded control"]
    #[inline(always)]
    pub const fn ccpc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare preloaded control"]
    #[inline(always)]
    pub fn set_ccpc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare control update selection"]
    #[inline(always)]
    pub const fn ccus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare control update selection"]
    #[inline(always)]
    pub fn set_ccus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/compare DMA selection"]
    #[inline(always)]
    pub const fn ccds(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare DMA selection"]
    #[inline(always)]
    pub fn set_ccds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Master mode selection"]
    #[inline(always)]
    pub const fn mms(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Master mode selection"]
    #[inline(always)]
    pub fn set_mms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "TI1 selection"]
    #[inline(always)]
    pub const fn ti1s(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TI1 selection"]
    #[inline(always)]
    pub fn set_ti1s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Output Idle state 1"]
    #[inline(always)]
    pub const fn ois1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 1"]
    #[inline(always)]
    pub fn set_ois1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Output Idle state 1"]
    #[inline(always)]
    pub const fn ois1n(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 1"]
    #[inline(always)]
    pub fn set_ois1n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Output Idle state 2"]
    #[inline(always)]
    pub const fn ois2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 2"]
    #[inline(always)]
    pub fn set_ois2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output Idle state 2"]
    #[inline(always)]
    pub const fn ois2n(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 2"]
    #[inline(always)]
    pub fn set_ois2n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Output Idle state 3"]
    #[inline(always)]
    pub const fn ois3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 3"]
    #[inline(always)]
    pub fn set_ois3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Output Idle state 3"]
    #[inline(always)]
    pub const fn ois3n(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 3"]
    #[inline(always)]
    pub fn set_ois3n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Output Idle state 4"]
    #[inline(always)]
    pub const fn ois4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 4"]
    #[inline(always)]
    pub fn set_ois4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Output Idle state 4 (OC4N output)"]
    #[inline(always)]
    pub const fn ois4n(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 4 (OC4N output)"]
    #[inline(always)]
    pub fn set_ois4n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Output Idle state 5 (OC5 output)"]
    #[inline(always)]
    pub const fn ois5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 5 (OC5 output)"]
    #[inline(always)]
    pub fn set_ois5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Output Idle state 6 (OC6 output)"]
    #[inline(always)]
    pub const fn ois6(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Output Idle state 6 (OC6 output)"]
    #[inline(always)]
    pub fn set_ois6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Master mode selection 2"]
    #[inline(always)]
    pub const fn mms2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Master mode selection 2"]
    #[inline(always)]
    pub fn set_mms2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Master mode selection - bit 3"]
    #[inline(always)]
    pub const fn mms_3(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Master mode selection - bit 3"]
    #[inline(always)]
    pub fn set_mms_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
#[doc = "control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "DMA base address"]
    #[inline(always)]
    pub const fn dba(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA base address"]
    #[inline(always)]
    pub fn set_dba(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "DMA burst length"]
    #[inline(always)]
    pub const fn dbl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA burst length"]
    #[inline(always)]
    pub fn set_dbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        Dcr(0)
    }
}
#[doc = "DMA/Interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc = "Update interrupt enable"]
    #[inline(always)]
    pub const fn uie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt enable"]
    #[inline(always)]
    pub fn set_uie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub const fn cc1ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn set_cc1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub const fn cc2ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn set_cc2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub const fn cc3ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn set_cc3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub const fn cc4ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn set_cc4ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COM interrupt enable"]
    #[inline(always)]
    pub const fn comie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COM interrupt enable"]
    #[inline(always)]
    pub fn set_comie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger interrupt enable"]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt enable"]
    #[inline(always)]
    pub fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break interrupt enable"]
    #[inline(always)]
    pub const fn bie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break interrupt enable"]
    #[inline(always)]
    pub fn set_bie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Update DMA request enable"]
    #[inline(always)]
    pub const fn ude(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Update DMA request enable"]
    #[inline(always)]
    pub fn set_ude(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub const fn cc1de(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn set_cc1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub const fn cc2de(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn set_cc2de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub const fn cc3de(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn set_cc3de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub const fn cc4de(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn set_cc4de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "COM DMA request enable"]
    #[inline(always)]
    pub const fn comde(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "COM DMA request enable"]
    #[inline(always)]
    pub fn set_comde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Trigger DMA request enable"]
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA request enable"]
    #[inline(always)]
    pub fn set_tde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Index interrupt enable"]
    #[inline(always)]
    pub const fn idxie(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Index interrupt enable"]
    #[inline(always)]
    pub fn set_idxie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction Change interrupt enable"]
    #[inline(always)]
    pub const fn dirie(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Direction Change interrupt enable"]
    #[inline(always)]
    pub fn set_dirie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Index Error interrupt enable"]
    #[inline(always)]
    pub const fn ierrie(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Index Error interrupt enable"]
    #[inline(always)]
    pub fn set_ierrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transition Error interrupt enable"]
    #[inline(always)]
    pub const fn terrie(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transition Error interrupt enable"]
    #[inline(always)]
    pub fn set_terrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Dier {
    #[inline(always)]
    fn default() -> Dier {
        Dier(0)
    }
}
#[doc = "DMA address for full transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmar(pub u32);
impl Dmar {
    #[doc = "DMA register for burst accesses"]
    #[inline(always)]
    pub const fn dmab(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DMA register for burst accesses"]
    #[inline(always)]
    pub fn set_dmab(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmar {
    #[inline(always)]
    fn default() -> Dmar {
        Dmar(0)
    }
}
#[doc = "timer Deadtime Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtr2(pub u32);
impl Dtr2 {
    #[doc = "Dead-time falling edge generator setup"]
    #[inline(always)]
    pub const fn dtgf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Dead-time falling edge generator setup"]
    #[inline(always)]
    pub fn set_dtgf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub const fn dtae(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub fn set_dtae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Deadtime Preload Enable"]
    #[inline(always)]
    pub const fn dtpe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Deadtime Preload Enable"]
    #[inline(always)]
    pub fn set_dtpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Dtr2 {
    #[inline(always)]
    fn default() -> Dtr2 {
        Dtr2(0)
    }
}
#[doc = "DMA control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Index Enable"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Index Enable"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Index Direction"]
    #[inline(always)]
    pub const fn idir(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Index Direction"]
    #[inline(always)]
    pub fn set_idir(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Index Blanking"]
    #[inline(always)]
    pub const fn iblk(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Index Blanking"]
    #[inline(always)]
    pub fn set_iblk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "First Index"]
    #[inline(always)]
    pub const fn fidx(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "First Index"]
    #[inline(always)]
    pub fn set_fidx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Index Positioning"]
    #[inline(always)]
    pub const fn ipos(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index Positioning"]
    #[inline(always)]
    pub fn set_ipos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Pulse width"]
    #[inline(always)]
    pub const fn pw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pulse width"]
    #[inline(always)]
    pub fn set_pw(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Pulse Width prescaler"]
    #[inline(always)]
    pub const fn pwprsc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Pulse Width prescaler"]
    #[inline(always)]
    pub fn set_pwprsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
#[doc = "event generation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc = "Update generation"]
    #[inline(always)]
    pub const fn ug(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update generation"]
    #[inline(always)]
    pub fn set_ug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare 1 generation"]
    #[inline(always)]
    pub const fn cc1g(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 generation"]
    #[inline(always)]
    pub fn set_cc1g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/compare 2 generation"]
    #[inline(always)]
    pub const fn cc2g(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 generation"]
    #[inline(always)]
    pub fn set_cc2g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/compare 3 generation"]
    #[inline(always)]
    pub const fn cc3g(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 3 generation"]
    #[inline(always)]
    pub fn set_cc3g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/compare 4 generation"]
    #[inline(always)]
    pub const fn cc4g(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 4 generation"]
    #[inline(always)]
    pub fn set_cc4g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Capture/Compare control update generation"]
    #[inline(always)]
    pub const fn comg(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare control update generation"]
    #[inline(always)]
    pub fn set_comg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger generation"]
    #[inline(always)]
    pub const fn tg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger generation"]
    #[inline(always)]
    pub fn set_tg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break generation"]
    #[inline(always)]
    pub const fn bg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break generation"]
    #[inline(always)]
    pub fn set_bg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Break 2 generation"]
    #[inline(always)]
    pub const fn b2g(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 generation"]
    #[inline(always)]
    pub fn set_b2g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Egr {
    #[inline(always)]
    fn default() -> Egr {
        Egr(0)
    }
}
#[doc = "prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn psc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub fn set_psc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Psc {
    #[inline(always)]
    fn default() -> Psc {
        Psc(0)
    }
}
#[doc = "repetition counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Repetition counter value"]
    #[inline(always)]
    pub const fn rep(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Repetition counter value"]
    #[inline(always)]
    pub fn set_rep(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0)
    }
}
#[doc = "slave mode control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcr(pub u32);
impl Smcr {
    #[doc = "Slave mode selection"]
    #[inline(always)]
    pub const fn sms(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Slave mode selection"]
    #[inline(always)]
    pub fn set_sms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "OCREF clear selection"]
    #[inline(always)]
    pub const fn occs(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OCREF clear selection"]
    #[inline(always)]
    pub fn set_occs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger selection"]
    #[inline(always)]
    pub const fn ts(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Trigger selection"]
    #[inline(always)]
    pub fn set_ts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Master/Slave mode"]
    #[inline(always)]
    pub const fn msm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Master/Slave mode"]
    #[inline(always)]
    pub fn set_msm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "External trigger filter"]
    #[inline(always)]
    pub const fn etf(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "External trigger filter"]
    #[inline(always)]
    pub fn set_etf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "External trigger prescaler"]
    #[inline(always)]
    pub const fn etps(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "External trigger prescaler"]
    #[inline(always)]
    pub fn set_etps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "External clock enable"]
    #[inline(always)]
    pub const fn ece(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External clock enable"]
    #[inline(always)]
    pub fn set_ece(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "External trigger polarity"]
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger polarity"]
    #[inline(always)]
    pub fn set_etp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave mode selection - bit 3"]
    #[inline(always)]
    pub const fn sms_3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave mode selection - bit 3"]
    #[inline(always)]
    pub fn set_sms_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Trigger selection - bit 4:3"]
    #[inline(always)]
    pub const fn ts_4_3(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Trigger selection - bit 4:3"]
    #[inline(always)]
    pub fn set_ts_4_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "SMS Preload Enable"]
    #[inline(always)]
    pub const fn smspe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SMS Preload Enable"]
    #[inline(always)]
    pub fn set_smspe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SMS Preload Source"]
    #[inline(always)]
    pub const fn smsps(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SMS Preload Source"]
    #[inline(always)]
    pub fn set_smsps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Smcr {
    #[inline(always)]
    fn default() -> Smcr {
        Smcr(0)
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Update interrupt flag"]
    #[inline(always)]
    pub const fn uif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt flag"]
    #[inline(always)]
    pub fn set_uif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub const fn cc1if(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn set_cc1if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub const fn cc2if(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn set_cc2if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub const fn cc3if(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn set_cc3if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub const fn cc4if(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn set_cc4if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COM interrupt flag"]
    #[inline(always)]
    pub const fn comif(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COM interrupt flag"]
    #[inline(always)]
    pub fn set_comif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger interrupt flag"]
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger interrupt flag"]
    #[inline(always)]
    pub fn set_tif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Break interrupt flag"]
    #[inline(always)]
    pub const fn bif(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Break interrupt flag"]
    #[inline(always)]
    pub fn set_bif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Break 2 interrupt flag"]
    #[inline(always)]
    pub const fn b2if(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Break 2 interrupt flag"]
    #[inline(always)]
    pub fn set_b2if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub const fn cc1of(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn set_cc1of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub const fn cc2of(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn set_cc2of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub const fn cc3of(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn set_cc3of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub const fn cc4of(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn set_cc4of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "System Break interrupt flag"]
    #[inline(always)]
    pub const fn sbif(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "System Break interrupt flag"]
    #[inline(always)]
    pub fn set_sbif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Compare 5 interrupt flag"]
    #[inline(always)]
    pub const fn cc5if(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn set_cc5if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Compare 6 interrupt flag"]
    #[inline(always)]
    pub const fn cc6if(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn set_cc6if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Index interrupt flag"]
    #[inline(always)]
    pub const fn idxf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Index interrupt flag"]
    #[inline(always)]
    pub fn set_idxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction Change interrupt flag"]
    #[inline(always)]
    pub const fn dirf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Direction Change interrupt flag"]
    #[inline(always)]
    pub fn set_dirf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Index Error interrupt flag"]
    #[inline(always)]
    pub const fn ierrf(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Index Error interrupt flag"]
    #[inline(always)]
    pub fn set_ierrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transition Error interrupt flag"]
    #[inline(always)]
    pub const fn terrf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transition Error interrupt flag"]
    #[inline(always)]
    pub fn set_terrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "TIM timer input selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tisel(pub u32);
impl Tisel {
    #[doc = "TI1\\[0\\] to TI1\\[15\\] input selection"]
    #[inline(always)]
    pub const fn ti1sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "TI1\\[0\\] to TI1\\[15\\] input selection"]
    #[inline(always)]
    pub fn set_ti1sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "TI2\\[0\\] to TI2\\[15\\] input selection"]
    #[inline(always)]
    pub const fn ti2sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "TI2\\[0\\] to TI2\\[15\\] input selection"]
    #[inline(always)]
    pub fn set_ti2sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "TI3\\[0\\] to TI3\\[15\\] input selection"]
    #[inline(always)]
    pub const fn ti3sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "TI3\\[0\\] to TI3\\[15\\] input selection"]
    #[inline(always)]
    pub fn set_ti3sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "TI4\\[0\\] to TI4\\[15\\] input selection"]
    #[inline(always)]
    pub const fn ti4sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "TI4\\[0\\] to TI4\\[15\\] input selection"]
    #[inline(always)]
    pub fn set_ti4sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Tisel {
    #[inline(always)]
    fn default() -> Tisel {
        Tisel(0)
    }
}
