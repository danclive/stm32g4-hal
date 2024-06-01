#[doc = "Register `PERAR` reader"]
pub type R = crate::R<PerarSpec>;
#[doc = "Register `PERAR` writer"]
pub type W = crate::W<PerarSpec>;
#[doc = "Field `PERx` reader - Timerx Period value"]
pub type PerxR = crate::FieldReader<u16>;
#[doc = "Field `PERx` writer - Timerx Period value"]
pub type PerxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PerxR {
        PerxR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERAR").field("perx", &self.perx()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PerxW<PerarSpec> {
        PerxW::new(self, 0)
    }
}
#[doc = "Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerarSpec;
impl crate::RegisterSpec for PerarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perar::R`](R) reader structure"]
impl crate::Readable for PerarSpec {}
#[doc = "`write(|w| ..)` method takes [`perar::W`](W) writer structure"]
impl crate::Writable for PerarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERAR to value 0xffff"]
impl crate::Resettable for PerarSpec {
    const RESET_VALUE: u32 = 0xffff;
}
