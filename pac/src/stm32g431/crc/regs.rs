#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "RESET bit"]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RESET bit"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Polynomial size"]
    #[inline(always)]
    pub const fn polysize(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Polynomial size"]
    #[inline(always)]
    pub fn set_polysize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Reverse input data"]
    #[inline(always)]
    pub const fn rev_in(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Reverse input data"]
    #[inline(always)]
    pub fn set_rev_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Reverse output data"]
    #[inline(always)]
    pub const fn rev_out(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse output data"]
    #[inline(always)]
    pub fn set_rev_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "Data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Data register bits"]
    #[inline(always)]
    pub const fn dr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data register bits"]
    #[inline(always)]
    pub fn set_dr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
#[doc = "Independent data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc = "General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub const fn idr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General-purpose 8-bit data register bits"]
    #[inline(always)]
    pub fn set_idr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Idr {
    #[inline(always)]
    fn default() -> Idr {
        Idr(0)
    }
}
#[doc = "Initial CRC value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Init(pub u32);
impl Init {
    #[doc = "Programmable initial CRC value"]
    #[inline(always)]
    pub const fn crc_init(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Programmable initial CRC value"]
    #[inline(always)]
    pub fn set_crc_init(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Init {
    #[inline(always)]
    fn default() -> Init {
        Init(0)
    }
}
#[doc = "polynomial"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pol(pub u32);
impl Pol {
    #[doc = "Programmable polynomial"]
    #[inline(always)]
    pub const fn pol(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Programmable polynomial"]
    #[inline(always)]
    pub fn set_pol(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pol {
    #[inline(always)]
    fn default() -> Pol {
        Pol(0)
    }
}
