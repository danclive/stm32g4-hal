#[doc = "Register `TXBRP` reader"]
pub type R = crate::R<TxbrpSpec>;
#[doc = "Field `TRP` reader - TRP"]
pub type TrpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBRP").field("trp", &self.trp()).finish()
    }
}
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbrpSpec;
impl crate::RegisterSpec for TxbrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrp::R`](R) reader structure"]
impl crate::Readable for TxbrpSpec {}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TxbrpSpec {
    const RESET_VALUE: u32 = 0;
}
