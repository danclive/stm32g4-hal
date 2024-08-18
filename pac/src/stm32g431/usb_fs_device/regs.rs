#[doc = "Buffer table address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btable(pub u32);
impl Btable {
    #[doc = "BTABLE"]
    #[inline(always)]
    pub const fn btable(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "BTABLE"]
    #[inline(always)]
    pub fn set_btable(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
}
impl Default for Btable {
    #[inline(always)]
    fn default() -> Btable {
        Btable(0)
    }
}
#[doc = "USB control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntr(pub u32);
impl Cntr {
    #[doc = "FRES"]
    #[inline(always)]
    pub const fn fres(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FRES"]
    #[inline(always)]
    pub fn set_fres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PDWN"]
    #[inline(always)]
    pub const fn pdwn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PDWN"]
    #[inline(always)]
    pub fn set_pdwn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LP_MODE"]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LP_MODE"]
    #[inline(always)]
    pub fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FSUSP"]
    #[inline(always)]
    pub const fn fsusp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FSUSP"]
    #[inline(always)]
    pub fn set_fsusp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RESUME"]
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RESUME"]
    #[inline(always)]
    pub fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "L1RESUME"]
    #[inline(always)]
    pub const fn l1resume(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "L1RESUME"]
    #[inline(always)]
    pub fn set_l1resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "L1REQM"]
    #[inline(always)]
    pub const fn l1reqm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "L1REQM"]
    #[inline(always)]
    pub fn set_l1reqm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ESOFM"]
    #[inline(always)]
    pub const fn esofm(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ESOFM"]
    #[inline(always)]
    pub fn set_esofm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SOFM"]
    #[inline(always)]
    pub const fn sofm(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SOFM"]
    #[inline(always)]
    pub fn set_sofm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RESETM"]
    #[inline(always)]
    pub const fn resetm(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RESETM"]
    #[inline(always)]
    pub fn set_resetm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SUSPM"]
    #[inline(always)]
    pub const fn suspm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SUSPM"]
    #[inline(always)]
    pub fn set_suspm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "WKUPM"]
    #[inline(always)]
    pub const fn wkupm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "WKUPM"]
    #[inline(always)]
    pub fn set_wkupm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ERRM"]
    #[inline(always)]
    pub const fn errm(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ERRM"]
    #[inline(always)]
    pub fn set_errm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PMAOVRM"]
    #[inline(always)]
    pub const fn pmaovrm(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PMAOVRM"]
    #[inline(always)]
    pub fn set_pmaovrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTRM"]
    #[inline(always)]
    pub const fn ctrm(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTRM"]
    #[inline(always)]
    pub fn set_ctrm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cntr {
    #[inline(always)]
    fn default() -> Cntr {
        Cntr(0)
    }
}
#[doc = "USB device address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daddr(pub u32);
impl Daddr {
    #[doc = "ADD"]
    #[inline(always)]
    pub const fn add(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "ADD"]
    #[inline(always)]
    pub fn set_add(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "EF"]
    #[inline(always)]
    pub const fn ef(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EF"]
    #[inline(always)]
    pub fn set_ef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Daddr {
    #[inline(always)]
    fn default() -> Daddr {
        Daddr(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0r(pub u32);
impl Ep0r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep0r {
    #[inline(always)]
    fn default() -> Ep0r {
        Ep0r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep1r(pub u32);
impl Ep1r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep1r {
    #[inline(always)]
    fn default() -> Ep1r {
        Ep1r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep2r(pub u32);
impl Ep2r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep2r {
    #[inline(always)]
    fn default() -> Ep2r {
        Ep2r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep3r(pub u32);
impl Ep3r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep3r {
    #[inline(always)]
    fn default() -> Ep3r {
        Ep3r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep4r(pub u32);
impl Ep4r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep4r {
    #[inline(always)]
    fn default() -> Ep4r {
        Ep4r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep5r(pub u32);
impl Ep5r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep5r {
    #[inline(always)]
    fn default() -> Ep5r {
        Ep5r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep6r(pub u32);
impl Ep6r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep6r {
    #[inline(always)]
    fn default() -> Ep6r {
        Ep6r(0)
    }
}
#[doc = "USB endpoint n register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep7r(pub u32);
impl Ep7r {
    #[doc = "EA"]
    #[inline(always)]
    pub const fn ea(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn set_ea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub const fn stat_tx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_TX"]
    #[inline(always)]
    pub fn set_stat_tx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub const fn dtog_tx(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_TX"]
    #[inline(always)]
    pub fn set_dtog_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub const fn ctr_tx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_TX"]
    #[inline(always)]
    pub fn set_ctr_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub const fn ep_kind(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "EP_KIND"]
    #[inline(always)]
    pub fn set_ep_kind(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub const fn ep_type(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "EP_TYPE"]
    #[inline(always)]
    pub fn set_ep_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub const fn stat_rx(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "STAT_RX"]
    #[inline(always)]
    pub fn set_stat_rx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub const fn dtog_rx(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DTOG_RX"]
    #[inline(always)]
    pub fn set_dtog_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub const fn ctr_rx(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR_RX"]
    #[inline(always)]
    pub fn set_ctr_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ep7r {
    #[inline(always)]
    fn default() -> Ep7r {
        Ep7r(0)
    }
}
#[doc = "USB frame number register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fnr(pub u32);
impl Fnr {
    #[doc = "FN"]
    #[inline(always)]
    pub const fn fn_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "FN"]
    #[inline(always)]
    pub fn set_fn_(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "LSOF"]
    #[inline(always)]
    pub const fn lsof(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "LSOF"]
    #[inline(always)]
    pub fn set_lsof(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "LCK"]
    #[inline(always)]
    pub const fn lck(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "LCK"]
    #[inline(always)]
    pub fn set_lck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "RXDM"]
    #[inline(always)]
    pub const fn rxdm(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RXDM"]
    #[inline(always)]
    pub fn set_rxdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RXDP"]
    #[inline(always)]
    pub const fn rxdp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "RXDP"]
    #[inline(always)]
    pub fn set_rxdp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Fnr {
    #[inline(always)]
    fn default() -> Fnr {
        Fnr(0)
    }
}
#[doc = "USB interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Istr(pub u32);
impl Istr {
    #[doc = "EP_ID"]
    #[inline(always)]
    pub const fn ep_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "EP_ID"]
    #[inline(always)]
    pub fn set_ep_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DIR"]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DIR"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "L1REQ"]
    #[inline(always)]
    pub const fn l1req(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "L1REQ"]
    #[inline(always)]
    pub fn set_l1req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ESOF"]
    #[inline(always)]
    pub const fn esof(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ESOF"]
    #[inline(always)]
    pub fn set_esof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SOF"]
    #[inline(always)]
    pub const fn sof(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SOF"]
    #[inline(always)]
    pub fn set_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SUSP"]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SUSP"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "WKUP"]
    #[inline(always)]
    pub const fn wkup(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "WKUP"]
    #[inline(always)]
    pub fn set_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ERR"]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ERR"]
    #[inline(always)]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "PMAOVR"]
    #[inline(always)]
    pub const fn pmaovr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PMAOVR"]
    #[inline(always)]
    pub fn set_pmaovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTR"]
    #[inline(always)]
    pub const fn ctr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTR"]
    #[inline(always)]
    pub fn set_ctr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Istr {
    #[inline(always)]
    fn default() -> Istr {
        Istr(0)
    }
}
