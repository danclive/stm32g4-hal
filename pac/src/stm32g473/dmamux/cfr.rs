#[doc = "Register `CFR` writer"]
pub type W = crate::W<CfrSpec>;
#[doc = "Field `CSOF` writer - Clear synchronization overrun event flag"]
pub type CsofW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<CfrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof(&mut self) -> CsofW<CfrSpec> {
        CsofW::new(self, 0)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfrSpec;
impl crate::RegisterSpec for CfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CfrSpec {
    const RESET_VALUE: u32 = 0;
}
