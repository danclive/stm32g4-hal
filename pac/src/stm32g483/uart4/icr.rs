#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `PECF` writer - Parity error clear flag"]
pub type PecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag"]
pub type FecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCF` writer - Noise detected clear flag"]
pub type NcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag"]
pub type OrecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag"]
pub type IdlecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFECF` writer - TXFECF"]
pub type TxfecfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag"]
pub type TccfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBGTCF` writer - TCBGTCF"]
pub type TcbgtcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDCF` writer - LIN break detection clear flag"]
pub type LbdcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag"]
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag"]
pub type RtocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBCF` writer - End of block clear flag"]
pub type EobcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRCF` writer - UDRCF"]
pub type UdrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag"]
pub type CmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wakeup from Stop mode clear flag"]
pub type WucfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PecfW<IcrSpec> {
        PecfW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FecfW<IcrSpec> {
        FecfW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ncf(&mut self) -> NcfW<IcrSpec> {
        NcfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> OrecfW<IcrSpec> {
        OrecfW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IdlecfW<IcrSpec> {
        IdlecfW::new(self, 4)
    }
    #[doc = "Bit 5 - TXFECF"]
    #[inline(always)]
    #[must_use]
    pub fn txfecf(&mut self) -> TxfecfW<IcrSpec> {
        TxfecfW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TccfW<IcrSpec> {
        TccfW::new(self, 6)
    }
    #[doc = "Bit 7 - TCBGTCF"]
    #[inline(always)]
    #[must_use]
    pub fn tcbgtcf(&mut self) -> TcbgtcfW<IcrSpec> {
        TcbgtcfW::new(self, 7)
    }
    #[doc = "Bit 8 - LIN break detection clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LbdcfW<IcrSpec> {
        LbdcfW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CtscfW<IcrSpec> {
        CtscfW::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RtocfW<IcrSpec> {
        RtocfW::new(self, 11)
    }
    #[doc = "Bit 12 - End of block clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EobcfW<IcrSpec> {
        EobcfW::new(self, 12)
    }
    #[doc = "Bit 13 - UDRCF"]
    #[inline(always)]
    #[must_use]
    pub fn udrcf(&mut self) -> UdrcfW<IcrSpec> {
        UdrcfW::new(self, 13)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CmcfW<IcrSpec> {
        CmcfW::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WucfW<IcrSpec> {
        WucfW::new(self, 20)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
