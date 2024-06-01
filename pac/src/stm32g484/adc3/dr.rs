#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `RDATA` reader - Regular Data converted"]
pub type RdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular Data converted"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("rdata", &self.rdata()).finish()
    }
}
#[doc = "regular Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
