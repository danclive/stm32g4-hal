#[doc = "Register `PERDR` reader"]
pub type R = crate::R<PerdrSpec>;
#[doc = "Register `PERDR` writer"]
pub type W = crate::W<PerdrSpec>;
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
        f.debug_struct("PERDR").field("perx", &self.perx()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PerxW<PerdrSpec> {
        PerxW::new(self, 0)
    }
}
#[doc = "Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerdrSpec;
impl crate::RegisterSpec for PerdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perdr::R`](R) reader structure"]
impl crate::Readable for PerdrSpec {}
#[doc = "`write(|w| ..)` method takes [`perdr::W`](W) writer structure"]
impl crate::Writable for PerdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERDR to value 0xffff"]
impl crate::Resettable for PerdrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
