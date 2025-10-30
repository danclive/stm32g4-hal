#[doc = "Register `DAC_DHR12L2` reader"]
pub type R = crate::R<DacDhr12l2Spec>;
#[doc = "Register `DAC_DHR12L2` writer"]
pub type W = crate::W<DacDhr12l2Spec>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2."]
pub type Dacc2dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2."]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHRB` reader - DAC channel2 12-bit left-aligned data B"]
pub type Dacc2dhrbR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHRB` writer - DAC channel2 12-bit left-aligned data B"]
pub type Dacc2dhrbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data B"]
    #[inline(always)]
    pub fn dacc2dhrb(&self) -> Dacc2dhrbR {
        Dacc2dhrbR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR12L2")
            .field("dacc2dhr", &self.dacc2dhr())
            .field("dacc2dhrb", &self.dacc2dhrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<'_, DacDhr12l2Spec> {
        Dacc2dhrW::new(self, 4)
    }
    #[doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data B"]
    #[inline(always)]
    pub fn dacc2dhrb(&mut self) -> Dacc2dhrbW<'_, DacDhr12l2Spec> {
        Dacc2dhrbW::new(self, 20)
    }
}
#[doc = "DAC channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dhr12l2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12l2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr12l2Spec;
impl crate::RegisterSpec for DacDhr12l2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr12l2::R`](R) reader structure"]
impl crate::Readable for DacDhr12l2Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr12l2::W`](W) writer structure"]
impl crate::Writable for DacDhr12l2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_DHR12L2 to value 0"]
impl crate::Resettable for DacDhr12l2Spec {}
