#[doc = "Register `DAC_STMODR` reader"]
pub type R = crate::R<DacStmodrSpec>;
#[doc = "Register `DAC_STMODR` writer"]
pub type W = crate::W<DacStmodrSpec>;
#[doc = "Field `STRSTTRIGSEL1` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type Strsttrigsel1R = crate::FieldReader;
#[doc = "Field `STRSTTRIGSEL1` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type Strsttrigsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STINCTRIGSEL1` reader - DAC Channel 1 Sawtooth Increment trigger selection"]
pub type Stinctrigsel1R = crate::FieldReader;
#[doc = "Field `STINCTRIGSEL1` writer - DAC Channel 1 Sawtooth Increment trigger selection"]
pub type Stinctrigsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STRSTTRIGSEL2` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type Strsttrigsel2R = crate::FieldReader;
#[doc = "Field `STRSTTRIGSEL2` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub type Strsttrigsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STINCTRIGSEL2` reader - DAC Channel 2 Sawtooth Increment trigger selection"]
pub type Stinctrigsel2R = crate::FieldReader;
#[doc = "Field `STINCTRIGSEL2` writer - DAC Channel 2 Sawtooth Increment trigger selection"]
pub type Stinctrigsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&self) -> Strsttrigsel1R {
        Strsttrigsel1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&self) -> Stinctrigsel1R {
        Stinctrigsel1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&self) -> Strsttrigsel2R {
        Strsttrigsel2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&self) -> Stinctrigsel2R {
        Stinctrigsel2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_STMODR")
            .field("strsttrigsel1", &self.strsttrigsel1())
            .field("stinctrigsel1", &self.stinctrigsel1())
            .field("strsttrigsel2", &self.strsttrigsel2())
            .field("stinctrigsel2", &self.stinctrigsel2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&mut self) -> Strsttrigsel1W<'_, DacStmodrSpec> {
        Strsttrigsel1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&mut self) -> Stinctrigsel1W<'_, DacStmodrSpec> {
        Stinctrigsel1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&mut self) -> Strsttrigsel2W<'_, DacStmodrSpec> {
        Strsttrigsel2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&mut self) -> Stinctrigsel2W<'_, DacStmodrSpec> {
        Stinctrigsel2W::new(self, 24)
    }
}
#[doc = "Sawtooth Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_stmodr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_stmodr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacStmodrSpec;
impl crate::RegisterSpec for DacStmodrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_stmodr::R`](R) reader structure"]
impl crate::Readable for DacStmodrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_stmodr::W`](W) writer structure"]
impl crate::Writable for DacStmodrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_STMODR to value 0"]
impl crate::Resettable for DacStmodrSpec {}
