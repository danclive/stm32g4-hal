#[doc = "Register `TXBCR` reader"]
pub type R = crate::R<TxbcrSpec>;
#[doc = "Register `TXBCR` writer"]
pub type W = crate::W<TxbcrSpec>;
#[doc = "Field `CR` reader - CR"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - CR"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCR").field("cr", &self.cr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - CR"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CrW<TxbcrSpec> {
        CrW::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcrSpec;
impl crate::RegisterSpec for TxbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcr::R`](R) reader structure"]
impl crate::Readable for TxbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`txbcr::W`](W) writer structure"]
impl crate::Writable for TxbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TxbcrSpec {
    const RESET_VALUE: u32 = 0;
}
