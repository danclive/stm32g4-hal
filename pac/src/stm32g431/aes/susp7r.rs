#[doc = "Register `SUSP7R` reader"]
pub type R = crate::R<Susp7rSpec>;
#[doc = "Register `SUSP7R` writer"]
pub type W = crate::W<Susp7rSpec>;
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
        f.debug_struct("SUSP7R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, Susp7rSpec> {
        SuspW::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`read`](crate::Reg::read) this register and get [`susp7r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp7r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Susp7rSpec;
impl crate::RegisterSpec for Susp7rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp7r::R`](R) reader structure"]
impl crate::Readable for Susp7rSpec {}
#[doc = "`write(|w| ..)` method takes [`susp7r::W`](W) writer structure"]
impl crate::Writable for Susp7rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUSP7R to value 0"]
impl crate::Resettable for Susp7rSpec {}
