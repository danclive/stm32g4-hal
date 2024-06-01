#[doc = "Register `SKR` writer"]
pub type W = crate::W<SkrSpec>;
#[doc = "Field `KEY` writer - SRAM2 Key for software erase"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<SkrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - SRAM2 Key for software erase"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<SkrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "SRAM2 Key Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SkrSpec;
impl crate::RegisterSpec for SkrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`skr::W`](W) writer structure"]
impl crate::Writable for SkrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SkrSpec {
    const RESET_VALUE: u32 = 0;
}
