#[doc = "Register `DAC_SHRR` reader"]
pub type R = crate::R<DacShrrSpec>;
#[doc = "Register `DAC_SHRR` writer"]
pub type W = crate::W<DacShrrSpec>;
#[doc = "Field `TREFRESH1` reader - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh1R = crate::FieldReader;
#[doc = "Field `TREFRESH1` writer - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TREFRESH2` reader - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh2R = crate::FieldReader;
#[doc = "Field `TREFRESH2` writer - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type Trefresh2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh1(&self) -> Trefresh1R {
        Trefresh1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh2(&self) -> Trefresh2R {
        Trefresh2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SHRR")
            .field("trefresh1", &self.trefresh1())
            .field("trefresh2", &self.trefresh2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn trefresh1(&mut self) -> Trefresh1W<DacShrrSpec> {
        Trefresh1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn trefresh2(&mut self) -> Trefresh2W<DacShrrSpec> {
        Trefresh2W::new(self, 16)
    }
}
#[doc = "DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_shrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_shrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacShrrSpec;
impl crate::RegisterSpec for DacShrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_shrr::R`](R) reader structure"]
impl crate::Readable for DacShrrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_shrr::W`](W) writer structure"]
impl crate::Writable for DacShrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SHRR to value 0x0001_0001"]
impl crate::Resettable for DacShrrSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
