#[doc = "Register `SUSP1R` reader"]
pub type R = crate::R<Susp1rSpec>;
#[doc = "Register `SUSP1R` writer"]
pub type W = crate::W<Susp1rSpec>;
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
        f.debug_struct("SUSP1R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, Susp1rSpec> {
        SuspW::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`read`](crate::Reg::read) this register and get [`susp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Susp1rSpec;
impl crate::RegisterSpec for Susp1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp1r::R`](R) reader structure"]
impl crate::Readable for Susp1rSpec {}
#[doc = "`write(|w| ..)` method takes [`susp1r::W`](W) writer structure"]
impl crate::Writable for Susp1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUSP1R to value 0"]
impl crate::Resettable for Susp1rSpec {}
