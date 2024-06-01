#[doc = "Register `DAC_MCR` reader"]
pub type R = crate::R<DacMcrSpec>;
#[doc = "Register `DAC_MCR` writer"]
pub type W = crate::W<DacMcrSpec>;
#[doc = "Field `MODE1` reader - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMADOUBLE1` reader - DAC Channel1 DMA double data mode"]
pub type Dmadouble1R = crate::BitReader;
#[doc = "Field `DMADOUBLE1` writer - DAC Channel1 DMA double data mode"]
pub type Dmadouble1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINFORMAT1` reader - Enable signed format for DAC channel1"]
pub type Sinformat1R = crate::BitReader;
#[doc = "Field `SINFORMAT1` writer - Enable signed format for DAC channel1"]
pub type Sinformat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFSEL` reader - High frequency interface mode selection"]
pub type HfselR = crate::FieldReader;
#[doc = "Field `HFSEL` writer - High frequency interface mode selection"]
pub type HfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE2` reader - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
pub type Mode2R = crate::FieldReader;
#[doc = "Field `MODE2` writer - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMADOUBLE2` reader - DAC Channel2 DMA double data mode"]
pub type Dmadouble2R = crate::BitReader;
#[doc = "Field `DMADOUBLE2` writer - DAC Channel2 DMA double data mode"]
pub type Dmadouble2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINFORMAT2` reader - Enable signed format for DAC channel2"]
pub type Sinformat2R = crate::BitReader;
#[doc = "Field `SINFORMAT2` writer - Enable signed format for DAC channel2"]
pub type Sinformat2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&self) -> Dmadouble1R {
        Dmadouble1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&self) -> Sinformat1R {
        Sinformat1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&self) -> HfselR {
        HfselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&self) -> Dmadouble2R {
        Dmadouble2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&self) -> Sinformat2R {
        Sinformat2R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_MCR")
            .field("mode1", &self.mode1())
            .field("dmadouble1", &self.dmadouble1())
            .field("sinformat1", &self.sinformat1())
            .field("hfsel", &self.hfsel())
            .field("mode2", &self.mode2())
            .field("dmadouble2", &self.dmadouble2())
            .field("sinformat2", &self.sinformat2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp;amp; hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<DacMcrSpec> {
        Mode1W::new(self, 0)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmadouble1(&mut self) -> Dmadouble1W<DacMcrSpec> {
        Dmadouble1W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    #[must_use]
    pub fn sinformat1(&mut self) -> Sinformat1W<DacMcrSpec> {
        Sinformat1W::new(self, 9)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn hfsel(&mut self) -> HfselW<DacMcrSpec> {
        HfselW::new(self, 14)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp;amp; hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> Mode2W<DacMcrSpec> {
        Mode2W::new(self, 16)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmadouble2(&mut self) -> Dmadouble2W<DacMcrSpec> {
        Dmadouble2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    #[must_use]
    pub fn sinformat2(&mut self) -> Sinformat2W<DacMcrSpec> {
        Sinformat2W::new(self, 25)
    }
}
#[doc = "DAC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacMcrSpec;
impl crate::RegisterSpec for DacMcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_mcr::R`](R) reader structure"]
impl crate::Readable for DacMcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_mcr::W`](W) writer structure"]
impl crate::Writable for DacMcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_MCR to value 0"]
impl crate::Resettable for DacMcrSpec {
    const RESET_VALUE: u32 = 0;
}
