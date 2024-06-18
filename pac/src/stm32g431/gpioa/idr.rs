#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `IDR0` reader - Port input data (y = 0..15)"]
pub type Idr0R = crate::BitReader;
#[doc = "Field `IDR1` reader - Port input data (y = 0..15)"]
pub type Idr1R = crate::BitReader;
#[doc = "Field `IDR2` reader - Port input data (y = 0..15)"]
pub type Idr2R = crate::BitReader;
#[doc = "Field `IDR3` reader - Port input data (y = 0..15)"]
pub type Idr3R = crate::BitReader;
#[doc = "Field `IDR4` reader - Port input data (y = 0..15)"]
pub type Idr4R = crate::BitReader;
#[doc = "Field `IDR5` reader - Port input data (y = 0..15)"]
pub type Idr5R = crate::BitReader;
#[doc = "Field `IDR6` reader - Port input data (y = 0..15)"]
pub type Idr6R = crate::BitReader;
#[doc = "Field `IDR7` reader - Port input data (y = 0..15)"]
pub type Idr7R = crate::BitReader;
#[doc = "Field `IDR8` reader - Port input data (y = 0..15)"]
pub type Idr8R = crate::BitReader;
#[doc = "Field `IDR9` reader - Port input data (y = 0..15)"]
pub type Idr9R = crate::BitReader;
#[doc = "Field `IDR10` reader - Port input data (y = 0..15)"]
pub type Idr10R = crate::BitReader;
#[doc = "Field `IDR11` reader - Port input data (y = 0..15)"]
pub type Idr11R = crate::BitReader;
#[doc = "Field `IDR12` reader - Port input data (y = 0..15)"]
pub type Idr12R = crate::BitReader;
#[doc = "Field `IDR13` reader - Port input data (y = 0..15)"]
pub type Idr13R = crate::BitReader;
#[doc = "Field `IDR14` reader - Port input data (y = 0..15)"]
pub type Idr14R = crate::BitReader;
#[doc = "Field `IDR15` reader - Port input data (y = 0..15)"]
pub type Idr15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr0(&self) -> Idr0R {
        Idr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr1(&self) -> Idr1R {
        Idr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr2(&self) -> Idr2R {
        Idr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr3(&self) -> Idr3R {
        Idr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr4(&self) -> Idr4R {
        Idr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr5(&self) -> Idr5R {
        Idr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr6(&self) -> Idr6R {
        Idr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr7(&self) -> Idr7R {
        Idr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr8(&self) -> Idr8R {
        Idr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr9(&self) -> Idr9R {
        Idr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr10(&self) -> Idr10R {
        Idr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr11(&self) -> Idr11R {
        Idr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr12(&self) -> Idr12R {
        Idr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr13(&self) -> Idr13R {
        Idr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr14(&self) -> Idr14R {
        Idr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr15(&self) -> Idr15R {
        Idr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR")
            .field("idr15", &self.idr15())
            .field("idr14", &self.idr14())
            .field("idr13", &self.idr13())
            .field("idr12", &self.idr12())
            .field("idr11", &self.idr11())
            .field("idr10", &self.idr10())
            .field("idr9", &self.idr9())
            .field("idr8", &self.idr8())
            .field("idr7", &self.idr7())
            .field("idr6", &self.idr6())
            .field("idr5", &self.idr5())
            .field("idr4", &self.idr4())
            .field("idr3", &self.idr3())
            .field("idr2", &self.idr2())
            .field("idr1", &self.idr1())
            .field("idr0", &self.idr0())
            .finish()
    }
}
#[doc = "GPIO port input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {
    const RESET_VALUE: u32 = 0;
}
