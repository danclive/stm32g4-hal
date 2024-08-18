#[doc = "DMA channel 1 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
#[doc = "DMA channel 2 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        Ccr2(0)
    }
}
#[doc = "DMA channel 3 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        Ccr3(0)
    }
}
#[doc = "DMA channel 3 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        Ccr4(0)
    }
}
#[doc = "DMA channel 4 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr5 {
    #[inline(always)]
    fn default() -> Ccr5 {
        Ccr5(0)
    }
}
#[doc = "DMA channel 5 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr6(pub u32);
impl Ccr6 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr6 {
    #[inline(always)]
    fn default() -> Ccr6 {
        Ccr6(0)
    }
}
#[doc = "DMA channel 6 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr7(pub u32);
impl Ccr7 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr7 {
    #[inline(always)]
    fn default() -> Ccr7 {
        Ccr7(0)
    }
}
#[doc = "DMA channel 7 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr8(pub u32);
impl Ccr8 {
    #[doc = "channel enable"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel enable"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIE"]
    #[inline(always)]
    pub fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIE"]
    #[inline(always)]
    pub fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIE"]
    #[inline(always)]
    pub fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "CIRC"]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CIRC"]
    #[inline(always)]
    pub fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PINC"]
    #[inline(always)]
    pub fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "MINC"]
    #[inline(always)]
    pub fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PSIZE"]
    #[inline(always)]
    pub fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "MSIZE"]
    #[inline(always)]
    pub fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "PL"]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PL"]
    #[inline(always)]
    pub fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MEM2MEM"]
    #[inline(always)]
    pub fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr8 {
    #[inline(always)]
    fn default() -> Ccr8 {
        Ccr8(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar1(pub u32);
impl Cmar1 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar1 {
    #[inline(always)]
    fn default() -> Cmar1 {
        Cmar1(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar2(pub u32);
impl Cmar2 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar2 {
    #[inline(always)]
    fn default() -> Cmar2 {
        Cmar2(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar3(pub u32);
impl Cmar3 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar3 {
    #[inline(always)]
    fn default() -> Cmar3 {
        Cmar3(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar4(pub u32);
impl Cmar4 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar4 {
    #[inline(always)]
    fn default() -> Cmar4 {
        Cmar4(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar5(pub u32);
impl Cmar5 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar5 {
    #[inline(always)]
    fn default() -> Cmar5 {
        Cmar5(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar6(pub u32);
impl Cmar6 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar6 {
    #[inline(always)]
    fn default() -> Cmar6 {
        Cmar6(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar7(pub u32);
impl Cmar7 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar7 {
    #[inline(always)]
    fn default() -> Cmar7 {
        Cmar7(0)
    }
}
#[doc = "DMA channel x memory address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar8(pub u32);
impl Cmar8 {
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar8 {
    #[inline(always)]
    fn default() -> Cmar8 {
        Cmar8(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr1(pub u32);
impl Cndtr1 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr1 {
    #[inline(always)]
    fn default() -> Cndtr1 {
        Cndtr1(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr2(pub u32);
impl Cndtr2 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr2 {
    #[inline(always)]
    fn default() -> Cndtr2 {
        Cndtr2(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr3(pub u32);
impl Cndtr3 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr3 {
    #[inline(always)]
    fn default() -> Cndtr3 {
        Cndtr3(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr4(pub u32);
impl Cndtr4 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr4 {
    #[inline(always)]
    fn default() -> Cndtr4 {
        Cndtr4(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr5(pub u32);
impl Cndtr5 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr5 {
    #[inline(always)]
    fn default() -> Cndtr5 {
        Cndtr5(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr6(pub u32);
impl Cndtr6 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr6 {
    #[inline(always)]
    fn default() -> Cndtr6 {
        Cndtr6(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr7(pub u32);
impl Cndtr7 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr7 {
    #[inline(always)]
    fn default() -> Cndtr7 {
        Cndtr7(0)
    }
}
#[doc = "channel x number of data to transfer register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr8(pub u32);
impl Cndtr8 {
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data items to transfer"]
    #[inline(always)]
    pub fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr8 {
    #[inline(always)]
    fn default() -> Cndtr8 {
        Cndtr8(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar1(pub u32);
impl Cpar1 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar1 {
    #[inline(always)]
    fn default() -> Cpar1 {
        Cpar1(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar2(pub u32);
impl Cpar2 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar2 {
    #[inline(always)]
    fn default() -> Cpar2 {
        Cpar2(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar3(pub u32);
impl Cpar3 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar3 {
    #[inline(always)]
    fn default() -> Cpar3 {
        Cpar3(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar4(pub u32);
impl Cpar4 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar4 {
    #[inline(always)]
    fn default() -> Cpar4 {
        Cpar4(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar5(pub u32);
impl Cpar5 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar5 {
    #[inline(always)]
    fn default() -> Cpar5 {
        Cpar5(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar6(pub u32);
impl Cpar6 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar6 {
    #[inline(always)]
    fn default() -> Cpar6 {
        Cpar6(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar7(pub u32);
impl Cpar7 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar7 {
    #[inline(always)]
    fn default() -> Cpar7 {
        Cpar7(0)
    }
}
#[doc = "DMA channel x peripheral address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar8(pub u32);
impl Cpar8 {
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar8 {
    #[inline(always)]
    fn default() -> Cpar8 {
        Cpar8(0)
    }
}
#[doc = "DMA interrupt flag clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc = "GIF1"]
    #[inline(always)]
    pub const fn gif1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GIF1"]
    #[inline(always)]
    pub fn set_gif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIF1"]
    #[inline(always)]
    pub const fn tcif1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF1"]
    #[inline(always)]
    pub fn set_tcif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIF1"]
    #[inline(always)]
    pub const fn htif1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF1"]
    #[inline(always)]
    pub fn set_htif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIF1"]
    #[inline(always)]
    pub const fn teif1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF1"]
    #[inline(always)]
    pub fn set_teif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GIF2"]
    #[inline(always)]
    pub const fn gif2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GIF2"]
    #[inline(always)]
    pub fn set_gif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TCIF2"]
    #[inline(always)]
    pub const fn tcif2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF2"]
    #[inline(always)]
    pub fn set_tcif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HTIF2"]
    #[inline(always)]
    pub const fn htif2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF2"]
    #[inline(always)]
    pub fn set_htif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TEIF2"]
    #[inline(always)]
    pub const fn teif2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF2"]
    #[inline(always)]
    pub fn set_teif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GIF3"]
    #[inline(always)]
    pub const fn gif3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GIF3"]
    #[inline(always)]
    pub fn set_gif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TCIF3"]
    #[inline(always)]
    pub const fn tcif3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF3"]
    #[inline(always)]
    pub fn set_tcif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HTIF3"]
    #[inline(always)]
    pub const fn htif3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF3"]
    #[inline(always)]
    pub fn set_htif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TEIF3"]
    #[inline(always)]
    pub const fn teif3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF3"]
    #[inline(always)]
    pub fn set_teif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GIF4"]
    #[inline(always)]
    pub const fn gif4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GIF4"]
    #[inline(always)]
    pub fn set_gif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TCIF4"]
    #[inline(always)]
    pub const fn tcif4(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF4"]
    #[inline(always)]
    pub fn set_tcif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "HTIF4"]
    #[inline(always)]
    pub const fn htif4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF4"]
    #[inline(always)]
    pub fn set_htif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TEIF4"]
    #[inline(always)]
    pub const fn teif4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF4"]
    #[inline(always)]
    pub fn set_teif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GIF5"]
    #[inline(always)]
    pub const fn gif5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GIF5"]
    #[inline(always)]
    pub fn set_gif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TCIF5"]
    #[inline(always)]
    pub const fn tcif5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF5"]
    #[inline(always)]
    pub fn set_tcif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "HTIF5"]
    #[inline(always)]
    pub const fn htif5(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF5"]
    #[inline(always)]
    pub fn set_htif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TEIF5"]
    #[inline(always)]
    pub const fn teif5(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF5"]
    #[inline(always)]
    pub fn set_teif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GIF6"]
    #[inline(always)]
    pub const fn gif6(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GIF6"]
    #[inline(always)]
    pub fn set_gif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TCIF6"]
    #[inline(always)]
    pub const fn tcif6(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF6"]
    #[inline(always)]
    pub fn set_tcif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "HTIF6"]
    #[inline(always)]
    pub const fn htif6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF6"]
    #[inline(always)]
    pub fn set_htif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TEIF6"]
    #[inline(always)]
    pub const fn teif6(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF6"]
    #[inline(always)]
    pub fn set_teif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GIF7"]
    #[inline(always)]
    pub const fn gif7(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GIF7"]
    #[inline(always)]
    pub fn set_gif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TCIF7"]
    #[inline(always)]
    pub const fn tcif7(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF7"]
    #[inline(always)]
    pub fn set_tcif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "HTIF7"]
    #[inline(always)]
    pub const fn htif7(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF7"]
    #[inline(always)]
    pub fn set_htif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "TEIF7"]
    #[inline(always)]
    pub const fn teif7(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF7"]
    #[inline(always)]
    pub fn set_teif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GIF8"]
    #[inline(always)]
    pub const fn gif8(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GIF8"]
    #[inline(always)]
    pub fn set_gif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TCIF8"]
    #[inline(always)]
    pub const fn tcif8(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF8"]
    #[inline(always)]
    pub fn set_tcif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "HTIF8"]
    #[inline(always)]
    pub const fn htif8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF8"]
    #[inline(always)]
    pub fn set_htif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "TEIF8"]
    #[inline(always)]
    pub const fn teif8(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF8"]
    #[inline(always)]
    pub fn set_teif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ifcr {
    #[inline(always)]
    fn default() -> Ifcr {
        Ifcr(0)
    }
}
#[doc = "interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "GIF1"]
    #[inline(always)]
    pub const fn gif1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GIF1"]
    #[inline(always)]
    pub fn set_gif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TCIF1"]
    #[inline(always)]
    pub const fn tcif1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF1"]
    #[inline(always)]
    pub fn set_tcif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HTIF1"]
    #[inline(always)]
    pub const fn htif1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF1"]
    #[inline(always)]
    pub fn set_htif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TEIF1"]
    #[inline(always)]
    pub const fn teif1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF1"]
    #[inline(always)]
    pub fn set_teif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GIF2"]
    #[inline(always)]
    pub const fn gif2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GIF2"]
    #[inline(always)]
    pub fn set_gif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TCIF2"]
    #[inline(always)]
    pub const fn tcif2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF2"]
    #[inline(always)]
    pub fn set_tcif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HTIF2"]
    #[inline(always)]
    pub const fn htif2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF2"]
    #[inline(always)]
    pub fn set_htif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TEIF2"]
    #[inline(always)]
    pub const fn teif2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF2"]
    #[inline(always)]
    pub fn set_teif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GIF3"]
    #[inline(always)]
    pub const fn gif3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GIF3"]
    #[inline(always)]
    pub fn set_gif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TCIF3"]
    #[inline(always)]
    pub const fn tcif3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF3"]
    #[inline(always)]
    pub fn set_tcif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HTIF3"]
    #[inline(always)]
    pub const fn htif3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF3"]
    #[inline(always)]
    pub fn set_htif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TEIF3"]
    #[inline(always)]
    pub const fn teif3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF3"]
    #[inline(always)]
    pub fn set_teif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GIF4"]
    #[inline(always)]
    pub const fn gif4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GIF4"]
    #[inline(always)]
    pub fn set_gif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TCIF4"]
    #[inline(always)]
    pub const fn tcif4(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF4"]
    #[inline(always)]
    pub fn set_tcif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "HTIF4"]
    #[inline(always)]
    pub const fn htif4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF4"]
    #[inline(always)]
    pub fn set_htif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TEIF4"]
    #[inline(always)]
    pub const fn teif4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF4"]
    #[inline(always)]
    pub fn set_teif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GIF5"]
    #[inline(always)]
    pub const fn gif5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "GIF5"]
    #[inline(always)]
    pub fn set_gif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TCIF5"]
    #[inline(always)]
    pub const fn tcif5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF5"]
    #[inline(always)]
    pub fn set_tcif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "HTIF5"]
    #[inline(always)]
    pub const fn htif5(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF5"]
    #[inline(always)]
    pub fn set_htif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TEIF5"]
    #[inline(always)]
    pub const fn teif5(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF5"]
    #[inline(always)]
    pub fn set_teif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "GIF6"]
    #[inline(always)]
    pub const fn gif6(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GIF6"]
    #[inline(always)]
    pub fn set_gif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TCIF6"]
    #[inline(always)]
    pub const fn tcif6(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF6"]
    #[inline(always)]
    pub fn set_tcif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "HTIF6"]
    #[inline(always)]
    pub const fn htif6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF6"]
    #[inline(always)]
    pub fn set_htif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TEIF6"]
    #[inline(always)]
    pub const fn teif6(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF6"]
    #[inline(always)]
    pub fn set_teif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GIF7"]
    #[inline(always)]
    pub const fn gif7(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GIF7"]
    #[inline(always)]
    pub fn set_gif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TCIF7"]
    #[inline(always)]
    pub const fn tcif7(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF7"]
    #[inline(always)]
    pub fn set_tcif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "HTIF7"]
    #[inline(always)]
    pub const fn htif7(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF7"]
    #[inline(always)]
    pub fn set_htif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "TEIF7"]
    #[inline(always)]
    pub const fn teif7(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF7"]
    #[inline(always)]
    pub fn set_teif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "GIF8"]
    #[inline(always)]
    pub const fn gif8(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GIF8"]
    #[inline(always)]
    pub fn set_gif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TCIF8"]
    #[inline(always)]
    pub const fn tcif8(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "TCIF8"]
    #[inline(always)]
    pub fn set_tcif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "HTIF8"]
    #[inline(always)]
    pub const fn htif8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "HTIF8"]
    #[inline(always)]
    pub fn set_htif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "TEIF8"]
    #[inline(always)]
    pub const fn teif8(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TEIF8"]
    #[inline(always)]
    pub fn set_teif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
