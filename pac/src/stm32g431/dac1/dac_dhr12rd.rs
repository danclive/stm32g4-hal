#[doc = "Register `DAC_DHR12RD` reader"]
pub type R = crate::R<DacDhr12rdSpec>;
#[doc = "Register `DAC_DHR12RD` writer"]
pub type W = crate::W<DacDhr12rdSpec>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type Dacc1dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
pub type Dacc1dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
pub type Dacc2dhrR = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
pub type Dacc2dhrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> Dacc1dhrR {
        Dacc1dhrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> Dacc2dhrR {
        Dacc2dhrR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_DHR12RD")
            .field("dacc1dhr", &self.dacc1dhr())
            .field("dacc2dhr", &self.dacc2dhr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> Dacc1dhrW<DacDhr12rdSpec> {
        Dacc1dhrW::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> Dacc2dhrW<DacDhr12rdSpec> {
        Dacc2dhrW::new(self, 16)
    }
}
#[doc = "Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_dhr12rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_dhr12rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacDhr12rdSpec;
impl crate::RegisterSpec for DacDhr12rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr12rd::R`](R) reader structure"]
impl crate::Readable for DacDhr12rdSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr12rd::W`](W) writer structure"]
impl crate::Writable for DacDhr12rdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_DHR12RD to value 0"]
impl crate::Resettable for DacDhr12rdSpec {}
