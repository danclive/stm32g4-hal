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
    pub fn key(&mut self) -> KeyW<SkrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "SRAM2 Key Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SkrSpec;
impl crate::RegisterSpec for SkrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`skr::W`](W) writer structure"]
impl crate::Writable for SkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SkrSpec {}
