#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RdataSpec>;
#[doc = "Field `RDATA` reader - RDATA"]
pub type RdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA")
            .field("rdata", &self.rdata())
            .finish()
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
