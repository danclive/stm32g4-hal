#[doc = "Register `DAC_DHR8R1` reader"]
pub type R = crate::R<DacDhr8r1Spec>;
#[doc = "Register `DAC_DHR8R1` writer"]
pub type W = crate::W<DacDhr8r1Spec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
pub type Dacc1dhrR = crate::FieldReader;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DACC1DHRB` reader - DAC channel1 8-bit right-aligned data"]
pub type Dacc1dhrbR = crate::FieldReader;
#[doc = "Field `DACC1DHRB` writer - DAC channel1 8-bit right-aligned data"]
pub type Dacc1dhrbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> Dacc1dhrbR {
        Dacc1dhrbR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR8R1")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<DacDhr8r1Spec> {
        Dacc1dhrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhrb(&mut self) -> Dacc1dhrbW<DacDhr8r1Spec> {
        Dacc1dhrbW::new(self, 8)
    }
}
#[doc = "DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr8r1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr8r1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr8r1Spec;
impl crate::RegisterSpec for DacDhr8r1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr8r1::R`](R) reader structure"]
impl crate::Readable for DacDhr8r1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr8r1::W`](W) writer structure"]
impl crate::Writable for DacDhr8r1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR8R1 to value 0"]
impl crate::Resettable for DacDhr8r1Spec {
    const RESET_VALUE: u32 = 0;
}
