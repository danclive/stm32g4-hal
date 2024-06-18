#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RDR` reader - Receive data value"]
pub type RdrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Receive data value"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDR").field("rdr", &self.rdr()).finish()
    }
}
#[doc = "Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {
    const RESET_VALUE: u32 = 0;
}
