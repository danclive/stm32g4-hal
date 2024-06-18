#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RdataSpec>;
#[doc = "Field `RES` reader - RES"]
pub type ResR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RES"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA").field("res", &self.res()).finish()
    }
}
#[doc = "FMAC Read Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdataSpec;
impl crate::RegisterSpec for RdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RdataSpec {}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RdataSpec {
    const RESET_VALUE: u32 = 0;
}
