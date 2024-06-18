#[doc = "Register `PDKEYR` writer"]
pub type W = crate::W<PdkeyrSpec>;
#[doc = "Field `PDKEYR` writer - RUN_PD in FLASH_ACR key"]
pub type PdkeyrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<PdkeyrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - RUN_PD in FLASH_ACR key"]
    #[inline(always)]
    #[must_use]
    pub fn pdkeyr(&mut self) -> PdkeyrW<PdkeyrSpec> {
        PdkeyrW::new(self, 0)
    }
}
#[doc = "Power down key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdkeyrSpec;
impl crate::RegisterSpec for PdkeyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdkeyr::W`](W) writer structure"]
impl crate::Writable for PdkeyrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDKEYR to value 0"]
impl crate::Resettable for PdkeyrSpec {
    const RESET_VALUE: u32 = 0;
}
