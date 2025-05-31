#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RgsrSpec>;
#[doc = "Field `OF` reader - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
pub type OfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGSR").field("of", &self.of()).finish()
    }
}
#[doc = "DMAMux - DMA request generator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgsrSpec;
impl crate::RegisterSpec for RgsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RgsrSpec {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RgsrSpec {}
