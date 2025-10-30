#[doc = "Register `TX_PAYSZ` reader"]
pub type R = crate::R<TxPayszSpec>;
#[doc = "Register `TX_PAYSZ` writer"]
pub type W = crate::W<TxPayszSpec>;
#[doc = "Field `TXPAYSZ` reader - TXPAYSZ"]
pub type TxpayszR = crate::FieldReader<u16>;
#[doc = "Field `TXPAYSZ` writer - TXPAYSZ"]
pub type TxpayszW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    pub fn txpaysz(&self) -> TxpayszR {
        TxpayszR::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PAYSZ")
            .field("txpaysz", &self.txpaysz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    pub fn txpaysz(&mut self) -> TxpayszW<'_, TxPayszSpec> {
        TxpayszW::new(self, 0)
    }
}
#[doc = "UCPD Tx Paysize Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_paysz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_paysz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPayszSpec;
impl crate::RegisterSpec for TxPayszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_paysz::R`](R) reader structure"]
impl crate::Readable for TxPayszSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_paysz::W`](W) writer structure"]
impl crate::Writable for TxPayszSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_PAYSZ to value 0"]
impl crate::Resettable for TxPayszSpec {}
