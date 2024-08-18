#[doc = "FMAC Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "RIEN"]
    #[inline(always)]
    pub const fn rien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RIEN"]
    #[inline(always)]
    pub fn set_rien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WIEN"]
    #[inline(always)]
    pub const fn wien(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WIEN"]
    #[inline(always)]
    pub fn set_wien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OVFLIEN"]
    #[inline(always)]
    pub const fn ovflien(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OVFLIEN"]
    #[inline(always)]
    pub fn set_ovflien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "UNFLIEN"]
    #[inline(always)]
    pub const fn unflien(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UNFLIEN"]
    #[inline(always)]
    pub fn set_unflien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SATIEN"]
    #[inline(always)]
    pub const fn satien(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SATIEN"]
    #[inline(always)]
    pub fn set_satien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAREN"]
    #[inline(always)]
    pub const fn dmaren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMAREN"]
    #[inline(always)]
    pub fn set_dmaren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAWEN"]
    #[inline(always)]
    pub const fn dmawen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMAWEN"]
    #[inline(always)]
    pub fn set_dmawen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CLIPEN"]
    #[inline(always)]
    pub const fn clipen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CLIPEN"]
    #[inline(always)]
    pub fn set_clipen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "FMAC Parameter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "P"]
    #[inline(always)]
    pub const fn p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "P"]
    #[inline(always)]
    pub fn set_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Q"]
    #[inline(always)]
    pub const fn q(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Q"]
    #[inline(always)]
    pub fn set_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "R"]
    #[inline(always)]
    pub const fn r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "R"]
    #[inline(always)]
    pub fn set_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "FUNC"]
    #[inline(always)]
    pub const fn func(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "FUNC"]
    #[inline(always)]
    pub fn set_func(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "START"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "START"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
#[doc = "FMAC Read Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdata(pub u32);
impl Rdata {
    #[doc = "RDATA"]
    #[inline(always)]
    pub const fn rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RDATA"]
    #[inline(always)]
    pub fn set_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rdata {
    #[inline(always)]
    fn default() -> Rdata {
        Rdata(0)
    }
}
#[doc = "FMAC Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "YEMPTY"]
    #[inline(always)]
    pub const fn yempty(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "YEMPTY"]
    #[inline(always)]
    pub fn set_yempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "X1FULL"]
    #[inline(always)]
    pub const fn x1full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "X1FULL"]
    #[inline(always)]
    pub fn set_x1full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OVFL"]
    #[inline(always)]
    pub const fn ovfl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OVFL"]
    #[inline(always)]
    pub fn set_ovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "UNFL"]
    #[inline(always)]
    pub const fn unfl(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "UNFL"]
    #[inline(always)]
    pub fn set_unfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SAT"]
    #[inline(always)]
    pub const fn sat(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SAT"]
    #[inline(always)]
    pub fn set_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "FMAC Write Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdata(pub u32);
impl Wdata {
    #[doc = "WDATA"]
    #[inline(always)]
    pub const fn wdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "WDATA"]
    #[inline(always)]
    pub fn set_wdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Wdata {
    #[inline(always)]
    fn default() -> Wdata {
        Wdata(0)
    }
}
#[doc = "FMAC X1 Buffer Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct X1bufcfg(pub u32);
impl X1bufcfg {
    #[doc = "X1_BASE"]
    #[inline(always)]
    pub const fn x1_base(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "X1_BASE"]
    #[inline(always)]
    pub fn set_x1_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "X1_BUF_SIZE"]
    #[inline(always)]
    pub const fn x1_buf_size(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "X1_BUF_SIZE"]
    #[inline(always)]
    pub fn set_x1_buf_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "FULL_WM"]
    #[inline(always)]
    pub const fn full_wm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "FULL_WM"]
    #[inline(always)]
    pub fn set_full_wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for X1bufcfg {
    #[inline(always)]
    fn default() -> X1bufcfg {
        X1bufcfg(0)
    }
}
#[doc = "FMAC X2 Buffer Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct X2bufcfg(pub u32);
impl X2bufcfg {
    #[doc = "X1_BASE"]
    #[inline(always)]
    pub const fn x2_base(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "X1_BASE"]
    #[inline(always)]
    pub fn set_x2_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "X1_BUF_SIZE"]
    #[inline(always)]
    pub const fn x2_buf_size(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "X1_BUF_SIZE"]
    #[inline(always)]
    pub fn set_x2_buf_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for X2bufcfg {
    #[inline(always)]
    fn default() -> X2bufcfg {
        X2bufcfg(0)
    }
}
#[doc = "FMAC Y Buffer Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ybufcfg(pub u32);
impl Ybufcfg {
    #[doc = "X1_BASE"]
    #[inline(always)]
    pub const fn y_base(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "X1_BASE"]
    #[inline(always)]
    pub fn set_y_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "X1_BUF_SIZE"]
    #[inline(always)]
    pub const fn y_buf_size(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "X1_BUF_SIZE"]
    #[inline(always)]
    pub fn set_y_buf_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "EMPTY_WM"]
    #[inline(always)]
    pub const fn empty_wm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "EMPTY_WM"]
    #[inline(always)]
    pub fn set_empty_wm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Ybufcfg {
    #[inline(always)]
    fn default() -> Ybufcfg {
        Ybufcfg(0)
    }
}
