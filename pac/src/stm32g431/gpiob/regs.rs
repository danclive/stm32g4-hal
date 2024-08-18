#[doc = "GPIO alternate function high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afrh(pub u32);
impl Afrh {
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub const fn afrh15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn set_afrh15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Afrh {
    #[inline(always)]
    fn default() -> Afrh {
        Afrh(0)
    }
}
#[doc = "GPIO alternate function low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afrl(pub u32);
impl Afrl {
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub const fn afrl7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn set_afrl7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Afrl {
    #[inline(always)]
    fn default() -> Afrl {
        Afrl(0)
    }
}
#[doc = "GPIO port bit reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub const fn br15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset bit"]
    #[inline(always)]
    pub fn set_br15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        Brr(0)
    }
}
#[doc = "GPIO port bit set/reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsrr(pub u32);
impl Bsrr {
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn bs15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_bs15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn br0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub const fn br15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn set_br15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Bsrr {
    #[inline(always)]
    fn default() -> Bsrr {
        Bsrr(0)
    }
}
#[doc = "GPIO port input data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub const fn idr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn set_idr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Idr {
    #[inline(always)]
    fn default() -> Idr {
        Idr(0)
    }
}
#[doc = "GPIO port configuration lock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lckr(pub u32);
impl Lckr {
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lck15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lck15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub const fn lckk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn set_lckk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Lckr {
    #[inline(always)]
    fn default() -> Lckr {
        Lckr(0)
    }
}
#[doc = "GPIO port mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moder(pub u32);
impl Moder {
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn moder15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_moder15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Moder {
    #[inline(always)]
    fn default() -> Moder {
        Moder(0)
    }
}
#[doc = "GPIO port output data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Odr(pub u32);
impl Odr {
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub const fn odr15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn set_odr15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Odr {
    #[inline(always)]
    fn default() -> Odr {
        Odr(0)
    }
}
#[doc = "GPIO port output speed register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ospeedr(pub u32);
impl Ospeedr {
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ospeedr15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ospeedr15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ospeedr {
    #[inline(always)]
    fn default() -> Ospeedr {
        Ospeedr(0)
    }
}
#[doc = "GPIO port output type register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otyper(pub u32);
impl Otyper {
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn ot15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_ot15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Otyper {
    #[inline(always)]
    fn default() -> Otyper {
        Otyper(0)
    }
}
#[doc = "GPIO port pull-up/pull-down register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pupdr(pub u32);
impl Pupdr {
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub const fn pupdr15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn set_pupdr15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Pupdr {
    #[inline(always)]
    fn default() -> Pupdr {
        Pupdr(0)
    }
}
