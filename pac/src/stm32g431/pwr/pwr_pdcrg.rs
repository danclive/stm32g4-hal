#[doc = "Register `PWR_PDCRG` reader"]
pub type R = crate::R<PwrPdcrgSpec>;
#[doc = "Register `PWR_PDCRG` writer"]
pub type W = crate::W<PwrPdcrgSpec>;
#[doc = "Field `PD0` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd2R = crate::BitReader;
#[doc = "Field `PD2` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd4R = crate::BitReader;
#[doc = "Field `PD4` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd5R = crate::BitReader;
#[doc = "Field `PD5` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd6R = crate::BitReader;
#[doc = "Field `PD6` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd7R = crate::BitReader;
#[doc = "Field `PD7` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd8R = crate::BitReader;
#[doc = "Field `PD8` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd9R = crate::BitReader;
#[doc = "Field `PD9` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd10R = crate::BitReader;
#[doc = "Field `PD10` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
pub type Pd10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_PDCRG")
            .field("pd0", &self.pd0())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .field("pd3", &self.pd3())
            .field("pd4", &self.pd4())
            .field("pd5", &self.pd5())
            .field("pd6", &self.pd6())
            .field("pd7", &self.pd7())
            .field("pd8", &self.pd8())
            .field("pd9", &self.pd9())
            .field("pd10", &self.pd10())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd0(&mut self) -> Pd0W<'_, PwrPdcrgSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd1(&mut self) -> Pd1W<'_, PwrPdcrgSpec> {
        Pd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd2(&mut self) -> Pd2W<'_, PwrPdcrgSpec> {
        Pd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd3(&mut self) -> Pd3W<'_, PwrPdcrgSpec> {
        Pd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd4(&mut self) -> Pd4W<'_, PwrPdcrgSpec> {
        Pd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd5(&mut self) -> Pd5W<'_, PwrPdcrgSpec> {
        Pd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd6(&mut self) -> Pd6W<'_, PwrPdcrgSpec> {
        Pd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd7(&mut self) -> Pd7W<'_, PwrPdcrgSpec> {
        Pd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd8(&mut self) -> Pd8W<'_, PwrPdcrgSpec> {
        Pd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd9(&mut self) -> Pd9W<'_, PwrPdcrgSpec> {
        Pd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\\[y\\] when APC bit is set in PWR_CR3 register."]
    #[inline(always)]
    pub fn pd10(&mut self) -> Pd10W<'_, PwrPdcrgSpec> {
        Pd10W::new(self, 10)
    }
}
#[doc = "Power Port G pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_pdcrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pdcrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrPdcrgSpec;
impl crate::RegisterSpec for PwrPdcrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pdcrg::R`](R) reader structure"]
impl crate::Readable for PwrPdcrgSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_pdcrg::W`](W) writer structure"]
impl crate::Writable for PwrPdcrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_PDCRG to value 0"]
impl crate::Resettable for PwrPdcrgSpec {}
