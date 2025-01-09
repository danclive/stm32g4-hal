#[doc = "Register `TX_ORDSET` reader"]
pub type R = crate::R<TxOrdsetSpec>;
#[doc = "Register `TX_ORDSET` writer"]
pub type W = crate::W<TxOrdsetSpec>;
#[doc = "Field `TXORDSET` reader - TXORDSET"]
pub type TxordsetR = crate::FieldReader<u32>;
#[doc = "Field `TXORDSET` writer - TXORDSET"]
pub type TxordsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&self) -> TxordsetR {
        TxordsetR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ORDSET")
            .field("txordset", &self.txordset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&mut self) -> TxordsetW<TxOrdsetSpec> {
        TxordsetW::new(self, 0)
    }
}
#[doc = "UCPD Tx Ordered Set Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ordset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ordset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxOrdsetSpec;
impl crate::RegisterSpec for TxOrdsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ordset::R`](R) reader structure"]
impl crate::Readable for TxOrdsetSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_ordset::W`](W) writer structure"]
impl crate::Writable for TxOrdsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_ORDSET to value 0"]
impl crate::Resettable for TxOrdsetSpec {
    const RESET_VALUE: u32 = 0;
}
