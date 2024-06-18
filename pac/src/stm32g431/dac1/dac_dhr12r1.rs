#[doc = "Register `DAC_DHR12R1` reader"]
pub type R = crate::R<DacDhr12r1Spec>;
#[doc = "Register `DAC_DHR12R1` writer"]
pub type W = crate::W<DacDhr12r1Spec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type Dacc1dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC1DHRB` reader - DAC channel1 12-bit right-aligned data B"]
pub type Dacc1dhrbR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHRB` writer - DAC channel1 12-bit right-aligned data B"]
pub type Dacc1dhrbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B"]
    #[inline(always)]
    pub fn dacc1dhrb(&self) -> Dacc1dhrbR {
        Dacc1dhrbR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR12R1")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc1dhrb", &self.dacc1dhrb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<DacDhr12r1Spec> {
        Dacc1dhrW::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC channel1 12-bit right-aligned data B"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhrb(&mut self) -> Dacc1dhrbW<DacDhr12r1Spec> {
        Dacc1dhrbW::new(self, 16)
    }
}
#[doc = "DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dhr12r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr12r1Spec;
impl crate::RegisterSpec for DacDhr12r1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr12r1::R`](R) reader structure"]
impl crate::Readable for DacDhr12r1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr12r1::W`](W) writer structure"]
impl crate::Writable for DacDhr12r1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR12R1 to value 0"]
impl crate::Resettable for DacDhr12r1Spec {
    const RESET_VALUE: u32 = 0;
}
