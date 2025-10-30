#[doc = "Register `FDCAN_ILS` reader"]
pub type R = crate::R<FdcanIlsSpec>;
#[doc = "Register `FDCAN_ILS` writer"]
pub type W = crate::W<FdcanIlsSpec>;
#[doc = "Field `RXFIFO0` reader - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
pub type Rxfifo0R = crate::BitReader;
#[doc = "Field `RXFIFO0` writer - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
pub type Rxfifo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO1` reader - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
pub type Rxfifo1R = crate::BitReader;
#[doc = "Field `RXFIFO1` writer - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
pub type Rxfifo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSG` reader - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
pub type SmsgR = crate::BitReader;
#[doc = "Field `SMSG` writer - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
pub type SmsgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFERR` reader - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
pub type TferrR = crate::BitReader;
#[doc = "Field `TFERR` writer - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
pub type TferrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC` reader - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
pub type MiscR = crate::BitReader;
#[doc = "Field `MISC` writer - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
pub type MiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
    #[inline(always)]
    pub fn rxfifo0(&self) -> Rxfifo0R {
        Rxfifo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
    #[inline(always)]
    pub fn rxfifo1(&self) -> Rxfifo1R {
        Rxfifo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
    #[inline(always)]
    pub fn smsg(&self) -> SmsgR {
        SmsgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
    #[inline(always)]
    pub fn tferr(&self) -> TferrR {
        TferrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
    #[inline(always)]
    pub fn misc(&self) -> MiscR {
        MiscR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILS")
            .field("rxfifo0", &self.rxfifo0())
            .field("rxfifo1", &self.rxfifo1())
            .field("smsg", &self.smsg())
            .field("tferr", &self.tferr())
            .field("misc", &self.misc())
            .field("berr", &self.berr())
            .field("perr", &self.perr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption RF0LL: Rx FIFO 0 message lost interrupt line RF0FL: Rx FIFO 0 full interrupt line RF0NL: Rx FIFO 0 new message interrupt line"]
    #[inline(always)]
    pub fn rxfifo0(&mut self) -> Rxfifo0W<'_, FdcanIlsSpec> {
        Rxfifo0W::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption RF1LL: Rx FIFO 1 message lost interrupt line RF1FL: Rx FIFO 1 full interrupt line RF1NL: Rx FIFO 1 new message interrupt line"]
    #[inline(always)]
    pub fn rxfifo1(&mut self) -> Rxfifo1W<'_, FdcanIlsSpec> {
        Rxfifo1W::new(self, 1)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption TCFL: Transmission cancellation finished interrupt line TCL: Transmission completed interrupt line HPML: High-priority message interrupt line"]
    #[inline(always)]
    pub fn smsg(&mut self) -> SmsgW<'_, FdcanIlsSpec> {
        SmsgW::new(self, 2)
    }
    #[doc = "Bit 3 - Tx FIFO ERROR grouping the following interruption TEFLL: Tx event FIFO element lost interrupt line TEFFL: Tx event FIFO full interrupt line TEFNL: Tx event FIFO new entry interrupt line TFEL: Tx FIFO empty interrupt line"]
    #[inline(always)]
    pub fn tferr(&mut self) -> TferrW<'_, FdcanIlsSpec> {
        TferrW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption TOOL: Timeout occurred interrupt line MRAFL: Message RAM access failure interrupt line TSWL: Timestamp wraparound interrupt line"]
    #[inline(always)]
    pub fn misc(&mut self) -> MiscW<'_, FdcanIlsSpec> {
        MiscW::new(self, 4)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption EPL Error passive interrupt line ELOL: Error logging overflow interrupt line"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, FdcanIlsSpec> {
        BerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption ARAL: Access to reserved address line PEDL: Protocol error in data phase line PEAL: Protocol error in arbitration phase line WDIL: Watchdog interrupt line BOL: Bus_Off status EWL: Warning status interrupt line"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, FdcanIlsSpec> {
        PerrW::new(self, 6)
    }
}
#[doc = "FDCAN interrupt line select register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIlsSpec;
impl crate::RegisterSpec for FdcanIlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ils::R`](R) reader structure"]
impl crate::Readable for FdcanIlsSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ils::W`](W) writer structure"]
impl crate::Writable for FdcanIlsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_ILS to value 0"]
impl crate::Resettable for FdcanIlsSpec {}
