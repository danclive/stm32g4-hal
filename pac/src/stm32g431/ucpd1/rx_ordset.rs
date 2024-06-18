#[doc = "Register `RX_ORDSET` reader"]
pub type R = crate::R<RxOrdsetSpec>;
#[doc = "Field `RXORDSET` reader - RXORDSET"]
pub type RxordsetR = crate::FieldReader;
#[doc = "Field `RXSOP3OF4` reader - RXSOP3OF4"]
pub type Rxsop3of4R = crate::BitReader;
#[doc = "Field `RXSOPKINVALID` reader - RXSOPKINVALID"]
pub type RxsopkinvalidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - RXORDSET"]
    #[inline(always)]
    pub fn rxordset(&self) -> RxordsetR {
        RxordsetR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RXSOP3OF4"]
    #[inline(always)]
    pub fn rxsop3of4(&self) -> Rxsop3of4R {
        Rxsop3of4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RXSOPKINVALID"]
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RxsopkinvalidR {
        RxsopkinvalidR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDSET")
            .field("rxordset", &self.rxordset())
            .field("rxsop3of4", &self.rxsop3of4())
            .field("rxsopkinvalid", &self.rxsopkinvalid())
            .finish()
    }
}
#[doc = "UCPD Rx Ordered Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ordset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOrdsetSpec;
impl crate::RegisterSpec for RxOrdsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordset::R`](R) reader structure"]
impl crate::Readable for RxOrdsetSpec {}
#[doc = "`reset()` method sets RX_ORDSET to value 0"]
impl crate::Resettable for RxOrdsetSpec {
    const RESET_VALUE: u32 = 0;
}
