#[doc = "Register `TR2` reader"]
pub type R = crate::R<Tr2Spec>;
#[doc = "Register `TR2` writer"]
pub type W = crate::W<Tr2Spec>;
#[doc = "Field `LT2` reader - LT2"]
pub type Lt2R = crate::FieldReader;
#[doc = "Field `LT2` writer - LT2"]
pub type Lt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HT2` reader - HT2"]
pub type Ht2R = crate::FieldReader;
#[doc = "Field `HT2` writer - HT2"]
pub type Ht2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LT2"]
    #[inline(always)]
    pub fn lt2(&self) -> Lt2R {
        Lt2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HT2"]
    #[inline(always)]
    pub fn ht2(&self) -> Ht2R {
        Ht2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR2")
            .field("ht2", &self.ht2())
            .field("lt2", &self.lt2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - LT2"]
    #[inline(always)]
    #[must_use]
    pub fn lt2(&mut self) -> Lt2W<Tr2Spec> {
        Lt2W::new(self, 0)
    }
    #[doc = "Bits 16:23 - HT2"]
    #[inline(always)]
    #[must_use]
    pub fn ht2(&mut self) -> Ht2W<Tr2Spec> {
        Ht2W::new(self, 16)
    }
}
#[doc = "watchdog threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tr2Spec;
impl crate::RegisterSpec for Tr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr2::R`](R) reader structure"]
impl crate::Readable for Tr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tr2::W`](W) writer structure"]
impl crate::Writable for Tr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR2 to value 0x00ff_0000"]
impl crate::Resettable for Tr2Spec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
