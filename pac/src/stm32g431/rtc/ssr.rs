#[doc = "Register `SSR` reader"]
pub type R = crate::R<SsrSpec>;
#[doc = "Field `SS` reader - Sub second value"]
pub type SsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSR").field("ss", &self.ss()).finish()
    }
}
#[doc = "sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrSpec;
impl crate::RegisterSpec for SsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssr::R`](R) reader structure"]
impl crate::Readable for SsrSpec {}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SsrSpec {}
