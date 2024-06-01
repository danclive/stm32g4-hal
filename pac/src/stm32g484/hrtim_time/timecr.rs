#[doc = "Register `TIMECR` reader"]
pub type R = crate::R<TimecrSpec>;
#[doc = "Register `TIMECR` writer"]
pub type W = crate::W<TimecrSpec>;
#[doc = "Field `CK_PSCx` reader - HRTIM Timer x Clock prescaler"]
pub type CkPscxR = crate::FieldReader;
#[doc = "Field `CK_PSCx` writer - HRTIM Timer x Clock prescaler"]
pub type CkPscxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONT` reader - Continuous mode"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous mode"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRIG` reader - Re-triggerable mode"]
pub type RetrigR = crate::BitReader;
#[doc = "Field `RETRIG` writer - Re-triggerable mode"]
pub type RetrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HalfR = crate::BitReader;
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HalfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSHPLL` reader - Push-Pull mode enable"]
pub type PshpllR = crate::BitReader;
#[doc = "Field `PSHPLL` writer - Push-Pull mode enable"]
pub type PshpllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTLVD` reader - Interleaved mode"]
pub type IntlvdR = crate::FieldReader;
#[doc = "Field `INTLVD` writer - Interleaved mode"]
pub type IntlvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSYNCU` reader - Re-Synchronized Update"]
pub type RsyncuR = crate::BitReader;
#[doc = "Field `RSYNCU` writer - Re-Synchronized Update"]
pub type RsyncuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCRSTx` reader - Synchronization Resets Timer x"]
pub type SyncrstxR = crate::BitReader;
#[doc = "Field `SYNCRSTx` writer - Synchronization Resets Timer x"]
pub type SyncrstxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCSTRTx` reader - Synchronization Starts Timer x"]
pub type SyncstrtxR = crate::BitReader;
#[doc = "Field `SYNCSTRTx` writer - Synchronization Starts Timer x"]
pub type SyncstrtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELCMP2` reader - Delayed CMP2 mode"]
pub type Delcmp2R = crate::FieldReader;
#[doc = "Field `DELCMP2` writer - Delayed CMP2 mode"]
pub type Delcmp2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELCMP4` reader - Delayed CMP4 mode"]
pub type Delcmp4R = crate::FieldReader;
#[doc = "Field `DELCMP4` writer - Delayed CMP4 mode"]
pub type Delcmp4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TFU` reader - TFU"]
pub type TfuR = crate::BitReader;
#[doc = "Field `TFU` writer - TFU"]
pub type TfuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxREPU` reader - Timer x Repetition update"]
pub type TxRepuR = crate::BitReader;
#[doc = "Field `TxREPU` writer - Timer x Repetition update"]
pub type TxRepuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxRSTU` reader - Timerx reset update"]
pub type TxRstuR = crate::BitReader;
#[doc = "Field `TxRSTU` writer - Timerx reset update"]
pub type TxRstuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAU` reader - TAU"]
pub type TauR = crate::BitReader;
#[doc = "Field `TAU` writer - TAU"]
pub type TauW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBU` reader - TBU"]
pub type TbuR = crate::BitReader;
#[doc = "Field `TBU` writer - TBU"]
pub type TbuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCU` reader - TCU"]
pub type TcuR = crate::BitReader;
#[doc = "Field `TCU` writer - TCU"]
pub type TcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDU` reader - TDU"]
pub type TduR = crate::BitReader;
#[doc = "Field `TDU` writer - TDU"]
pub type TduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTU` reader - Master Timer update"]
pub type MstuR = crate::BitReader;
#[doc = "Field `MSTU` writer - Master Timer update"]
pub type MstuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DacsyncR = crate::FieldReader;
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DacsyncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PreenR = crate::BitReader;
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PreenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDGAT` reader - Update Gating"]
pub type UpdgatR = crate::FieldReader;
#[doc = "Field `UPDGAT` writer - Update Gating"]
pub type UpdgatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ck_pscx(&self) -> CkPscxR {
        CkPscxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RetrigR {
        RetrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HalfR {
        HalfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PshpllR {
        PshpllR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&self) -> IntlvdR {
        IntlvdR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Re-Synchronized Update"]
    #[inline(always)]
    pub fn rsyncu(&self) -> RsyncuR {
        RsyncuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SyncrstxR {
        SyncrstxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SyncstrtxR {
        SyncstrtxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> Delcmp2R {
        Delcmp2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> Delcmp4R {
        Delcmp4R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - TFU"]
    #[inline(always)]
    pub fn tfu(&self) -> TfuR {
        TfuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TxRepuR {
        TxRepuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TxRstuR {
        TxRstuR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TAU"]
    #[inline(always)]
    pub fn tau(&self) -> TauR {
        TauR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TbuR {
        TbuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TcuR {
        TcuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TduR {
        TduR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MstuR {
        MstuR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DacsyncR {
        DacsyncR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PreenR {
        PreenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UpdgatR {
        UpdgatR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMECR")
            .field("updgat", &self.updgat())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("mstu", &self.mstu())
            .field("tdu", &self.tdu())
            .field("tcu", &self.tcu())
            .field("tbu", &self.tbu())
            .field("tau", &self.tau())
            .field("tx_rstu", &self.tx_rstu())
            .field("tx_repu", &self.tx_repu())
            .field("tfu", &self.tfu())
            .field("delcmp4", &self.delcmp4())
            .field("delcmp2", &self.delcmp2())
            .field("syncstrtx", &self.syncstrtx())
            .field("syncrstx", &self.syncrstx())
            .field("rsyncu", &self.rsyncu())
            .field("intlvd", &self.intlvd())
            .field("pshpll", &self.pshpll())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ck_pscx", &self.ck_pscx())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ck_pscx(&mut self) -> CkPscxW<TimecrSpec> {
        CkPscxW::new(self, 0)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<TimecrSpec> {
        ContW::new(self, 3)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RetrigW<TimecrSpec> {
        RetrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HalfW<TimecrSpec> {
        HalfW::new(self, 5)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pshpll(&mut self) -> PshpllW<TimecrSpec> {
        PshpllW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Interleaved mode"]
    #[inline(always)]
    #[must_use]
    pub fn intlvd(&mut self) -> IntlvdW<TimecrSpec> {
        IntlvdW::new(self, 7)
    }
    #[doc = "Bit 9 - Re-Synchronized Update"]
    #[inline(always)]
    #[must_use]
    pub fn rsyncu(&mut self) -> RsyncuW<TimecrSpec> {
        RsyncuW::new(self, 9)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstx(&mut self) -> SyncrstxW<TimecrSpec> {
        SyncrstxW::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtx(&mut self) -> SyncstrtxW<TimecrSpec> {
        SyncstrtxW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp2(&mut self) -> Delcmp2W<TimecrSpec> {
        Delcmp2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp4(&mut self) -> Delcmp4W<TimecrSpec> {
        Delcmp4W::new(self, 14)
    }
    #[doc = "Bit 16 - TFU"]
    #[inline(always)]
    #[must_use]
    pub fn tfu(&mut self) -> TfuW<TimecrSpec> {
        TfuW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_repu(&mut self) -> TxRepuW<TimecrSpec> {
        TxRepuW::new(self, 17)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rstu(&mut self) -> TxRstuW<TimecrSpec> {
        TxRstuW::new(self, 18)
    }
    #[doc = "Bit 19 - TAU"]
    #[inline(always)]
    #[must_use]
    pub fn tau(&mut self) -> TauW<TimecrSpec> {
        TauW::new(self, 19)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TbuW<TimecrSpec> {
        TbuW::new(self, 20)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    #[must_use]
    pub fn tcu(&mut self) -> TcuW<TimecrSpec> {
        TcuW::new(self, 21)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    #[must_use]
    pub fn tdu(&mut self) -> TduW<TimecrSpec> {
        TduW::new(self, 22)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    #[must_use]
    pub fn mstu(&mut self) -> MstuW<TimecrSpec> {
        MstuW::new(self, 24)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DacsyncW<TimecrSpec> {
        DacsyncW::new(self, 25)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PreenW<TimecrSpec> {
        PreenW::new(self, 27)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    #[must_use]
    pub fn updgat(&mut self) -> UpdgatW<TimecrSpec> {
        UpdgatW::new(self, 28)
    }
}
#[doc = "Timerx Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimecrSpec;
impl crate::RegisterSpec for TimecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timecr::R`](R) reader structure"]
impl crate::Readable for TimecrSpec {}
#[doc = "`write(|w| ..)` method takes [`timecr::W`](W) writer structure"]
impl crate::Writable for TimecrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMECR to value 0"]
impl crate::Resettable for TimecrSpec {
    const RESET_VALUE: u32 = 0;
}
