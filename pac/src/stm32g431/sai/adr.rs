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
    #[must_use]
    pub fn data(&mut self) -> DataW<AdrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "AData register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdrSpec;
impl crate::RegisterSpec for AdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adr::R`](R) reader structure"]
impl crate::Readable for AdrSpec {}
#[doc = "`write(|w| ..)` method takes [`adr::W`](W) writer structure"]
impl crate::Writable for AdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADR to value 0"]
impl crate::Resettable for AdrSpec {
    const RESET_VALUE: u32 = 0;
}
