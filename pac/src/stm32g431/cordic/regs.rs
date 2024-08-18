#[doc = "CORDIC Control Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "FUNC"]
    #[inline(always)]
    pub const fn func(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FUNC"]
    #[inline(always)]
    pub fn set_func(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PRECISION"]
    #[inline(always)]
    pub const fn precision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PRECISION"]
    #[inline(always)]
    pub fn set_precision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "SCALE"]
    #[inline(always)]
    pub const fn scale(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "SCALE"]
    #[inline(always)]
    pub fn set_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "IEN"]
    #[inline(always)]
    pub const fn ien(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IEN"]
    #[inline(always)]
    pub fn set_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAREN"]
    #[inline(always)]
    pub const fn dmaren(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMAREN"]
    #[inline(always)]
    pub fn set_dmaren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAWEN"]
    #[inline(always)]
    pub const fn dmawen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMAWEN"]
    #[inline(always)]
    pub fn set_dmawen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "NRES"]
    #[inline(always)]
    pub const fn nres(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NRES"]
    #[inline(always)]
    pub fn set_nres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "NARGS"]
    #[inline(always)]
    pub const fn nargs(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "NARGS"]
    #[inline(always)]
    pub fn set_nargs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "RESSIZE"]
    #[inline(always)]
    pub const fn ressize(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "RESSIZE"]
    #[inline(always)]
    pub fn set_ressize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ARGSIZE"]
    #[inline(always)]
    pub const fn argsize(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ARGSIZE"]
    #[inline(always)]
    pub fn set_argsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "RRDY"]
    #[inline(always)]
    pub const fn rrdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RRDY"]
    #[inline(always)]
    pub fn set_rrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
#[doc = "FMAC Read Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdata(pub u32);
impl Rdata {
    #[doc = "RES"]
    #[inline(always)]
    pub const fn res(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RES"]
    #[inline(always)]
    pub fn set_res(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdata {
    #[inline(always)]
    fn default() -> Rdata {
        Rdata(0)
    }
}
#[doc = "FMAC Write Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdata(pub u32);
impl Wdata {
    #[doc = "ARG"]
    #[inline(always)]
    pub const fn arg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ARG"]
    #[inline(always)]
    pub fn set_arg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wdata {
    #[inline(always)]
    fn default() -> Wdata {
        Wdata(0)
    }
}
