#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RxdrSpec>;
#[doc = "Field `RXDATA` reader - RXDATA"]
pub type RxdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RXDATA"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[doc = "UCPD Rx Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdrSpec;
impl crate::RegisterSpec for RxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RxdrSpec {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RxdrSpec {
    const RESET_VALUE: u32 = 0;
}
