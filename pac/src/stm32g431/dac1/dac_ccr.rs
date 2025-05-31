#[doc = "Register `DAC_CCR` reader"]
pub type R = crate::R<DacCcrSpec>;
#[doc = "Register `DAC_CCR` writer"]
pub type W = crate::W<DacCcrSpec>;
#[doc = "Field `OTRIM1` reader - DAC Channel 1 offset trimming value"]
pub type Otrim1R = crate::FieldReader;
#[doc = "Field `OTRIM1` writer - DAC Channel 1 offset trimming value"]
pub type Otrim1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OTRIM2` reader - DAC Channel 2 offset trimming value"]
pub type Otrim2R = crate::FieldReader;
#[doc = "Field `OTRIM2` writer - DAC Channel 2 offset trimming value"]
pub type Otrim2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub fn otrim1(&self) -> Otrim1R {
        Otrim1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub fn otrim2(&self) -> Otrim2R {
        Otrim2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CCR")
            .field("otrim1", &self.otrim1())
            .field("otrim2", &self.otrim2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub fn otrim1(&mut self) -> Otrim1W<DacCcrSpec> {
        Otrim1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub fn otrim2(&mut self) -> Otrim2W<DacCcrSpec> {
        Otrim2W::new(self, 16)
    }
}
#[doc = "DAC calibration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCcrSpec;
impl crate::RegisterSpec for DacCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_ccr::R`](R) reader structure"]
impl crate::Readable for DacCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dac_ccr::W`](W) writer structure"]
impl crate::Writable for DacCcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_CCR to value 0"]
impl crate::Resettable for DacCcrSpec {}
