#[doc = "Register `SUSP2R` reader"]
pub type R = crate::R<Susp2rSpec>;
#[doc = "Register `SUSP2R` writer"]
pub type W = crate::W<Susp2rSpec>;
#[doc = "Field `SUSP` reader - AES suspend"]
pub type SuspR = crate::FieldReader<u32>;
#[doc = "Field `SUSP` writer - AES suspend"]
pub type SuspW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUSP2R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SuspW<Susp2rSpec> {
        SuspW::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Susp2rSpec;
impl crate::RegisterSpec for Susp2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp2r::R`](R) reader structure"]
impl crate::Readable for Susp2rSpec {}
#[doc = "`write(|w| ..)` method takes [`susp2r::W`](W) writer structure"]
impl crate::Writable for Susp2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP2R to value 0"]
impl crate::Resettable for Susp2rSpec {
    const RESET_VALUE: u32 = 0;
}
