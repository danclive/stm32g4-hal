#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp0r(pub u32);
impl Bkp0r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp0r {
    #[inline(always)]
    fn default() -> Bkp0r {
        Bkp0r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp10r(pub u32);
impl Bkp10r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp10r {
    #[inline(always)]
    fn default() -> Bkp10r {
        Bkp10r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp11r(pub u32);
impl Bkp11r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp11r {
    #[inline(always)]
    fn default() -> Bkp11r {
        Bkp11r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp12r(pub u32);
impl Bkp12r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp12r {
    #[inline(always)]
    fn default() -> Bkp12r {
        Bkp12r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp13r(pub u32);
impl Bkp13r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp13r {
    #[inline(always)]
    fn default() -> Bkp13r {
        Bkp13r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp14r(pub u32);
impl Bkp14r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp14r {
    #[inline(always)]
    fn default() -> Bkp14r {
        Bkp14r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp15r(pub u32);
impl Bkp15r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp15r {
    #[inline(always)]
    fn default() -> Bkp15r {
        Bkp15r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp16r(pub u32);
impl Bkp16r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp16r {
    #[inline(always)]
    fn default() -> Bkp16r {
        Bkp16r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp17r(pub u32);
impl Bkp17r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp17r {
    #[inline(always)]
    fn default() -> Bkp17r {
        Bkp17r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp18r(pub u32);
impl Bkp18r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp18r {
    #[inline(always)]
    fn default() -> Bkp18r {
        Bkp18r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp19r(pub u32);
impl Bkp19r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp19r {
    #[inline(always)]
    fn default() -> Bkp19r {
        Bkp19r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp1r(pub u32);
impl Bkp1r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp1r {
    #[inline(always)]
    fn default() -> Bkp1r {
        Bkp1r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp20r(pub u32);
impl Bkp20r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp20r {
    #[inline(always)]
    fn default() -> Bkp20r {
        Bkp20r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp21r(pub u32);
impl Bkp21r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp21r {
    #[inline(always)]
    fn default() -> Bkp21r {
        Bkp21r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp22r(pub u32);
impl Bkp22r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp22r {
    #[inline(always)]
    fn default() -> Bkp22r {
        Bkp22r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp23r(pub u32);
impl Bkp23r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp23r {
    #[inline(always)]
    fn default() -> Bkp23r {
        Bkp23r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp24r(pub u32);
impl Bkp24r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp24r {
    #[inline(always)]
    fn default() -> Bkp24r {
        Bkp24r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp25r(pub u32);
impl Bkp25r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp25r {
    #[inline(always)]
    fn default() -> Bkp25r {
        Bkp25r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp26r(pub u32);
impl Bkp26r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp26r {
    #[inline(always)]
    fn default() -> Bkp26r {
        Bkp26r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp27r(pub u32);
impl Bkp27r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp27r {
    #[inline(always)]
    fn default() -> Bkp27r {
        Bkp27r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp28r(pub u32);
impl Bkp28r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp28r {
    #[inline(always)]
    fn default() -> Bkp28r {
        Bkp28r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp29r(pub u32);
impl Bkp29r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp29r {
    #[inline(always)]
    fn default() -> Bkp29r {
        Bkp29r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp2r(pub u32);
impl Bkp2r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp2r {
    #[inline(always)]
    fn default() -> Bkp2r {
        Bkp2r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp30r(pub u32);
impl Bkp30r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp30r {
    #[inline(always)]
    fn default() -> Bkp30r {
        Bkp30r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp31r(pub u32);
impl Bkp31r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp31r {
    #[inline(always)]
    fn default() -> Bkp31r {
        Bkp31r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp3r(pub u32);
impl Bkp3r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp3r {
    #[inline(always)]
    fn default() -> Bkp3r {
        Bkp3r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp4r(pub u32);
impl Bkp4r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp4r {
    #[inline(always)]
    fn default() -> Bkp4r {
        Bkp4r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp5r(pub u32);
impl Bkp5r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp5r {
    #[inline(always)]
    fn default() -> Bkp5r {
        Bkp5r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp6r(pub u32);
impl Bkp6r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp6r {
    #[inline(always)]
    fn default() -> Bkp6r {
        Bkp6r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp7r(pub u32);
impl Bkp7r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp7r {
    #[inline(always)]
    fn default() -> Bkp7r {
        Bkp7r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp8r(pub u32);
impl Bkp8r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp8r {
    #[inline(always)]
    fn default() -> Bkp8r {
        Bkp8r(0)
    }
}
#[doc = "TAMP backup register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp9r(pub u32);
impl Bkp9r {
    #[doc = "BKP"]
    #[inline(always)]
    pub const fn bkp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BKP"]
    #[inline(always)]
    pub fn set_bkp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bkp9r {
    #[inline(always)]
    fn default() -> Bkp9r {
        Bkp9r(0)
    }
}
#[doc = "control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "TAMP1E"]
    #[inline(always)]
    pub const fn tamp1e(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1E"]
    #[inline(always)]
    pub fn set_tamp1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TAMP2E"]
    #[inline(always)]
    pub const fn tamp2e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2E"]
    #[inline(always)]
    pub fn set_tamp2e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TAMP2E"]
    #[inline(always)]
    pub const fn tamp3e(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2E"]
    #[inline(always)]
    pub fn set_tamp3e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITAMP3E"]
    #[inline(always)]
    pub const fn itamp3e(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP3E"]
    #[inline(always)]
    pub fn set_itamp3e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ITAMP4E"]
    #[inline(always)]
    pub const fn itamp4e(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP4E"]
    #[inline(always)]
    pub fn set_itamp4e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ITAMP5E"]
    #[inline(always)]
    pub const fn itamp5e(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP5E"]
    #[inline(always)]
    pub fn set_itamp5e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ITAMP6E"]
    #[inline(always)]
    pub const fn itamp6e(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP6E"]
    #[inline(always)]
    pub fn set_itamp6e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
    #[doc = "TAMP1NOER"]
    #[inline(always)]
    pub const fn tamp1noer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1NOER"]
    #[inline(always)]
    pub fn set_tamp1noer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TAMP2NOER"]
    #[inline(always)]
    pub const fn tamp2noer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2NOER"]
    #[inline(always)]
    pub fn set_tamp2noer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TAMP3NOER"]
    #[inline(always)]
    pub const fn tamp3noer(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP3NOER"]
    #[inline(always)]
    pub fn set_tamp3noer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TAMP1MSK"]
    #[inline(always)]
    pub const fn tamp1msk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1MSK"]
    #[inline(always)]
    pub fn set_tamp1msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TAMP2MSK"]
    #[inline(always)]
    pub const fn tamp2msk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2MSK"]
    #[inline(always)]
    pub fn set_tamp2msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TAMP3MSK"]
    #[inline(always)]
    pub const fn tamp3msk(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP3MSK"]
    #[inline(always)]
    pub fn set_tamp3msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TAMP1TRG"]
    #[inline(always)]
    pub const fn tamp1trg(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1TRG"]
    #[inline(always)]
    pub fn set_tamp1trg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TAMP2TRG"]
    #[inline(always)]
    pub const fn tamp2trg(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2TRG"]
    #[inline(always)]
    pub fn set_tamp2trg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "TAMP3TRG"]
    #[inline(always)]
    pub const fn tamp3trg(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP3TRG"]
    #[inline(always)]
    pub fn set_tamp3trg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
#[doc = "TAMP filter control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fltcr(pub u32);
impl Fltcr {
    #[doc = "TAMPFREQ"]
    #[inline(always)]
    pub const fn tampfreq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "TAMPFREQ"]
    #[inline(always)]
    pub fn set_tampfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "TAMPFLT"]
    #[inline(always)]
    pub const fn tampflt(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "TAMPFLT"]
    #[inline(always)]
    pub fn set_tampflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "TAMPPRCH"]
    #[inline(always)]
    pub const fn tampprch(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "TAMPPRCH"]
    #[inline(always)]
    pub fn set_tampprch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "TAMPPUDIS"]
    #[inline(always)]
    pub const fn tamppudis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TAMPPUDIS"]
    #[inline(always)]
    pub fn set_tamppudis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Fltcr {
    #[inline(always)]
    fn default() -> Fltcr {
        Fltcr(0)
    }
}
#[doc = "TAMP interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "TAMP1IE"]
    #[inline(always)]
    pub const fn tamp1ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1IE"]
    #[inline(always)]
    pub fn set_tamp1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TAMP2IE"]
    #[inline(always)]
    pub const fn tamp2ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2IE"]
    #[inline(always)]
    pub fn set_tamp2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TAMP3IE"]
    #[inline(always)]
    pub const fn tamp3ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP3IE"]
    #[inline(always)]
    pub fn set_tamp3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITAMP3IE"]
    #[inline(always)]
    pub const fn itamp3ie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP3IE"]
    #[inline(always)]
    pub fn set_itamp3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ITAMP4IE"]
    #[inline(always)]
    pub const fn itamp4ie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP4IE"]
    #[inline(always)]
    pub fn set_itamp4ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ITAMP5IE"]
    #[inline(always)]
    pub const fn itamp5ie(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP5IE"]
    #[inline(always)]
    pub fn set_itamp5ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ITAMP6IE"]
    #[inline(always)]
    pub const fn itamp6ie(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP6IE"]
    #[inline(always)]
    pub fn set_itamp6ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
#[doc = "TAMP masked interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misr(pub u32);
impl Misr {
    #[doc = "TAMP1MF:"]
    #[inline(always)]
    pub const fn tamp1mf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1MF:"]
    #[inline(always)]
    pub fn set_tamp1mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TAMP2MF"]
    #[inline(always)]
    pub const fn tamp2mf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2MF"]
    #[inline(always)]
    pub fn set_tamp2mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TAMP3MF"]
    #[inline(always)]
    pub const fn tamp3mf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP3MF"]
    #[inline(always)]
    pub fn set_tamp3mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITAMP3MF"]
    #[inline(always)]
    pub const fn itamp3mf(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP3MF"]
    #[inline(always)]
    pub fn set_itamp3mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ITAMP4MF"]
    #[inline(always)]
    pub const fn itamp4mf(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP4MF"]
    #[inline(always)]
    pub fn set_itamp4mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ITAMP5MF"]
    #[inline(always)]
    pub const fn itamp5mf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP5MF"]
    #[inline(always)]
    pub fn set_itamp5mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ITAMP6MF"]
    #[inline(always)]
    pub const fn itamp6mf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP6MF"]
    #[inline(always)]
    pub fn set_itamp6mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Misr {
    #[inline(always)]
    fn default() -> Misr {
        Misr(0)
    }
}
#[doc = "TAMP status clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "CTAMP1F"]
    #[inline(always)]
    pub const fn ctamp1f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CTAMP1F"]
    #[inline(always)]
    pub fn set_ctamp1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CTAMP2F"]
    #[inline(always)]
    pub const fn ctamp2f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CTAMP2F"]
    #[inline(always)]
    pub fn set_ctamp2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTAMP3F"]
    #[inline(always)]
    pub const fn ctamp3f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTAMP3F"]
    #[inline(always)]
    pub fn set_ctamp3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CITAMP3F"]
    #[inline(always)]
    pub const fn citamp3f(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CITAMP3F"]
    #[inline(always)]
    pub fn set_citamp3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "CITAMP4F"]
    #[inline(always)]
    pub const fn citamp4f(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "CITAMP4F"]
    #[inline(always)]
    pub fn set_citamp4f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CITAMP5F"]
    #[inline(always)]
    pub const fn citamp5f(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CITAMP5F"]
    #[inline(always)]
    pub fn set_citamp5f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CITAMP6F"]
    #[inline(always)]
    pub const fn citamp6f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CITAMP6F"]
    #[inline(always)]
    pub fn set_citamp6f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
#[doc = "TAMP status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "TAMP1F"]
    #[inline(always)]
    pub const fn tamp1f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP1F"]
    #[inline(always)]
    pub fn set_tamp1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TAMP2F"]
    #[inline(always)]
    pub const fn tamp2f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP2F"]
    #[inline(always)]
    pub fn set_tamp2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TAMP3F"]
    #[inline(always)]
    pub const fn tamp3f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TAMP3F"]
    #[inline(always)]
    pub fn set_tamp3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITAMP3F"]
    #[inline(always)]
    pub const fn itamp3f(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP3F"]
    #[inline(always)]
    pub fn set_itamp3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ITAMP4F"]
    #[inline(always)]
    pub const fn itamp4f(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP4F"]
    #[inline(always)]
    pub fn set_itamp4f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ITAMP5F"]
    #[inline(always)]
    pub const fn itamp5f(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP5F"]
    #[inline(always)]
    pub fn set_itamp5f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ITAMP6F"]
    #[inline(always)]
    pub const fn itamp6f(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ITAMP6F"]
    #[inline(always)]
    pub fn set_itamp6f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
