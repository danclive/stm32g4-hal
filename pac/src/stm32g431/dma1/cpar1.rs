#[doc = "Register `CPAR1` reader"]
pub type R = crate::R<Cpar1Spec>;
#[doc = "Register `CPAR1` writer"]
pub type W = crate::W<Cpar1Spec>;
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
        f.debug_struct("CPAR1").field("pa", &self.pa()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, Cpar1Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpar1Spec;
impl crate::RegisterSpec for Cpar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar1::R`](R) reader structure"]
impl crate::Readable for Cpar1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpar1::W`](W) writer structure"]
impl crate::Writable for Cpar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPAR1 to value 0"]
impl crate::Resettable for Cpar1Spec {}
