#[doc = "Event mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emr1(pub u32);
impl Emr1 {
    #[doc = "Event Mask on line 0"]
    #[inline(always)]
    pub const fn em0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 0"]
    #[inline(always)]
    pub fn set_em0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event Mask on line 1"]
    #[inline(always)]
    pub const fn em1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 1"]
    #[inline(always)]
    pub fn set_em1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event Mask on line 2"]
    #[inline(always)]
    pub const fn em2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 2"]
    #[inline(always)]
    pub fn set_em2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event Mask on line 3"]
    #[inline(always)]
    pub const fn em3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 3"]
    #[inline(always)]
    pub fn set_em3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event Mask on line 4"]
    #[inline(always)]
    pub const fn em4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 4"]
    #[inline(always)]
    pub fn set_em4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event Mask on line 5"]
    #[inline(always)]
    pub const fn em5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 5"]
    #[inline(always)]
    pub fn set_em5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Event Mask on line 6"]
    #[inline(always)]
    pub const fn em6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 6"]
    #[inline(always)]
    pub fn set_em6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Event Mask on line 7"]
    #[inline(always)]
    pub const fn em7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 7"]
    #[inline(always)]
    pub fn set_em7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Event Mask on line 8"]
    #[inline(always)]
    pub const fn em8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 8"]
    #[inline(always)]
    pub fn set_em8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Event Mask on line 9"]
    #[inline(always)]
    pub const fn em9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 9"]
    #[inline(always)]
    pub fn set_em9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Event Mask on line 10"]
    #[inline(always)]
    pub const fn em10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 10"]
    #[inline(always)]
    pub fn set_em10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Event Mask on line 11"]
    #[inline(always)]
    pub const fn em11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 11"]
    #[inline(always)]
    pub fn set_em11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Event Mask on line 12"]
    #[inline(always)]
    pub const fn em12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 12"]
    #[inline(always)]
    pub fn set_em12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Event Mask on line 13"]
    #[inline(always)]
    pub const fn em13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 13"]
    #[inline(always)]
    pub fn set_em13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Event Mask on line 14"]
    #[inline(always)]
    pub const fn em14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 14"]
    #[inline(always)]
    pub fn set_em14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Event Mask on line 15"]
    #[inline(always)]
    pub const fn em15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 15"]
    #[inline(always)]
    pub fn set_em15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Event Mask on line 16"]
    #[inline(always)]
    pub const fn em16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 16"]
    #[inline(always)]
    pub fn set_em16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Event Mask on line 17"]
    #[inline(always)]
    pub const fn em17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 17"]
    #[inline(always)]
    pub fn set_em17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event Mask on line 18"]
    #[inline(always)]
    pub const fn em18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 18"]
    #[inline(always)]
    pub fn set_em18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Event Mask on line 19"]
    #[inline(always)]
    pub const fn em19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 19"]
    #[inline(always)]
    pub fn set_em19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event Mask on line 20"]
    #[inline(always)]
    pub const fn em20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 20"]
    #[inline(always)]
    pub fn set_em20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Event Mask on line 21"]
    #[inline(always)]
    pub const fn em21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 21"]
    #[inline(always)]
    pub fn set_em21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Event Mask on line 22"]
    #[inline(always)]
    pub const fn em22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 22"]
    #[inline(always)]
    pub fn set_em22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Event Mask on line 23"]
    #[inline(always)]
    pub const fn em23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 23"]
    #[inline(always)]
    pub fn set_em23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Event Mask on line 24"]
    #[inline(always)]
    pub const fn em24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 24"]
    #[inline(always)]
    pub fn set_em24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Event Mask on line 25"]
    #[inline(always)]
    pub const fn em25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 25"]
    #[inline(always)]
    pub fn set_em25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Event Mask on line 26"]
    #[inline(always)]
    pub const fn em26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 26"]
    #[inline(always)]
    pub fn set_em26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Event Mask on line 27"]
    #[inline(always)]
    pub const fn em27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 27"]
    #[inline(always)]
    pub fn set_em27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Event Mask on line 28"]
    #[inline(always)]
    pub const fn em28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 28"]
    #[inline(always)]
    pub fn set_em28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Event Mask on line 29"]
    #[inline(always)]
    pub const fn em29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 29"]
    #[inline(always)]
    pub fn set_em29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Event Mask on line 30"]
    #[inline(always)]
    pub const fn em30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 30"]
    #[inline(always)]
    pub fn set_em30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Event Mask on line 31"]
    #[inline(always)]
    pub const fn em31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Event Mask on line 31"]
    #[inline(always)]
    pub fn set_em31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Emr1 {
    #[inline(always)]
    fn default() -> Emr1 {
        Emr1(0)
    }
}
#[doc = "Event mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emr2(pub u32);
impl Emr2 {
    #[doc = "Event mask on external/internal line 32"]
    #[inline(always)]
    pub const fn em32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn set_em32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event mask on external/internal line 33"]
    #[inline(always)]
    pub const fn em33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn set_em33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event mask on external/internal line 34"]
    #[inline(always)]
    pub const fn em34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn set_em34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event mask on external/internal line 35"]
    #[inline(always)]
    pub const fn em35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn set_em35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event mask on external/internal line 36"]
    #[inline(always)]
    pub const fn em36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn set_em36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event mask on external/internal line 37"]
    #[inline(always)]
    pub const fn em37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn set_em37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Event mask on external/internal line 38"]
    #[inline(always)]
    pub const fn em38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 38"]
    #[inline(always)]
    pub fn set_em38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Event mask on external/internal line 39"]
    #[inline(always)]
    pub const fn em39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 39"]
    #[inline(always)]
    pub fn set_em39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Event mask on external/internal line 40"]
    #[inline(always)]
    pub const fn em40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Event mask on external/internal line 40"]
    #[inline(always)]
    pub fn set_em40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Emr2 {
    #[inline(always)]
    fn default() -> Emr2 {
        Emr2(0)
    }
}
#[doc = "Falling Trigger selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftsr1(pub u32);
impl Ftsr1 {
    #[doc = "Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub const fn ft0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn set_ft0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub const fn ft1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn set_ft1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub const fn ft2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn set_ft2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub const fn ft3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn set_ft3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub const fn ft4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn set_ft4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub const fn ft5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn set_ft5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub const fn ft6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn set_ft6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub const fn ft7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn set_ft7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub const fn ft8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn set_ft8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub const fn ft9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn set_ft9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub const fn ft10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn set_ft10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub const fn ft11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn set_ft11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub const fn ft12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn set_ft12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub const fn ft13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn set_ft13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub const fn ft14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn set_ft14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub const fn ft15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn set_ft15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub const fn ft16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn set_ft16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub const fn ft17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn set_ft17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub const fn ft19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn set_ft19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub const fn ft20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn set_ft20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub const fn ft21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn set_ft21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub const fn ft22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn set_ft22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Ftsr1 {
    #[inline(always)]
    fn default() -> Ftsr1 {
        Ftsr1(0)
    }
}
#[doc = "Falling Trigger selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftsr2(pub u32);
impl Ftsr2 {
    #[doc = "Falling trigger event configuration bit of line 35"]
    #[inline(always)]
    pub const fn ft35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn set_ft35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling trigger event configuration bit of line 36"]
    #[inline(always)]
    pub const fn ft36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn set_ft36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Falling trigger event configuration bit of line 37"]
    #[inline(always)]
    pub const fn ft37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn set_ft37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Falling trigger event configuration bit of line 38"]
    #[inline(always)]
    pub const fn ft38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Falling trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn set_ft38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Ftsr2 {
    #[inline(always)]
    fn default() -> Ftsr2 {
        Ftsr2(0)
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr1(pub u32);
impl Imr1 {
    #[doc = "Interrupt Mask on line 0"]
    #[inline(always)]
    pub const fn im0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn set_im0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Mask on line 1"]
    #[inline(always)]
    pub const fn im1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn set_im1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Mask on line 2"]
    #[inline(always)]
    pub const fn im2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn set_im2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Mask on line 3"]
    #[inline(always)]
    pub const fn im3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn set_im3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Mask on line 4"]
    #[inline(always)]
    pub const fn im4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn set_im4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Mask on line 5"]
    #[inline(always)]
    pub const fn im5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn set_im5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Mask on line 6"]
    #[inline(always)]
    pub const fn im6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn set_im6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Mask on line 7"]
    #[inline(always)]
    pub const fn im7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn set_im7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Mask on line 8"]
    #[inline(always)]
    pub const fn im8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn set_im8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Mask on line 9"]
    #[inline(always)]
    pub const fn im9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn set_im9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Mask on line 10"]
    #[inline(always)]
    pub const fn im10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn set_im10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Mask on line 11"]
    #[inline(always)]
    pub const fn im11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn set_im11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Mask on line 12"]
    #[inline(always)]
    pub const fn im12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn set_im12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Mask on line 13"]
    #[inline(always)]
    pub const fn im13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn set_im13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Mask on line 14"]
    #[inline(always)]
    pub const fn im14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn set_im14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Mask on line 15"]
    #[inline(always)]
    pub const fn im15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn set_im15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Mask on line 16"]
    #[inline(always)]
    pub const fn im16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn set_im16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Mask on line 17"]
    #[inline(always)]
    pub const fn im17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn set_im17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Mask on line 18"]
    #[inline(always)]
    pub const fn im18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn set_im18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Mask on line 19"]
    #[inline(always)]
    pub const fn im19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn set_im19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Mask on line 20"]
    #[inline(always)]
    pub const fn im20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 20"]
    #[inline(always)]
    pub fn set_im20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Mask on line 21"]
    #[inline(always)]
    pub const fn im21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn set_im21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Mask on line 22"]
    #[inline(always)]
    pub const fn im22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn set_im22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Mask on line 23"]
    #[inline(always)]
    pub const fn im23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 23"]
    #[inline(always)]
    pub fn set_im23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Mask on line 24"]
    #[inline(always)]
    pub const fn im24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 24"]
    #[inline(always)]
    pub fn set_im24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Mask on line 25"]
    #[inline(always)]
    pub const fn im25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 25"]
    #[inline(always)]
    pub fn set_im25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Mask on line 26"]
    #[inline(always)]
    pub const fn im26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 26"]
    #[inline(always)]
    pub fn set_im26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Mask on line 27"]
    #[inline(always)]
    pub const fn im27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 27"]
    #[inline(always)]
    pub fn set_im27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Mask on line 28"]
    #[inline(always)]
    pub const fn im28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 28"]
    #[inline(always)]
    pub fn set_im28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Mask on line 29"]
    #[inline(always)]
    pub const fn im29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 29"]
    #[inline(always)]
    pub fn set_im29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Mask on line 30"]
    #[inline(always)]
    pub const fn im30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 30"]
    #[inline(always)]
    pub fn set_im30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Mask on line 31"]
    #[inline(always)]
    pub const fn im31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on line 31"]
    #[inline(always)]
    pub fn set_im31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Imr1 {
    #[inline(always)]
    fn default() -> Imr1 {
        Imr1(0)
    }
}
#[doc = "Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr2(pub u32);
impl Imr2 {
    #[doc = "Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub const fn im32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 32"]
    #[inline(always)]
    pub fn set_im32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub const fn im33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 33"]
    #[inline(always)]
    pub fn set_im33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub const fn im34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 34"]
    #[inline(always)]
    pub fn set_im34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub const fn im35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 35"]
    #[inline(always)]
    pub fn set_im35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    pub const fn im36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 36"]
    #[inline(always)]
    pub fn set_im36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    pub const fn im37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 37"]
    #[inline(always)]
    pub fn set_im37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Mask on external/internal line 38"]
    #[inline(always)]
    pub const fn im38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 38"]
    #[inline(always)]
    pub fn set_im38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Mask on external/internal line 39"]
    #[inline(always)]
    pub const fn im39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 39"]
    #[inline(always)]
    pub fn set_im39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Mask on external/internal line 40"]
    #[inline(always)]
    pub const fn im40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 40"]
    #[inline(always)]
    pub fn set_im40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Mask on external/internal line 41"]
    #[inline(always)]
    pub const fn im41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 41"]
    #[inline(always)]
    pub fn set_im41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Mask on external/internal line 42"]
    #[inline(always)]
    pub const fn im42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 42"]
    #[inline(always)]
    pub fn set_im42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Mask on external/internal line 43"]
    #[inline(always)]
    pub const fn im43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Mask on external/internal line 43"]
    #[inline(always)]
    pub fn set_im43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Imr2 {
    #[inline(always)]
    fn default() -> Imr2 {
        Imr2(0)
    }
}
#[doc = "Pending register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr1(pub u32);
impl Pr1 {
    #[doc = "Pending bit 0"]
    #[inline(always)]
    pub const fn pif0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 0"]
    #[inline(always)]
    pub fn set_pif0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pending bit 1"]
    #[inline(always)]
    pub const fn pif1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 1"]
    #[inline(always)]
    pub fn set_pif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pending bit 2"]
    #[inline(always)]
    pub const fn pif2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 2"]
    #[inline(always)]
    pub fn set_pif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pending bit 3"]
    #[inline(always)]
    pub const fn pif3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 3"]
    #[inline(always)]
    pub fn set_pif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pending bit 4"]
    #[inline(always)]
    pub const fn pif4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 4"]
    #[inline(always)]
    pub fn set_pif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pending bit 5"]
    #[inline(always)]
    pub const fn pif5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 5"]
    #[inline(always)]
    pub fn set_pif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pending bit 6"]
    #[inline(always)]
    pub const fn pif6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 6"]
    #[inline(always)]
    pub fn set_pif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Pending bit 7"]
    #[inline(always)]
    pub const fn pif7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 7"]
    #[inline(always)]
    pub fn set_pif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Pending bit 8"]
    #[inline(always)]
    pub const fn pif8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 8"]
    #[inline(always)]
    pub fn set_pif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Pending bit 9"]
    #[inline(always)]
    pub const fn pif9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 9"]
    #[inline(always)]
    pub fn set_pif9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Pending bit 10"]
    #[inline(always)]
    pub const fn pif10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 10"]
    #[inline(always)]
    pub fn set_pif10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Pending bit 11"]
    #[inline(always)]
    pub const fn pif11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 11"]
    #[inline(always)]
    pub fn set_pif11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pending bit 12"]
    #[inline(always)]
    pub const fn pif12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 12"]
    #[inline(always)]
    pub fn set_pif12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pending bit 13"]
    #[inline(always)]
    pub const fn pif13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 13"]
    #[inline(always)]
    pub fn set_pif13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pending bit 14"]
    #[inline(always)]
    pub const fn pif14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 14"]
    #[inline(always)]
    pub fn set_pif14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Pending bit 15"]
    #[inline(always)]
    pub const fn pif15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 15"]
    #[inline(always)]
    pub fn set_pif15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Pending bit 16"]
    #[inline(always)]
    pub const fn pif16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 16"]
    #[inline(always)]
    pub fn set_pif16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pending bit 17"]
    #[inline(always)]
    pub const fn pif17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 17"]
    #[inline(always)]
    pub fn set_pif17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Pending bit 19"]
    #[inline(always)]
    pub const fn pif19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 19"]
    #[inline(always)]
    pub fn set_pif19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Pending bit 20"]
    #[inline(always)]
    pub const fn pif20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 20"]
    #[inline(always)]
    pub fn set_pif20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Pending bit 21"]
    #[inline(always)]
    pub const fn pif21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 21"]
    #[inline(always)]
    pub fn set_pif21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Pending bit 22"]
    #[inline(always)]
    pub const fn pif22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Pending bit 22"]
    #[inline(always)]
    pub fn set_pif22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Pr1 {
    #[inline(always)]
    fn default() -> Pr1 {
        Pr1(0)
    }
}
#[doc = "Pending register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr2(pub u32);
impl Pr2 {
    #[doc = "Pending interrupt flag on line 35"]
    #[inline(always)]
    pub const fn pif35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn set_pif35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pending interrupt flag on line 36"]
    #[inline(always)]
    pub const fn pif36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn set_pif36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Pending interrupt flag on line 37"]
    #[inline(always)]
    pub const fn pif37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn set_pif37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pending interrupt flag on line 38"]
    #[inline(always)]
    pub const fn pif38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn set_pif38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Pr2 {
    #[inline(always)]
    fn default() -> Pr2 {
        Pr2(0)
    }
}
#[doc = "Rising Trigger selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtsr1(pub u32);
impl Rtsr1 {
    #[doc = "Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub const fn rt0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn set_rt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub const fn rt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn set_rt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub const fn rt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn set_rt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub const fn rt3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn set_rt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub const fn rt4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn set_rt4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub const fn rt5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn set_rt5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub const fn rt6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn set_rt6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub const fn rt7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn set_rt7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub const fn rt8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn set_rt8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub const fn rt9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn set_rt9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub const fn rt10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn set_rt10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub const fn rt11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn set_rt11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub const fn rt12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn set_rt12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub const fn rt13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn set_rt13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub const fn rt14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn set_rt14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub const fn rt15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn set_rt15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub const fn rt16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn set_rt16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub const fn rt17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn set_rt17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub const fn rt19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub fn set_rt19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Rising trigger event configuration of line 20"]
    #[inline(always)]
    pub const fn rt20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 20"]
    #[inline(always)]
    pub fn set_rt20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub const fn rt21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub fn set_rt21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub const fn rt22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub fn set_rt22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "RT"]
    #[inline(always)]
    pub const fn rt(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "RT"]
    #[inline(always)]
    pub fn set_rt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Rtsr1 {
    #[inline(always)]
    fn default() -> Rtsr1 {
        Rtsr1(0)
    }
}
#[doc = "Rising Trigger selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtsr2(pub u32);
impl Rtsr2 {
    #[doc = "Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub const fn rt32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn set_rt32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub const fn rt33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn set_rt33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub const fn rt38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn set_rt38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rising trigger event configuration bit of line 39"]
    #[inline(always)]
    pub const fn rt39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 39"]
    #[inline(always)]
    pub fn set_rt39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    pub const fn rt40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    pub fn set_rt40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    pub const fn rt41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    pub fn set_rt41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Rtsr2 {
    #[inline(always)]
    fn default() -> Rtsr2 {
        Rtsr2(0)
    }
}
#[doc = "Software interrupt event register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swier1(pub u32);
impl Swier1 {
    #[doc = "Software Interrupt on line 0"]
    #[inline(always)]
    pub const fn swi0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 0"]
    #[inline(always)]
    pub fn set_swi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Interrupt on line 1"]
    #[inline(always)]
    pub const fn swi1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 1"]
    #[inline(always)]
    pub fn set_swi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Interrupt on line 2"]
    #[inline(always)]
    pub const fn swi2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 2"]
    #[inline(always)]
    pub fn set_swi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Software Interrupt on line 3"]
    #[inline(always)]
    pub const fn swi3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 3"]
    #[inline(always)]
    pub fn set_swi3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Software Interrupt on line 4"]
    #[inline(always)]
    pub const fn swi4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 4"]
    #[inline(always)]
    pub fn set_swi4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Software Interrupt on line 5"]
    #[inline(always)]
    pub const fn swi5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 5"]
    #[inline(always)]
    pub fn set_swi5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software Interrupt on line 6"]
    #[inline(always)]
    pub const fn swi6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 6"]
    #[inline(always)]
    pub fn set_swi6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Software Interrupt on line 7"]
    #[inline(always)]
    pub const fn swi7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 7"]
    #[inline(always)]
    pub fn set_swi7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Software Interrupt on line 8"]
    #[inline(always)]
    pub const fn swi8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 8"]
    #[inline(always)]
    pub fn set_swi8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Software Interrupt on line 9"]
    #[inline(always)]
    pub const fn swi9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 9"]
    #[inline(always)]
    pub fn set_swi9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Software Interrupt on line 10"]
    #[inline(always)]
    pub const fn swi10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 10"]
    #[inline(always)]
    pub fn set_swi10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Interrupt on line 11"]
    #[inline(always)]
    pub const fn swi11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 11"]
    #[inline(always)]
    pub fn set_swi11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Software Interrupt on line 12"]
    #[inline(always)]
    pub const fn swi12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 12"]
    #[inline(always)]
    pub fn set_swi12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Software Interrupt on line 13"]
    #[inline(always)]
    pub const fn swi13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 13"]
    #[inline(always)]
    pub fn set_swi13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Interrupt on line 14"]
    #[inline(always)]
    pub const fn swi14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 14"]
    #[inline(always)]
    pub fn set_swi14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Software Interrupt on line 15"]
    #[inline(always)]
    pub const fn swi15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 15"]
    #[inline(always)]
    pub fn set_swi15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Software Interrupt on line 16"]
    #[inline(always)]
    pub const fn swi16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 16"]
    #[inline(always)]
    pub fn set_swi16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Software Interrupt on line 17"]
    #[inline(always)]
    pub const fn swi17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 17"]
    #[inline(always)]
    pub fn set_swi17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Software Interrupt on line 19"]
    #[inline(always)]
    pub const fn swi19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 19"]
    #[inline(always)]
    pub fn set_swi19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Software Interrupt on line 20"]
    #[inline(always)]
    pub const fn swi20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 20"]
    #[inline(always)]
    pub fn set_swi20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Interrupt on line 21"]
    #[inline(always)]
    pub const fn swi21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 21"]
    #[inline(always)]
    pub fn set_swi21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Software Interrupt on line 22"]
    #[inline(always)]
    pub const fn swi22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Software Interrupt on line 22"]
    #[inline(always)]
    pub fn set_swi22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Swier1 {
    #[inline(always)]
    fn default() -> Swier1 {
        Swier1(0)
    }
}
#[doc = "Software interrupt event register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swier2(pub u32);
impl Swier2 {
    #[doc = "Software interrupt on line 35"]
    #[inline(always)]
    pub const fn swi35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 35"]
    #[inline(always)]
    pub fn set_swi35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Software interrupt on line 36"]
    #[inline(always)]
    pub const fn swi36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 36"]
    #[inline(always)]
    pub fn set_swi36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Software interrupt on line 37"]
    #[inline(always)]
    pub const fn swi37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 37"]
    #[inline(always)]
    pub fn set_swi37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Software interrupt on line 38"]
    #[inline(always)]
    pub const fn swi38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Software interrupt on line 38"]
    #[inline(always)]
    pub fn set_swi38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Swier2 {
    #[inline(always)]
    fn default() -> Swier2 {
        Swier2(0)
    }
}
