#[doc = "Register `CALFACT` reader"]
pub type R = crate::R<CalfactSpec>;
#[doc = "Register `CALFACT` writer"]
pub type W = crate::W<CalfactSpec>;
#[doc = "Field `CALFACT_S` reader - CALFACT_S"]
pub type CalfactSR = crate::FieldReader;
#[doc = "Field `CALFACT_S` writer - CALFACT_S"]
pub type CalfactSW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CALFACT_D` reader - CALFACT_D"]
pub type CalfactDR = crate::FieldReader;
#[doc = "Field `CALFACT_D` writer - CALFACT_D"]
pub type CalfactDW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CalfactSR {
        CalfactSR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CalfactDR {
        CalfactDR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT")
            .field("calfact_d", &self.calfact_d())
            .field("calfact_s", &self.calfact_s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - CALFACT_S"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_s(&mut self) -> CalfactSW<CalfactSpec> {
        CalfactSW::new(self, 0)
    }
    #[doc = "Bits 16:22 - CALFACT_D"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_d(&mut self) -> CalfactDW<CalfactSpec> {
        CalfactDW::new(self, 16)
    }
}
#[doc = "Calibration Factors\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalfactSpec;
impl crate::RegisterSpec for CalfactSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact::R`](R) reader structure"]
impl crate::Readable for CalfactSpec {}
#[doc = "`write(|w| ..)` method takes [`calfact::W`](W) writer structure"]
impl crate::Writable for CalfactSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CalfactSpec {
    const RESET_VALUE: u32 = 0;
}
