#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TxbtoSpec>;
#[doc = "Field `TO` reader - TO"]
pub type ToR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - TO"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO").field("to", &self.to()).finish()
    }
}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtoSpec;
impl crate::RegisterSpec for TxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TxbtoSpec {}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TxbtoSpec {
    const RESET_VALUE: u32 = 0;
}
