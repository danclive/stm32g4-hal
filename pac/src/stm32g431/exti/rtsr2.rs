#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<Rtsr2Spec>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<Rtsr2Spec>;
#[doc = "Field `RT32` reader - Rising trigger event configuration bit of line 32"]
pub type Rt32R = crate::BitReader;
#[doc = "Field `RT32` writer - Rising trigger event configuration bit of line 32"]
pub type Rt32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT33` reader - Rising trigger event configuration bit of line 32"]
pub type Rt33R = crate::BitReader;
#[doc = "Field `RT33` writer - Rising trigger event configuration bit of line 32"]
pub type Rt33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT38` reader - Rising trigger event configuration bit of line 38"]
pub type Rt38R = crate::BitReader;
#[doc = "Field `RT38` writer - Rising trigger event configuration bit of line 38"]
pub type Rt38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT39` reader - Rising trigger event configuration bit of line 39"]
pub type Rt39R = crate::BitReader;
#[doc = "Field `RT39` writer - Rising trigger event configuration bit of line 39"]
pub type Rt39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT40` reader - Rising trigger event configuration bit of line 40"]
pub type Rt40R = crate::BitReader;
#[doc = "Field `RT40` writer - Rising trigger event configuration bit of line 40"]
pub type Rt40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT41` reader - Rising trigger event configuration bit of line 41"]
pub type Rt41R = crate::BitReader;
#[doc = "Field `RT41` writer - Rising trigger event configuration bit of line 41"]
pub type Rt41W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn rt32(&self) -> Rt32R {
        Rt32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn rt33(&self) -> Rt33R {
        Rt33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn rt38(&self) -> Rt38R {
        Rt38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of line 39"]
    #[inline(always)]
    pub fn rt39(&self) -> Rt39R {
        Rt39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    pub fn rt40(&self) -> Rt40R {
        Rt40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    pub fn rt41(&self) -> Rt41R {
        Rt41R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("rt32", &self.rt32())
            .field("rt33", &self.rt33())
            .field("rt38", &self.rt38())
            .field("rt39", &self.rt39())
            .field("rt40", &self.rt40())
            .field("rt41", &self.rt41())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn rt32(&mut self) -> Rt32W<Rtsr2Spec> {
        Rt32W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 32"]
    #[inline(always)]
    #[must_use]
    pub fn rt33(&mut self) -> Rt33W<Rtsr2Spec> {
        Rt33W::new(self, 1)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    #[must_use]
    pub fn rt38(&mut self) -> Rt38W<Rtsr2Spec> {
        Rt38W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of line 39"]
    #[inline(always)]
    #[must_use]
    pub fn rt39(&mut self) -> Rt39W<Rtsr2Spec> {
        Rt39W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 40"]
    #[inline(always)]
    #[must_use]
    pub fn rt40(&mut self) -> Rt40W<Rtsr2Spec> {
        Rt40W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 41"]
    #[inline(always)]
    #[must_use]
    pub fn rt41(&mut self) -> Rt41W<Rtsr2Spec> {
        Rt41W::new(self, 9)
    }
}
#[doc = "Rising Trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtsr2Spec;
impl crate::RegisterSpec for Rtsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for Rtsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for Rtsr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for Rtsr2Spec {
    const RESET_VALUE: u32 = 0;
}
