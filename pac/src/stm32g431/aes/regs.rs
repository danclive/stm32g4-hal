#[doc = "control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "AES enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AES enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub const fn datatype(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn set_datatype(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "AES operating mode"]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "AES operating mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "AES chaining mode"]
    #[inline(always)]
    pub const fn chmod(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "AES chaining mode"]
    #[inline(always)]
    pub fn set_chmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Computation Complete Flag Clear"]
    #[inline(always)]
    pub const fn ccfc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn set_ccfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error clear"]
    #[inline(always)]
    pub const fn errc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Error clear"]
    #[inline(always)]
    pub fn set_errc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CCF flag interrupt enable"]
    #[inline(always)]
    pub const fn ccfie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CCF flag interrupt enable"]
    #[inline(always)]
    pub fn set_ccfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn errie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn set_errie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable DMA management of data input phase"]
    #[inline(always)]
    pub const fn dmainen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn set_dmainen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable DMA management of data output phase"]
    #[inline(always)]
    pub const fn dmaouten(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn set_dmaouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GCMPH"]
    #[inline(always)]
    pub const fn gcmph(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "GCMPH"]
    #[inline(always)]
    pub fn set_gcmph(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "CHMOD_2"]
    #[inline(always)]
    pub const fn chmod_2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CHMOD_2"]
    #[inline(always)]
    pub fn set_chmod_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "KEYSIZE"]
    #[inline(always)]
    pub const fn keysize(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "KEYSIZE"]
    #[inline(always)]
    pub fn set_keysize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "NPBLB"]
    #[inline(always)]
    pub const fn npblb(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "NPBLB"]
    #[inline(always)]
    pub fn set_npblb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "data input register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dinr(pub u32);
impl Dinr {
    #[doc = "Data Input Register"]
    #[inline(always)]
    pub const fn aes_dinr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Input Register"]
    #[inline(always)]
    pub fn set_aes_dinr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dinr {
    #[inline(always)]
    fn default() -> Dinr {
        Dinr(0)
    }
}
#[doc = "data output register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doutr(pub u32);
impl Doutr {
    #[doc = "Data output register"]
    #[inline(always)]
    pub const fn aes_doutr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data output register"]
    #[inline(always)]
    pub fn set_aes_doutr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Doutr {
    #[inline(always)]
    fn default() -> Doutr {
        Doutr(0)
    }
}
#[doc = "initialization vector register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivr0(pub u32);
impl Ivr0 {
    #[doc = "initialization vector register (LSB IVR \\[31:0\\])"]
    #[inline(always)]
    pub const fn aes_ivr0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "initialization vector register (LSB IVR \\[31:0\\])"]
    #[inline(always)]
    pub fn set_aes_ivr0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ivr0 {
    #[inline(always)]
    fn default() -> Ivr0 {
        Ivr0(0)
    }
}
#[doc = "initialization vector register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivr1(pub u32);
impl Ivr1 {
    #[doc = "Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub const fn aes_ivr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn set_aes_ivr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ivr1 {
    #[inline(always)]
    fn default() -> Ivr1 {
        Ivr1(0)
    }
}
#[doc = "initialization vector register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivr2(pub u32);
impl Ivr2 {
    #[doc = "Initialization Vector Register (IVR \\[95:64\\])"]
    #[inline(always)]
    pub const fn aes_ivr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initialization Vector Register (IVR \\[95:64\\])"]
    #[inline(always)]
    pub fn set_aes_ivr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ivr2 {
    #[inline(always)]
    fn default() -> Ivr2 {
        Ivr2(0)
    }
}
#[doc = "initialization vector register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivr3(pub u32);
impl Ivr3 {
    #[doc = "Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub const fn aes_ivr3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn set_aes_ivr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ivr3 {
    #[inline(always)]
    fn default() -> Ivr3 {
        Ivr3(0)
    }
}
#[doc = "key register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr0(pub u32);
impl Keyr0 {
    #[doc = "Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    pub const fn aes_keyr0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    pub fn set_aes_keyr0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr0 {
    #[inline(always)]
    fn default() -> Keyr0 {
        Keyr0(0)
    }
}
#[doc = "key register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr1(pub u32);
impl Keyr1 {
    #[doc = "AES key register (key \\[63:32\\])"]
    #[inline(always)]
    pub const fn aes_keyr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key register (key \\[63:32\\])"]
    #[inline(always)]
    pub fn set_aes_keyr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr1 {
    #[inline(always)]
    fn default() -> Keyr1 {
        Keyr1(0)
    }
}
#[doc = "key register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr2(pub u32);
impl Keyr2 {
    #[doc = "AES key register (key \\[95:64\\])"]
    #[inline(always)]
    pub const fn aes_keyr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key register (key \\[95:64\\])"]
    #[inline(always)]
    pub fn set_aes_keyr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr2 {
    #[inline(always)]
    fn default() -> Keyr2 {
        Keyr2(0)
    }
}
#[doc = "key register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr3(pub u32);
impl Keyr3 {
    #[doc = "AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub const fn aes_keyr3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn set_aes_keyr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr3 {
    #[inline(always)]
    fn default() -> Keyr3 {
        Keyr3(0)
    }
}
#[doc = "key register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr4(pub u32);
impl Keyr4 {
    #[doc = "AES key"]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr4 {
    #[inline(always)]
    fn default() -> Keyr4 {
        Keyr4(0)
    }
}
#[doc = "key register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr5(pub u32);
impl Keyr5 {
    #[doc = "AES key"]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr5 {
    #[inline(always)]
    fn default() -> Keyr5 {
        Keyr5(0)
    }
}
#[doc = "key register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr6(pub u32);
impl Keyr6 {
    #[doc = "AES key"]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr6 {
    #[inline(always)]
    fn default() -> Keyr6 {
        Keyr6(0)
    }
}
#[doc = "key register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyr7(pub u32);
impl Keyr7 {
    #[doc = "AES key"]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyr7 {
    #[inline(always)]
    fn default() -> Keyr7 {
        Keyr7(0)
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Computation complete flag"]
    #[inline(always)]
    pub const fn ccf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Computation complete flag"]
    #[inline(always)]
    pub fn set_ccf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read error flag"]
    #[inline(always)]
    pub const fn rderr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read error flag"]
    #[inline(always)]
    pub fn set_rderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write error flag"]
    #[inline(always)]
    pub const fn wrerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write error flag"]
    #[inline(always)]
    pub fn set_wrerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BUSY"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BUSY"]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp0r(pub u32);
impl Susp0r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp0r {
    #[inline(always)]
    fn default() -> Susp0r {
        Susp0r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp1r(pub u32);
impl Susp1r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp1r {
    #[inline(always)]
    fn default() -> Susp1r {
        Susp1r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp2r(pub u32);
impl Susp2r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp2r {
    #[inline(always)]
    fn default() -> Susp2r {
        Susp2r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp3r(pub u32);
impl Susp3r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp3r {
    #[inline(always)]
    fn default() -> Susp3r {
        Susp3r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp4r(pub u32);
impl Susp4r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp4r {
    #[inline(always)]
    fn default() -> Susp4r {
        Susp4r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp5r(pub u32);
impl Susp5r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp5r {
    #[inline(always)]
    fn default() -> Susp5r {
        Susp5r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp6r(pub u32);
impl Susp6r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp6r {
    #[inline(always)]
    fn default() -> Susp6r {
        Susp6r(0)
    }
}
#[doc = "suspend registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Susp7r(pub u32);
impl Susp7r {
    #[doc = "AES suspend"]
    #[inline(always)]
    pub const fn susp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES suspend"]
    #[inline(always)]
    pub fn set_susp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Susp7r {
    #[inline(always)]
    fn default() -> Susp7r {
        Susp7r(0)
    }
}
