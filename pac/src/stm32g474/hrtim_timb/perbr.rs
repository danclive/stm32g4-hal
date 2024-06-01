#[doc = "Register `PERBR` reader"]
pub type R = crate::R<PerbrSpec>;
#[doc = "Register `PERBR` writer"]
pub type W = crate::W<PerbrSpec>;
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
        f.debug_struct("PERBR").field("perx", &self.perx()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    #[must_use]
    pub fn perx(&mut self) -> PerxW<PerbrSpec> {
        PerxW::new(self, 0)
    }
}
#[doc = "Timerx Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbrSpec;
impl crate::RegisterSpec for PerbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perbr::R`](R) reader structure"]
impl crate::Readable for PerbrSpec {}
#[doc = "`write(|w| ..)` method takes [`perbr::W`](W) writer structure"]
impl crate::Writable for PerbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERBR to value 0xffff"]
impl crate::Resettable for PerbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
