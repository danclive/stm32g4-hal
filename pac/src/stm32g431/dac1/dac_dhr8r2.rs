#[doc = "Register `DAC_DHR8R2` reader"]
pub type R = crate::R<DacDhr8r2Spec>;
#[doc = "Register `DAC_DHR8R2` writer"]
pub type W = crate::W<DacDhr8r2Spec>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type Dacc2dhrR = crate::FieldReader;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DACC2DHRB` reader - DAC channel2 8-bit right-aligned data"]
pub type Dacc2dhrbR = crate::FieldReader;
#[doc = "Field `DACC2DHRB` writer - DAC channel2 8-bit right-aligned data"]
pub type Dacc2dhrbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&self) -> Dacc2dhrbR {
        Dacc2dhrbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR8R2")
            .field("dacc2dhr", &self.dacc2dhr())
            .field("dacc2dhrb", &self.dacc2dhrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<'_, DacDhr8r2Spec> {
        Dacc2dhrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&mut self) -> Dacc2dhrbW<'_, DacDhr8r2Spec> {
        Dacc2dhrbW::new(self, 8)
    }
}
#[doc = "DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dhr8r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr8r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr8r2Spec;
impl crate::RegisterSpec for DacDhr8r2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr8r2::R`](R) reader structure"]
impl crate::Readable for DacDhr8r2Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr8r2::W`](W) writer structure"]
impl crate::Writable for DacDhr8r2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_DHR8R2 to value 0"]
impl crate::Resettable for DacDhr8r2Spec {}
