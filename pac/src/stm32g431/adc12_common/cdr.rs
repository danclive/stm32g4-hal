#[doc = "Register `CDR` reader"]
pub type R = crate::R<CdrSpec>;
#[doc = "Field `RDATA_MST` reader - Regular data of the master ADC"]
pub type RdataMstR = crate::FieldReader<u16>;
#[doc = "Field `RDATA_SLV` reader - Regular data of the slave ADC"]
pub type RdataSlvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data of the master ADC"]
    #[inline(always)]
    pub fn rdata_mst(&self) -> RdataMstR {
        RdataMstR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Regular data of the slave ADC"]
    #[inline(always)]
    pub fn rdata_slv(&self) -> RdataSlvR {
        RdataSlvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR")
            .field("rdata_slv", &self.rdata_slv())
            .field("rdata_mst", &self.rdata_mst())
            .finish()
    }
}
#[doc = "ADC common regular data register for dual and triple modes\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdrSpec;
impl crate::RegisterSpec for CdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr::R`](R) reader structure"]
impl crate::Readable for CdrSpec {}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CdrSpec {}
