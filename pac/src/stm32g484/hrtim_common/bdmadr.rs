#[doc = "Register `BDMADR` writer"]
pub type W = crate::W<BdmadrSpec>;
#[doc = "Field `BDMADR` writer - Burst DMA Data register"]
pub type BdmadrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<BdmadrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    #[must_use]
    pub fn bdmadr(&mut self) -> BdmadrW<BdmadrSpec> {
        BdmadrW::new(self, 0)
    }
}
#[doc = "Burst DMA Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmadr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdmadrSpec;
impl crate::RegisterSpec for BdmadrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bdmadr::W`](W) writer structure"]
impl crate::Writable for BdmadrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDMADR to value 0"]
impl crate::Resettable for BdmadrSpec {
    const RESET_VALUE: u32 = 0;
}
