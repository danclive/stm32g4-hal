#[doc = "Register `DAC_SHSR2` reader"]
pub type R = crate::R<DacShsr2Spec>;
#[doc = "Register `DAC_SHSR2` writer"]
pub type W = crate::W<DacShsr2Spec>;
#[doc = "Field `TSAMPLE2` reader - DAC Channel 2 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, if BWSTx=1, the write operation is ignored."]
pub type Tsample2R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE2` writer - DAC Channel 2 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, if BWSTx=1, the write operation is ignored."]
pub type Tsample2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, if BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn tsample2(&self) -> Tsample2R {
        Tsample2R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SHSR2")
            .field("tsample2", &self.tsample2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time (only valid in sample &amp; hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWSTx of DAC_SR register is low, if BWSTx=1, the write operation is ignored."]
    #[inline(always)]
    pub fn tsample2(&mut self) -> Tsample2W<'_, DacShsr2Spec> {
        Tsample2W::new(self, 0)
    }
}
#[doc = "DAC Sample and Hold sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_shsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacShsr2Spec;
impl crate::RegisterSpec for DacShsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shsr2::R`](R) reader structure"]
impl crate::Readable for DacShsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_shsr2::W`](W) writer structure"]
impl crate::Writable for DacShsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_SHSR2 to value 0"]
impl crate::Resettable for DacShsr2Spec {}
