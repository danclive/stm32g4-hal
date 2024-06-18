#[doc = "Register `RQR` writer"]
pub type W = crate::W<RqrSpec>;
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SbkrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MmrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` writer - TXFRQ"]
pub type TxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RqrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    #[must_use]
    pub fn sbkrq(&mut self) -> SbkrqW<RqrSpec> {
        SbkrqW::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mmrq(&mut self) -> MmrqW<RqrSpec> {
        MmrqW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrq(&mut self) -> RxfrqW<RqrSpec> {
        RxfrqW::new(self, 3)
    }
    #[doc = "Bit 4 - TXFRQ"]
    #[inline(always)]
    #[must_use]
    pub fn txfrq(&mut self) -> TxfrqW<RqrSpec> {
        TxfrqW::new(self, 4)
    }
}
#[doc = "Request register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RqrSpec;
impl crate::RegisterSpec for RqrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RqrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RqrSpec {
    const RESET_VALUE: u32 = 0;
}
