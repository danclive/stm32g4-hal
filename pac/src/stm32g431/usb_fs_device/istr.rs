#[doc = "Register `ISTR` reader"]
pub type R = crate::R<IstrSpec>;
#[doc = "Register `ISTR` writer"]
pub type W = crate::W<IstrSpec>;
#[doc = "Field `EP_ID` reader - EP_ID"]
pub type EpIdR = crate::FieldReader;
#[doc = "Field `EP_ID` writer - EP_ID"]
pub type EpIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIR` reader - DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1REQ` reader - L1REQ"]
pub type L1reqR = crate::BitReader;
#[doc = "Field `L1REQ` writer - L1REQ"]
pub type L1reqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOF` reader - ESOF"]
pub type EsofR = crate::BitReader;
#[doc = "Field `ESOF` writer - ESOF"]
pub type EsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - SOF"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - SOF"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - RESET"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - RESET"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - SUSP"]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - SUSP"]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - WKUP"]
pub type WkupR = crate::BitReader;
#[doc = "Field `WKUP` writer - WKUP"]
pub type WkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - ERR"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - ERR"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVR` reader - PMAOVR"]
pub type PmaovrR = crate::BitReader;
#[doc = "Field `PMAOVR` writer - PMAOVR"]
pub type PmaovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - CTR"]
pub type CtrR = crate::BitReader;
#[doc = "Field `CTR` writer - CTR"]
pub type CtrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    pub fn ep_id(&self) -> EpIdR {
        EpIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    pub fn l1req(&self) -> L1reqR {
        L1reqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    pub fn esof(&self) -> EsofR {
        EsofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    pub fn wkup(&self) -> WkupR {
        WkupR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PmaovrR {
        PmaovrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTR")
            .field("ep_id", &self.ep_id())
            .field("dir", &self.dir())
            .field("l1req", &self.l1req())
            .field("esof", &self.esof())
            .field("sof", &self.sof())
            .field("reset", &self.reset())
            .field("susp", &self.susp())
            .field("wkup", &self.wkup())
            .field("err", &self.err())
            .field("pmaovr", &self.pmaovr())
            .field("ctr", &self.ctr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EP_ID"]
    #[inline(always)]
    pub fn ep_id(&mut self) -> EpIdW<'_, IstrSpec> {
        EpIdW::new(self, 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, IstrSpec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 7 - L1REQ"]
    #[inline(always)]
    pub fn l1req(&mut self) -> L1reqW<'_, IstrSpec> {
        L1reqW::new(self, 7)
    }
    #[doc = "Bit 8 - ESOF"]
    #[inline(always)]
    pub fn esof(&mut self) -> EsofW<'_, IstrSpec> {
        EsofW::new(self, 8)
    }
    #[doc = "Bit 9 - SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SofW<'_, IstrSpec> {
        SofW::new(self, 9)
    }
    #[doc = "Bit 10 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, IstrSpec> {
        ResetW::new(self, 10)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<'_, IstrSpec> {
        SuspW::new(self, 11)
    }
    #[doc = "Bit 12 - WKUP"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WkupW<'_, IstrSpec> {
        WkupW::new(self, 12)
    }
    #[doc = "Bit 13 - ERR"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IstrSpec> {
        ErrW::new(self, 13)
    }
    #[doc = "Bit 14 - PMAOVR"]
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PmaovrW<'_, IstrSpec> {
        PmaovrW::new(self, 14)
    }
    #[doc = "Bit 15 - CTR"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<'_, IstrSpec> {
        CtrW::new(self, 15)
    }
}
#[doc = "USB interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstrSpec;
impl crate::RegisterSpec for IstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istr::R`](R) reader structure"]
impl crate::Readable for IstrSpec {}
#[doc = "`write(|w| ..)` method takes [`istr::W`](W) writer structure"]
impl crate::Writable for IstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for IstrSpec {}
