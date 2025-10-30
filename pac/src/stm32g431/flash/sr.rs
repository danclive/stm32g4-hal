#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Operation error"]
pub type OperrR = crate::BitReader;
#[doc = "Field `OPERR` writer - Operation error"]
pub type OperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` reader - Programming error"]
pub type ProgerrR = crate::BitReader;
#[doc = "Field `PROGERR` writer - Programming error"]
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WrperrR = crate::BitReader;
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WrperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PgaerrR = crate::BitReader;
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PgaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZERR` reader - Size error"]
pub type SizerrR = crate::BitReader;
#[doc = "Field `SIZERR` writer - Size error"]
pub type SizerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PgserrR = crate::BitReader;
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PgserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISERR` reader - Fast programming data miss error"]
pub type MiserrR = crate::BitReader;
#[doc = "Field `MISERR` writer - Fast programming data miss error"]
pub type MiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FasterrR = crate::BitReader;
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub type FasterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDERR` reader - PCROP read error"]
pub type RderrR = crate::BitReader;
#[doc = "Field `RDERR` writer - PCROP read error"]
pub type RderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTVERR` reader - Option validity error"]
pub type OptverrR = crate::BitReader;
#[doc = "Field `OPTVERR` writer - Option validity error"]
pub type OptverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY` reader - Busy"]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WrperrR {
        WrperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PgaerrR {
        PgaerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SizerrR {
        SizerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PgserrR {
        PgserrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&self) -> MiserrR {
        MiserrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FasterrR {
        FasterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&self) -> RderrR {
        RderrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OptverrR {
        OptverrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("eop", &self.eop())
            .field("operr", &self.operr())
            .field("progerr", &self.progerr())
            .field("wrperr", &self.wrperr())
            .field("pgaerr", &self.pgaerr())
            .field("sizerr", &self.sizerr())
            .field("pgserr", &self.pgserr())
            .field("miserr", &self.miserr())
            .field("fasterr", &self.fasterr())
            .field("rderr", &self.rderr())
            .field("optverr", &self.optverr())
            .field("bsy", &self.bsy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<'_, SrSpec> {
        EopW::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&mut self) -> OperrW<'_, SrSpec> {
        OperrW::new(self, 1)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&mut self) -> ProgerrW<'_, SrSpec> {
        ProgerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WrperrW<'_, SrSpec> {
        WrperrW::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PgaerrW<'_, SrSpec> {
        PgaerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&mut self) -> SizerrW<'_, SrSpec> {
        SizerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&mut self) -> PgserrW<'_, SrSpec> {
        PgserrW::new(self, 7)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&mut self) -> MiserrW<'_, SrSpec> {
        MiserrW::new(self, 8)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&mut self) -> FasterrW<'_, SrSpec> {
        FasterrW::new(self, 9)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&mut self) -> RderrW<'_, SrSpec> {
        RderrW::new(self, 14)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&mut self) -> OptverrW<'_, SrSpec> {
        OptverrW::new(self, 15)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
