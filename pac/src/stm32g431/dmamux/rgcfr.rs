#[doc = "Register `RGCFR` writer"]
pub type W = crate::W<RgcfrSpec>;
#[doc = "Field `COF` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type CofW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl core::fmt::Debug for crate::generic::Reg<RgcfrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    pub fn cof(&mut self) -> CofW<RgcfrSpec> {
        CofW::new(self, 0)
    }
}
#[doc = "DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgcfrSpec;
impl crate::RegisterSpec for RgcfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure"]
impl crate::Writable for RgcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RgcfrSpec {}
