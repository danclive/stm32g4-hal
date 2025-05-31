#[doc = "Register `SUSP4R` reader"]
pub type R = crate::R<Susp4rSpec>;
#[doc = "Register `SUSP4R` writer"]
pub type W = crate::W<Susp4rSpec>;
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
        f.debug_struct("SUSP4R")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<Susp4rSpec> {
        SuspW::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`read`](crate::Reg::read) this register and get [`susp4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Susp4rSpec;
impl crate::RegisterSpec for Susp4rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`susp4r::R`](R) reader structure"]
impl crate::Readable for Susp4rSpec {}
#[doc = "`write(|w| ..)` method takes [`susp4r::W`](W) writer structure"]
impl crate::Writable for Susp4rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUSP4R to value 0"]
impl crate::Resettable for Susp4rSpec {}
