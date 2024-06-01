#[doc = "Register `RX_PAYSZ` reader"]
pub type R = crate::R<RxPayszSpec>;
#[doc = "Field `RXPAYSZ` reader - RXPAYSZ"]
pub type RxpayszR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RxpayszR {
        RxpayszR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PAYSZ")
            .field("rxpaysz", &self.rxpaysz())
            .finish()
    }
}
#[doc = "UCPD Rx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_paysz::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPayszSpec;
impl crate::RegisterSpec for RxPayszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_paysz::R`](R) reader structure"]
impl crate::Readable for RxPayszSpec {}
#[doc = "`reset()` method sets RX_PAYSZ to value 0"]
impl crate::Resettable for RxPayszSpec {
    const RESET_VALUE: u32 = 0;
}
