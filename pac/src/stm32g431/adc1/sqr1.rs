#[doc = "Register `SQR1` reader"]
pub type R = crate::R<Sqr1Spec>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<Sqr1Spec>;
#[doc = "Field `L` reader - Regular channel sequence length"]
pub type LR = crate::FieldReader;
#[doc = "Field `L` writer - Regular channel sequence length"]
pub type LW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQ1` reader - SQ1"]
pub type Sq1R = crate::FieldReader;
#[doc = "Field `SQ1` writer - SQ1"]
pub type Sq1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ2` reader - SQ2"]
pub type Sq2R = crate::FieldReader;
#[doc = "Field `SQ2` writer - SQ2"]
pub type Sq2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ3` reader - SQ3"]
pub type Sq3R = crate::FieldReader;
#[doc = "Field `SQ3` writer - SQ3"]
pub type Sq3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ4` reader - SQ4"]
pub type Sq4R = crate::FieldReader;
#[doc = "Field `SQ4` writer - SQ4"]
pub type Sq4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> Sq1R {
        Sq1R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> Sq2R {
        Sq2R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> Sq3R {
        Sq3R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> Sq4R {
        Sq4R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SQR1")
            .field("sq4", &self.sq4())
            .field("sq3", &self.sq3())
            .field("sq2", &self.sq2())
            .field("sq1", &self.sq1())
            .field("l", &self.l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<Sqr1Spec> {
        LW::new(self, 0)
    }
    #[doc = "Bits 6:10 - SQ1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> Sq1W<Sqr1Spec> {
        Sq1W::new(self, 6)
    }
    #[doc = "Bits 12:16 - SQ2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> Sq2W<Sqr1Spec> {
        Sq2W::new(self, 12)
    }
    #[doc = "Bits 18:22 - SQ3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> Sq3W<Sqr1Spec> {
        Sq3W::new(self, 18)
    }
    #[doc = "Bits 24:28 - SQ4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> Sq4W<Sqr1Spec> {
        Sq4W::new(self, 24)
    }
}
#[doc = "regular sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sqr1Spec;
impl crate::RegisterSpec for Sqr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr1::R`](R) reader structure"]
impl crate::Readable for Sqr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sqr1::W`](W) writer structure"]
impl crate::Writable for Sqr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for Sqr1Spec {
    const RESET_VALUE: u32 = 0;
}
