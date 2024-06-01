#[doc = "Register `DAC_SHHR` reader"]
pub type R = crate::R<DacShhrSpec>;
#[doc = "Register `DAC_SHHR` writer"]
pub type W = crate::W<DacShhrSpec>;
#[doc = "Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold1R = crate::FieldReader<u16>;
#[doc = "Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `THOLD2` reader - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold2R = crate::FieldReader<u16>;
#[doc = "Field `THOLD2` writer - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold1(&self) -> Thold1R {
        Thold1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold2(&self) -> Thold2R {
        Thold2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SHHR")
            .field("thold1", &self.thold1())
            .field("thold2", &self.thold2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in sample &amp;amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> Thold1W<DacShhrSpec> {
        Thold1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time (only valid in sample &amp;amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn thold2(&mut self) -> Thold2W<DacShhrSpec> {
        Thold2W::new(self, 16)
    }
}
#[doc = "DAC Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_shhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_shhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacShhrSpec;
impl crate::RegisterSpec for DacShhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shhr::R`](R) reader structure"]
impl crate::Readable for DacShhrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_shhr::W`](W) writer structure"]
impl crate::Writable for DacShhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SHHR to value 0x0001_0001"]
impl crate::Resettable for DacShhrSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
