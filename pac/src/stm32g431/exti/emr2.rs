#[doc = "Register `EMR2` reader"]
pub type R = crate::R<Emr2Spec>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<Emr2Spec>;
#[doc = "Field `EM32` reader - Event mask on external/internal line 32"]
pub type Em32R = crate::BitReader;
#[doc = "Field `EM32` writer - Event mask on external/internal line 32"]
pub type Em32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM33` reader - Event mask on external/internal line 33"]
pub type Em33R = crate::BitReader;
#[doc = "Field `EM33` writer - Event mask on external/internal line 33"]
pub type Em33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM34` reader - Event mask on external/internal line 34"]
pub type Em34R = crate::BitReader;
#[doc = "Field `EM34` writer - Event mask on external/internal line 34"]
pub type Em34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM35` reader - Event mask on external/internal line 35"]
pub type Em35R = crate::BitReader;
#[doc = "Field `EM35` writer - Event mask on external/internal line 35"]
pub type Em35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM36` reader - Event mask on external/internal line 36"]
pub type Em36R = crate::BitReader;
#[doc = "Field `EM36` writer - Event mask on external/internal line 36"]
pub type Em36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM37` reader - Event mask on external/internal line 37"]
pub type Em37R = crate::BitReader;
#[doc = "Field `EM37` writer - Event mask on external/internal line 37"]
pub type Em37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM38` reader - Event mask on external/internal line 38"]
pub type Em38R = crate::BitReader;
#[doc = "Field `EM38` writer - Event mask on external/internal line 38"]
pub type Em38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM39` reader - Event mask on external/internal line 39"]
pub type Em39R = crate::BitReader;
#[doc = "Field `EM39` writer - Event mask on external/internal line 39"]
pub type Em39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM40` reader - Event mask on external/internal line 40"]
pub type Em40R = crate::BitReader;
#[doc = "Field `EM40` writer - Event mask on external/internal line 40"]
pub type Em40W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn em32(&self) -> Em32R {
        Em32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn em33(&self) -> Em33R {
        Em33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn em34(&self) -> Em34R {
        Em34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn em35(&self) -> Em35R {
        Em35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn em36(&self) -> Em36R {
        Em36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn em37(&self) -> Em37R {
        Em37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event mask on external/internal line 38"]
    #[inline(always)]
    pub fn em38(&self) -> Em38R {
        Em38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event mask on external/internal line 39"]
    #[inline(always)]
    pub fn em39(&self) -> Em39R {
        Em39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event mask on external/internal line 40"]
    #[inline(always)]
    pub fn em40(&self) -> Em40R {
        Em40R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR2")
            .field("em32", &self.em32())
            .field("em33", &self.em33())
            .field("em34", &self.em34())
            .field("em35", &self.em35())
            .field("em36", &self.em36())
            .field("em37", &self.em37())
            .field("em38", &self.em38())
            .field("em39", &self.em39())
            .field("em40", &self.em40())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Event mask on external/internal line 32"]
    #[inline(always)]
    pub fn em32(&mut self) -> Em32W<Emr2Spec> {
        Em32W::new(self, 0)
    }
    #[doc = "Bit 1 - Event mask on external/internal line 33"]
    #[inline(always)]
    pub fn em33(&mut self) -> Em33W<Emr2Spec> {
        Em33W::new(self, 1)
    }
    #[doc = "Bit 2 - Event mask on external/internal line 34"]
    #[inline(always)]
    pub fn em34(&mut self) -> Em34W<Emr2Spec> {
        Em34W::new(self, 2)
    }
    #[doc = "Bit 3 - Event mask on external/internal line 35"]
    #[inline(always)]
    pub fn em35(&mut self) -> Em35W<Emr2Spec> {
        Em35W::new(self, 3)
    }
    #[doc = "Bit 4 - Event mask on external/internal line 36"]
    #[inline(always)]
    pub fn em36(&mut self) -> Em36W<Emr2Spec> {
        Em36W::new(self, 4)
    }
    #[doc = "Bit 5 - Event mask on external/internal line 37"]
    #[inline(always)]
    pub fn em37(&mut self) -> Em37W<Emr2Spec> {
        Em37W::new(self, 5)
    }
    #[doc = "Bit 6 - Event mask on external/internal line 38"]
    #[inline(always)]
    pub fn em38(&mut self) -> Em38W<Emr2Spec> {
        Em38W::new(self, 6)
    }
    #[doc = "Bit 7 - Event mask on external/internal line 39"]
    #[inline(always)]
    pub fn em39(&mut self) -> Em39W<Emr2Spec> {
        Em39W::new(self, 7)
    }
    #[doc = "Bit 8 - Event mask on external/internal line 40"]
    #[inline(always)]
    pub fn em40(&mut self) -> Em40W<Emr2Spec> {
        Em40W::new(self, 8)
    }
}
#[doc = "Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emr2Spec;
impl crate::RegisterSpec for Emr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for Emr2Spec {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for Emr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for Emr2Spec {}
