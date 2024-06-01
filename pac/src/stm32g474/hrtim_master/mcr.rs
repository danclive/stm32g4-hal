#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `CK_PSC` reader - HRTIM Master Clock prescaler"]
pub type CkPscR = crate::FieldReader;
#[doc = "Field `CK_PSC` writer - HRTIM Master Clock prescaler"]
pub type CkPscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONT` reader - Master Continuous mode"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Master Continuous mode"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRIG` reader - Master Re-triggerable mode"]
pub type RetrigR = crate::BitReader;
#[doc = "Field `RETRIG` writer - Master Re-triggerable mode"]
pub type RetrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HalfR = crate::BitReader;
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HalfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTLVD` reader - Interleaved mode"]
pub type IntlvdR = crate::FieldReader;
#[doc = "Field `INTLVD` writer - Interleaved mode"]
pub type IntlvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC_IN` reader - synchronization input"]
pub type SyncInR = crate::FieldReader;
#[doc = "Field `SYNC_IN` writer - synchronization input"]
pub type SyncInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCRSTM` reader - Synchronization Resets Master"]
pub type SyncrstmR = crate::BitReader;
#[doc = "Field `SYNCRSTM` writer - Synchronization Resets Master"]
pub type SyncrstmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCSTRTM` reader - Synchronization Starts Master"]
pub type SyncstrtmR = crate::BitReader;
#[doc = "Field `SYNCSTRTM` writer - Synchronization Starts Master"]
pub type SyncstrtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_OUT` reader - Synchronization output"]
pub type SyncOutR = crate::FieldReader;
#[doc = "Field `SYNC_OUT` writer - Synchronization output"]
pub type SyncOutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC_SRC` reader - Synchronization source"]
pub type SyncSrcR = crate::FieldReader;
#[doc = "Field `SYNC_SRC` writer - Synchronization source"]
pub type SyncSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCEN` reader - Master Counter enable"]
pub type McenR = crate::BitReader;
#[doc = "Field `MCEN` writer - Master Counter enable"]
pub type McenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACEN` reader - Timer A counter enable"]
pub type TacenR = crate::BitReader;
#[doc = "Field `TACEN` writer - Timer A counter enable"]
pub type TacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCEN` reader - Timer B counter enable"]
pub type TbcenR = crate::BitReader;
#[doc = "Field `TBCEN` writer - Timer B counter enable"]
pub type TbcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCEN` reader - Timer C counter enable"]
pub type TccenR = crate::BitReader;
#[doc = "Field `TCCEN` writer - Timer C counter enable"]
pub type TccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCEN` reader - Timer D counter enable"]
pub type TdcenR = crate::BitReader;
#[doc = "Field `TDCEN` writer - Timer D counter enable"]
pub type TdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TECEN` reader - Timer E counter enable"]
pub type TecenR = crate::BitReader;
#[doc = "Field `TECEN` writer - Timer E counter enable"]
pub type TecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCEN` reader - Timer F counter enable"]
pub type TfcenR = crate::BitReader;
#[doc = "Field `TFCEN` writer - Timer F counter enable"]
pub type TfcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DacsyncR = crate::FieldReader;
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DacsyncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PreenR = crate::BitReader;
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PreenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPU` reader - Master Timer Repetition update"]
pub type MrepuR = crate::BitReader;
#[doc = "Field `MREPU` writer - Master Timer Repetition update"]
pub type MrepuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSTDMA` reader - Burst DMA Update"]
pub type BrstdmaR = crate::FieldReader;
#[doc = "Field `BRSTDMA` writer - Burst DMA Update"]
pub type BrstdmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ck_psc(&self) -> CkPscR {
        CkPscR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RetrigR {
        RetrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HalfR {
        HalfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&self) -> IntlvdR {
        IntlvdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - synchronization input"]
    #[inline(always)]
    pub fn sync_in(&self) -> SyncInR {
        SyncInR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&self) -> SyncrstmR {
        SyncrstmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&self) -> SyncstrtmR {
        SyncstrtmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn sync_out(&self) -> SyncOutR {
        SyncOutR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn sync_src(&self) -> SyncSrcR {
        SyncSrcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&self) -> McenR {
        McenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&self) -> TacenR {
        TacenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&self) -> TbcenR {
        TbcenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&self) -> TccenR {
        TccenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TdcenR {
        TdcenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&self) -> TecenR {
        TecenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer F counter enable"]
    #[inline(always)]
    pub fn tfcen(&self) -> TfcenR {
        TfcenR::new(((self.bits >> 22) & 1) != 0)
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
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&self) -> MrepuR {
        MrepuR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&self) -> BrstdmaR {
        BrstdmaR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("brstdma", &self.brstdma())
            .field("mrepu", &self.mrepu())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("tfcen", &self.tfcen())
            .field("tecen", &self.tecen())
            .field("tdcen", &self.tdcen())
            .field("tccen", &self.tccen())
            .field("tbcen", &self.tbcen())
            .field("tacen", &self.tacen())
            .field("mcen", &self.mcen())
            .field("sync_src", &self.sync_src())
            .field("sync_out", &self.sync_out())
            .field("syncstrtm", &self.syncstrtm())
            .field("syncrstm", &self.syncrstm())
            .field("sync_in", &self.sync_in())
            .field("intlvd", &self.intlvd())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ck_psc", &self.ck_psc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ck_psc(&mut self) -> CkPscW<McrSpec> {
        CkPscW::new(self, 0)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<McrSpec> {
        ContW::new(self, 3)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RetrigW<McrSpec> {
        RetrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HalfW<McrSpec> {
        HalfW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interleaved mode"]
    #[inline(always)]
    #[must_use]
    pub fn intlvd(&mut self) -> IntlvdW<McrSpec> {
        IntlvdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - synchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn sync_in(&mut self) -> SyncInW<McrSpec> {
        SyncInW::new(self, 8)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstm(&mut self) -> SyncrstmW<McrSpec> {
        SyncrstmW::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtm(&mut self) -> SyncstrtmW<McrSpec> {
        SyncstrtmW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    #[must_use]
    pub fn sync_out(&mut self) -> SyncOutW<McrSpec> {
        SyncOutW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    #[must_use]
    pub fn sync_src(&mut self) -> SyncSrcW<McrSpec> {
        SyncSrcW::new(self, 14)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcen(&mut self) -> McenW<McrSpec> {
        McenW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tacen(&mut self) -> TacenW<McrSpec> {
        TacenW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbcen(&mut self) -> TbcenW<McrSpec> {
        TbcenW::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tccen(&mut self) -> TccenW<McrSpec> {
        TccenW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcen(&mut self) -> TdcenW<McrSpec> {
        TdcenW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tecen(&mut self) -> TecenW<McrSpec> {
        TecenW::new(self, 21)
    }
    #[doc = "Bit 22 - Timer F counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfcen(&mut self) -> TfcenW<McrSpec> {
        TfcenW::new(self, 22)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DacsyncW<McrSpec> {
        DacsyncW::new(self, 25)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PreenW<McrSpec> {
        PreenW::new(self, 27)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn mrepu(&mut self) -> MrepuW<McrSpec> {
        MrepuW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    #[must_use]
    pub fn brstdma(&mut self) -> BrstdmaW<McrSpec> {
        BrstdmaW::new(self, 30)
    }
}
#[doc = "Master Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
