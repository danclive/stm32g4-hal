#[doc = "Register `AF2` reader"]
pub type R = crate::R<Af2Spec>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<Af2Spec>;
#[doc = "Field `OCRSEL` reader - OCREF_CLR source selection"]
pub type OcrselR = crate::FieldReader;
#[doc = "Field `OCRSEL` writer - OCREF_CLR source selection"]
pub type OcrselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&self) -> OcrselR {
        OcrselR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("ocrsel", &self.ocrsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OcrselW<'_, Af2Spec> {
        OcrselW::new(self, 16)
    }
}
#[doc = "TIM alternate function option register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af2Spec;
impl crate::RegisterSpec for Af2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for Af2Spec {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for Af2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for Af2Spec {}
