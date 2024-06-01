#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TxbcfSpec>;
#[doc = "Field `CF` reader - CF"]
pub type CfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - CF"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCF").field("cf", &self.cf()).finish()
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcfSpec;
impl crate::RegisterSpec for TxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TxbcfSpec {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TxbcfSpec {
    const RESET_VALUE: u32 = 0;
}
