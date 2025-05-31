#[doc = "Register `CNTR` reader"]
pub type R = crate::R<CntrSpec>;
#[doc = "Register `CNTR` writer"]
pub type W = crate::W<CntrSpec>;
#[doc = "Field `FRES` reader - FRES"]
pub type FresR = crate::BitReader;
#[doc = "Field `FRES` writer - FRES"]
pub type FresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDWN` reader - PDWN"]
pub type PdwnR = crate::BitReader;
#[doc = "Field `PDWN` writer - PDWN"]
pub type PdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MODE` reader - LP_MODE"]
pub type LpModeR = crate::BitReader;
#[doc = "Field `LP_MODE` writer - LP_MODE"]
pub type LpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUSP` reader - FSUSP"]
pub type FsuspR = crate::BitReader;
#[doc = "Field `FSUSP` writer - FSUSP"]
pub type FsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - RESUME"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - RESUME"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1RESUME` reader - L1RESUME"]
pub type L1resumeR = crate::BitReader;
#[doc = "Field `L1RESUME` writer - L1RESUME"]
pub type L1resumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1REQM` reader - L1REQM"]
pub type L1reqmR = crate::BitReader;
#[doc = "Field `L1REQM` writer - L1REQM"]
pub type L1reqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESOFM` reader - ESOFM"]
pub type EsofmR = crate::BitReader;
#[doc = "Field `ESOFM` writer - ESOFM"]
pub type EsofmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFM` reader - SOFM"]
pub type SofmR = crate::BitReader;
#[doc = "Field `SOFM` writer - SOFM"]
pub type SofmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETM` reader - RESETM"]
pub type ResetmR = crate::BitReader;
#[doc = "Field `RESETM` writer - RESETM"]
pub type ResetmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPM` reader - SUSPM"]
pub type SuspmR = crate::BitReader;
#[doc = "Field `SUSPM` writer - SUSPM"]
pub type SuspmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPM` reader - WKUPM"]
pub type WkupmR = crate::BitReader;
#[doc = "Field `WKUPM` writer - WKUPM"]
pub type WkupmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRM` reader - ERRM"]
pub type ErrmR = crate::BitReader;
#[doc = "Field `ERRM` writer - ERRM"]
pub type ErrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMAOVRM` reader - PMAOVRM"]
pub type PmaovrmR = crate::BitReader;
#[doc = "Field `PMAOVRM` writer - PMAOVRM"]
pub type PmaovrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRM` reader - CTRM"]
pub type CtrmR = crate::BitReader;
#[doc = "Field `CTRM` writer - CTRM"]
pub type CtrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FRES"]
    #[inline(always)]
    pub fn fres(&self) -> FresR {
        FresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&self) -> PdwnR {
        PdwnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LP_MODE"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LpModeR {
        LpModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FSUSP"]
    #[inline(always)]
    pub fn fsusp(&self) -> FsuspR {
        FsuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESUME"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1RESUME"]
    #[inline(always)]
    pub fn l1resume(&self) -> L1resumeR {
        L1resumeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - L1REQM"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1reqmR {
        L1reqmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ESOFM"]
    #[inline(always)]
    pub fn esofm(&self) -> EsofmR {
        EsofmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SofmR {
        SofmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RESETM"]
    #[inline(always)]
    pub fn resetm(&self) -> ResetmR {
        ResetmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSPM"]
    #[inline(always)]
    pub fn suspm(&self) -> SuspmR {
        SuspmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WKUPM"]
    #[inline(always)]
    pub fn wkupm(&self) -> WkupmR {
        WkupmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ERRM"]
    #[inline(always)]
    pub fn errm(&self) -> ErrmR {
        ErrmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PMAOVRM"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PmaovrmR {
        PmaovrmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTRM"]
    #[inline(always)]
    pub fn ctrm(&self) -> CtrmR {
        CtrmR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTR")
            .field("fres", &self.fres())
            .field("pdwn", &self.pdwn())
            .field("lp_mode", &self.lp_mode())
            .field("fsusp", &self.fsusp())
            .field("resume", &self.resume())
            .field("l1resume", &self.l1resume())
            .field("l1reqm", &self.l1reqm())
            .field("esofm", &self.esofm())
            .field("sofm", &self.sofm())
            .field("resetm", &self.resetm())
            .field("suspm", &self.suspm())
            .field("wkupm", &self.wkupm())
            .field("errm", &self.errm())
            .field("pmaovrm", &self.pmaovrm())
            .field("ctrm", &self.ctrm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FRES"]
    #[inline(always)]
    pub fn fres(&mut self) -> FresW<CntrSpec> {
        FresW::new(self, 0)
    }
    #[doc = "Bit 1 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PdwnW<CntrSpec> {
        PdwnW::new(self, 1)
    }
    #[doc = "Bit 2 - LP_MODE"]
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LpModeW<CntrSpec> {
        LpModeW::new(self, 2)
    }
    #[doc = "Bit 3 - FSUSP"]
    #[inline(always)]
    pub fn fsusp(&mut self) -> FsuspW<CntrSpec> {
        FsuspW::new(self, 3)
    }
    #[doc = "Bit 4 - RESUME"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<CntrSpec> {
        ResumeW::new(self, 4)
    }
    #[doc = "Bit 5 - L1RESUME"]
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1resumeW<CntrSpec> {
        L1resumeW::new(self, 5)
    }
    #[doc = "Bit 7 - L1REQM"]
    #[inline(always)]
    pub fn l1reqm(&mut self) -> L1reqmW<CntrSpec> {
        L1reqmW::new(self, 7)
    }
    #[doc = "Bit 8 - ESOFM"]
    #[inline(always)]
    pub fn esofm(&mut self) -> EsofmW<CntrSpec> {
        EsofmW::new(self, 8)
    }
    #[doc = "Bit 9 - SOFM"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SofmW<CntrSpec> {
        SofmW::new(self, 9)
    }
    #[doc = "Bit 10 - RESETM"]
    #[inline(always)]
    pub fn resetm(&mut self) -> ResetmW<CntrSpec> {
        ResetmW::new(self, 10)
    }
    #[doc = "Bit 11 - SUSPM"]
    #[inline(always)]
    pub fn suspm(&mut self) -> SuspmW<CntrSpec> {
        SuspmW::new(self, 11)
    }
    #[doc = "Bit 12 - WKUPM"]
    #[inline(always)]
    pub fn wkupm(&mut self) -> WkupmW<CntrSpec> {
        WkupmW::new(self, 12)
    }
    #[doc = "Bit 13 - ERRM"]
    #[inline(always)]
    pub fn errm(&mut self) -> ErrmW<CntrSpec> {
        ErrmW::new(self, 13)
    }
    #[doc = "Bit 14 - PMAOVRM"]
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PmaovrmW<CntrSpec> {
        PmaovrmW::new(self, 14)
    }
    #[doc = "Bit 15 - CTRM"]
    #[inline(always)]
    pub fn ctrm(&mut self) -> CtrmW<CntrSpec> {
        CtrmW::new(self, 15)
    }
}
#[doc = "USB control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrSpec;
impl crate::RegisterSpec for CntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CntrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTR to value 0"]
impl crate::Resettable for CntrSpec {}
