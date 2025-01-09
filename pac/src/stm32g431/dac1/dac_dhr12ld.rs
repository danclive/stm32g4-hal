#[doc = "Register `DAC_DHR12LD` reader"]
pub type R = crate::R<DacDhr12ldSpec>;
#[doc = "Register `DAC_DHR12LD` writer"]
pub type W = crate::W<DacDhr12ldSpec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type Dacc1dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
pub type Dacc2dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR12LD")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<DacDhr12ldSpec> {
        Dacc1dhrW::new(self, 4)
    }
    #[doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<DacDhr12ldSpec> {
        Dacc2dhrW::new(self, 20)
    }
}
#[doc = "DUAL DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dhr12ld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12ld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr12ldSpec;
impl crate::RegisterSpec for DacDhr12ldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr12ld::R`](R) reader structure"]
impl crate::Readable for DacDhr12ldSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr12ld::W`](W) writer structure"]
impl crate::Writable for DacDhr12ldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR12LD to value 0"]
impl crate::Resettable for DacDhr12ldSpec {
    const RESET_VALUE: u32 = 0;
}
