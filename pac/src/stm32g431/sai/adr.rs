#[doc = "Register `ADR` reader"]
pub type R = crate::R<AdrSpec>;
#[doc = "Register `ADR` writer"]
pub type W = crate::W<AdrSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADR").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, AdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "AData register\n\nYou can [`read`](crate::Reg::read) this register and get [`adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdrSpec;
impl crate::RegisterSpec for AdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adr::R`](R) reader structure"]
impl crate::Readable for AdrSpec {}
#[doc = "`write(|w| ..)` method takes [`adr::W`](W) writer structure"]
impl crate::Writable for AdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADR to value 0"]
impl crate::Resettable for AdrSpec {}
