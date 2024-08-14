#[doc = "Register `RTSR1` reader"]
pub type R = crate::R<Rtsr1Spec>;
#[doc = "Register `RTSR1` writer"]
pub type W = crate::W<Rtsr1Spec>;
#[doc = "Field `RT0` reader - Rising trigger event configuration of line 0"]
pub type Rt0R = crate::BitReader;
#[doc = "Field `RT0` writer - Rising trigger event configuration of line 0"]
pub type Rt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1` reader - Rising trigger event configuration of line 1"]
pub type Rt1R = crate::BitReader;
#[doc = "Field `RT1` writer - Rising trigger event configuration of line 1"]
pub type Rt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT2` reader - Rising trigger event configuration of line 2"]
pub type Rt2R = crate::BitReader;
#[doc = "Field `RT2` writer - Rising trigger event configuration of line 2"]
pub type Rt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT3` reader - Rising trigger event configuration of line 3"]
pub type Rt3R = crate::BitReader;
#[doc = "Field `RT3` writer - Rising trigger event configuration of line 3"]
pub type Rt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT4` reader - Rising trigger event configuration of line 4"]
pub type Rt4R = crate::BitReader;
#[doc = "Field `RT4` writer - Rising trigger event configuration of line 4"]
pub type Rt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT5` reader - Rising trigger event configuration of line 5"]
pub type Rt5R = crate::BitReader;
#[doc = "Field `RT5` writer - Rising trigger event configuration of line 5"]
pub type Rt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT6` reader - Rising trigger event configuration of line 6"]
pub type Rt6R = crate::BitReader;
#[doc = "Field `RT6` writer - Rising trigger event configuration of line 6"]
pub type Rt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT7` reader - Rising trigger event configuration of line 7"]
pub type Rt7R = crate::BitReader;
#[doc = "Field `RT7` writer - Rising trigger event configuration of line 7"]
pub type Rt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT8` reader - Rising trigger event configuration of line 8"]
pub type Rt8R = crate::BitReader;
#[doc = "Field `RT8` writer - Rising trigger event configuration of line 8"]
pub type Rt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT9` reader - Rising trigger event configuration of line 9"]
pub type Rt9R = crate::BitReader;
#[doc = "Field `RT9` writer - Rising trigger event configuration of line 9"]
pub type Rt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT10` reader - Rising trigger event configuration of line 10"]
pub type Rt10R = crate::BitReader;
#[doc = "Field `RT10` writer - Rising trigger event configuration of line 10"]
pub type Rt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT11` reader - Rising trigger event configuration of line 11"]
pub type Rt11R = crate::BitReader;
#[doc = "Field `RT11` writer - Rising trigger event configuration of line 11"]
pub type Rt11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT12` reader - Rising trigger event configuration of line 12"]
pub type Rt12R = crate::BitReader;
#[doc = "Field `RT12` writer - Rising trigger event configuration of line 12"]
pub type Rt12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT13` reader - Rising trigger event configuration of line 13"]
pub type Rt13R = crate::BitReader;
#[doc = "Field `RT13` writer - Rising trigger event configuration of line 13"]
pub type Rt13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT14` reader - Rising trigger event configuration of line 14"]
pub type Rt14R = crate::BitReader;
#[doc = "Field `RT14` writer - Rising trigger event configuration of line 14"]
pub type Rt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT15` reader - Rising trigger event configuration of line 15"]
pub type Rt15R = crate::BitReader;
#[doc = "Field `RT15` writer - Rising trigger event configuration of line 15"]
pub type Rt15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT16` reader - Rising trigger event configuration of line 16"]
pub type Rt16R = crate::BitReader;
#[doc = "Field `RT16` writer - Rising trigger event configuration of line 16"]
pub type Rt16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT17` reader - Rising trigger event configuration of line 17"]
pub type Rt17R = crate::BitReader;
#[doc = "Field `RT17` writer - Rising trigger event configuration of line 17"]
pub type Rt17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT19` reader - Rising trigger event configuration of line 19"]
pub type Rt19R = crate::BitReader;
#[doc = "Field `RT19` writer - Rising trigger event configuration of line 19"]
pub type Rt19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT20` reader - Rising trigger event configuration of line 20"]
pub type Rt20R = crate::BitReader;
#[doc = "Field `RT20` writer - Rising trigger event configuration of line 20"]
pub type Rt20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT21` reader - Rising trigger event configuration of line 21"]
pub type Rt21R = crate::BitReader;
#[doc = "Field `RT21` writer - Rising trigger event configuration of line 21"]
pub type Rt21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT22` reader - Rising trigger event configuration of line 22"]
pub type Rt22R = crate::BitReader;
#[doc = "Field `RT22` writer - Rising trigger event configuration of line 22"]
pub type Rt22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT` reader - RT"]
pub type RtR = crate::FieldReader;
#[doc = "Field `RT` writer - RT"]
pub type RtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn rt0(&self) -> Rt0R {
        Rt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn rt1(&self) -> Rt1R {
        Rt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn rt2(&self) -> Rt2R {
        Rt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn rt3(&self) -> Rt3R {
        Rt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn rt4(&self) -> Rt4R {
        Rt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn rt5(&self) -> Rt5R {
        Rt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn rt6(&self) -> Rt6R {
        Rt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn rt7(&self) -> Rt7R {
        Rt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn rt8(&self) -> Rt8R {
        Rt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn rt9(&self) -> Rt9R {
        Rt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn rt10(&self) -> Rt10R {
        Rt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn rt11(&self) -> Rt11R {
        Rt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn rt12(&self) -> Rt12R {
        Rt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn rt13(&self) -> Rt13R {
        Rt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn rt14(&self) -> Rt14R {
        Rt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn rt15(&self) -> Rt15R {
        Rt15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn rt16(&self) -> Rt16R {
        Rt16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn rt17(&self) -> Rt17R {
        Rt17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub fn rt19(&self) -> Rt19R {
        Rt19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration of line 20"]
    #[inline(always)]
    pub fn rt20(&self) -> Rt20R {
        Rt20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration of line 21"]
    #[inline(always)]
    pub fn rt21(&self) -> Rt21R {
        Rt21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration of line 22"]
    #[inline(always)]
    pub fn rt22(&self) -> Rt22R {
        Rt22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 29:31 - RT"]
    #[inline(always)]
    pub fn rt(&self) -> RtR {
        RtR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR1")
            .field("rt0", &self.rt0())
            .field("rt1", &self.rt1())
            .field("rt2", &self.rt2())
            .field("rt3", &self.rt3())
            .field("rt4", &self.rt4())
            .field("rt5", &self.rt5())
            .field("rt6", &self.rt6())
            .field("rt7", &self.rt7())
            .field("rt8", &self.rt8())
            .field("rt9", &self.rt9())
            .field("rt10", &self.rt10())
            .field("rt11", &self.rt11())
            .field("rt12", &self.rt12())
            .field("rt13", &self.rt13())
            .field("rt14", &self.rt14())
            .field("rt15", &self.rt15())
            .field("rt16", &self.rt16())
            .field("rt17", &self.rt17())
            .field("rt19", &self.rt19())
            .field("rt20", &self.rt20())
            .field("rt21", &self.rt21())
            .field("rt22", &self.rt22())
            .field("rt", &self.rt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> Rt0W<Rtsr1Spec> {
        Rt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> Rt1W<Rtsr1Spec> {
        Rt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> Rt2W<Rtsr1Spec> {
        Rt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> Rt3W<Rtsr1Spec> {
        Rt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> Rt4W<Rtsr1Spec> {
        Rt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> Rt5W<Rtsr1Spec> {
        Rt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> Rt6W<Rtsr1Spec> {
        Rt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> Rt7W<Rtsr1Spec> {
        Rt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rt8(&mut self) -> Rt8W<Rtsr1Spec> {
        Rt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rt9(&mut self) -> Rt9W<Rtsr1Spec> {
        Rt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rt10(&mut self) -> Rt10W<Rtsr1Spec> {
        Rt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> Rt11W<Rtsr1Spec> {
        Rt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn rt12(&mut self) -> Rt12W<Rtsr1Spec> {
        Rt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn rt13(&mut self) -> Rt13W<Rtsr1Spec> {
        Rt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn rt14(&mut self) -> Rt14W<Rtsr1Spec> {
        Rt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn rt15(&mut self) -> Rt15W<Rtsr1Spec> {
        Rt15W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn rt16(&mut self) -> Rt16W<Rtsr1Spec> {
        Rt16W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn rt17(&mut self) -> Rt17W<Rtsr1Spec> {
        Rt17W::new(self, 17)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn rt19(&mut self) -> Rt19W<Rtsr1Spec> {
        Rt19W::new(self, 19)
    }
    #[doc = "Bit 20 - Rising trigger event configuration of line 20"]
    #[inline(always)]
    #[must_use]
    pub fn rt20(&mut self) -> Rt20W<Rtsr1Spec> {
        Rt20W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising trigger event configuration of line 21"]
    #[inline(always)]
    #[must_use]
    pub fn rt21(&mut self) -> Rt21W<Rtsr1Spec> {
        Rt21W::new(self, 21)
    }
    #[doc = "Bit 22 - Rising trigger event configuration of line 22"]
    #[inline(always)]
    #[must_use]
    pub fn rt22(&mut self) -> Rt22W<Rtsr1Spec> {
        Rt22W::new(self, 22)
    }
    #[doc = "Bits 29:31 - RT"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RtW<Rtsr1Spec> {
        RtW::new(self, 29)
    }
}
#[doc = "Rising Trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtsr1Spec;
impl crate::RegisterSpec for Rtsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr1::R`](R) reader structure"]
impl crate::Readable for Rtsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtsr1::W`](W) writer structure"]
impl crate::Writable for Rtsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR1 to value 0"]
impl crate::Resettable for Rtsr1Spec {
    const RESET_VALUE: u32 = 0;
}
