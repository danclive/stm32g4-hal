#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TxbcSpec>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TxbcSpec>;
#[doc = "Field `TFQM` reader - TFQM"]
pub type TfqmR = crate::BitReader;
#[doc = "Field `TFQM` writer - TFQM"]
pub type TfqmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - TFQM"]
    #[inline(always)]
    pub fn tfqm(&self) -> TfqmR {
        TfqmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBC").field("tfqm", &self.tfqm()).finish()
    }
}
impl W {
    #[doc = "Bit 24 - TFQM"]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TfqmW<TxbcSpec> {
        TfqmW::new(self, 24)
    }
}
#[doc = "FDCAN Tx Buffer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcSpec;
impl crate::RegisterSpec for TxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TxbcSpec {
    const RESET_VALUE: u32 = 0;
}
