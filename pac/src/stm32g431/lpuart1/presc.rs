#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PrescSpec>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PrescSpec>;
#[doc = "Field `PRESCALER` reader - PRESCALER"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - PRESCALER"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PRESCALER"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESC")
            .field("prescaler", &self.prescaler())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - PRESCALER"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<PrescSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescSpec;
impl crate::RegisterSpec for PrescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PrescSpec {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PrescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PrescSpec {
    const RESET_VALUE: u32 = 0;
}
