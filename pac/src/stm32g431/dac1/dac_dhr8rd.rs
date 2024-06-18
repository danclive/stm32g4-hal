#[doc = "Register `DAC_DHR8RD` reader"]
pub type R = crate::R<DacDhr8rdSpec>;
#[doc = "Register `DAC_DHR8RD` writer"]
pub type W = crate::W<DacDhr8rdSpec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
pub type Dacc1dhrR = crate::FieldReader;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type Dacc2dhrR = crate::FieldReader;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR8RD")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<DacDhr8rdSpec> {
        Dacc1dhrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<DacDhr8rdSpec> {
        Dacc2dhrW::new(self, 8)
    }
}
#[doc = "DUAL DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dhr8rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr8rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr8rdSpec;
impl crate::RegisterSpec for DacDhr8rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr8rd::R`](R) reader structure"]
impl crate::Readable for DacDhr8rdSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr8rd::W`](W) writer structure"]
impl crate::Writable for DacDhr8rdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR8RD to value 0"]
impl crate::Resettable for DacDhr8rdSpec {
    const RESET_VALUE: u32 = 0;
}
