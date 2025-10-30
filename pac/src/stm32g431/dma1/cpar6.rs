#[doc = "Register `CPAR6` reader"]
pub type R = crate::R<Cpar6Spec>;
#[doc = "Register `CPAR6` writer"]
pub type W = crate::W<Cpar6Spec>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPAR6").field("pa", &self.pa()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, Cpar6Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpar6Spec;
impl crate::RegisterSpec for Cpar6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar6::R`](R) reader structure"]
impl crate::Readable for Cpar6Spec {}
#[doc = "`write(|w| ..)` method takes [`cpar6::W`](W) writer structure"]
impl crate::Writable for Cpar6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPAR6 to value 0"]
impl crate::Resettable for Cpar6Spec {}
