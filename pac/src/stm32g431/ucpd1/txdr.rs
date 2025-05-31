#[doc = "Register `TXDR` reader"]
pub type R = crate::R<TxdrSpec>;
#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TxdrSpec>;
#[doc = "Field `TXDATA` reader - TXDATA"]
pub type TxdataR = crate::FieldReader;
#[doc = "Field `TXDATA` writer - TXDATA"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TXDATA"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDR")
            .field("txdata", &self.txdata())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - TXDATA"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<TxdrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "UCPD Tx Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdrSpec;
impl crate::RegisterSpec for TxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr::R`](R) reader structure"]
impl crate::Readable for TxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TxdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TxdrSpec {}
