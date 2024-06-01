#[doc = "Register `I2SCFGR` reader"]
pub type R = crate::R<I2scfgrSpec>;
#[doc = "Register `I2SCFGR` writer"]
pub type W = crate::W<I2scfgrSpec>;
#[doc = "Field `CHLEN` reader - CHLEN"]
pub type ChlenR = crate::BitReader;
#[doc = "Field `CHLEN` writer - CHLEN"]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - DATLEN"]
pub type DatlenR = crate::FieldReader;
#[doc = "Field `DATLEN` writer - DATLEN"]
pub type DatlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKPOL` reader - CKPOL"]
pub type CkpolR = crate::BitReader;
#[doc = "Field `CKPOL` writer - CKPOL"]
pub type CkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSTD` reader - I2SSTD"]
pub type I2sstdR = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2SSTD"]
pub type I2sstdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCMSYNC"]
pub type PcmsyncR = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCMSYNC"]
pub type PcmsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SCFG` reader - I2SCFG"]
pub type I2scfgR = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2SCFG"]
pub type I2scfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SE` reader - I2SE"]
pub type I2seR = crate::BitReader;
#[doc = "Field `I2SE` writer - I2SE"]
pub type I2seW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SMOD` reader - I2SMOD"]
pub type I2smodR = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2SMOD"]
pub type I2smodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CHLEN"]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DATLEN"]
    #[inline(always)]
    pub fn datlen(&self) -> DatlenR {
        DatlenR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2sstdR {
        I2sstdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PcmsyncR {
        PcmsyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2scfgR {
        I2scfgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2SE"]
    #[inline(always)]
    pub fn i2se(&self) -> I2seR {
        I2seR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2SMOD"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2smodR {
        I2smodR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCFGR")
            .field("chlen", &self.chlen())
            .field("datlen", &self.datlen())
            .field("ckpol", &self.ckpol())
            .field("i2sstd", &self.i2sstd())
            .field("pcmsync", &self.pcmsync())
            .field("i2scfg", &self.i2scfg())
            .field("i2se", &self.i2se())
            .field("i2smod", &self.i2smod())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CHLEN"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> ChlenW<I2scfgrSpec> {
        ChlenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - DATLEN"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DatlenW<I2scfgrSpec> {
        DatlenW::new(self, 1)
    }
    #[doc = "Bit 3 - CKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CkpolW<I2scfgrSpec> {
        CkpolW::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2sstdW<I2scfgrSpec> {
        I2sstdW::new(self, 4)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PcmsyncW<I2scfgrSpec> {
        PcmsyncW::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2SCFG"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2scfgW<I2scfgrSpec> {
        I2scfgW::new(self, 8)
    }
    #[doc = "Bit 10 - I2SE"]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2seW<I2scfgrSpec> {
        I2seW::new(self, 10)
    }
    #[doc = "Bit 11 - I2SMOD"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2smodW<I2scfgrSpec> {
        I2smodW::new(self, 11)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2scfgrSpec;
impl crate::RegisterSpec for I2scfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2scfgr::R`](R) reader structure"]
impl crate::Readable for I2scfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure"]
impl crate::Writable for I2scfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2scfgrSpec {
    const RESET_VALUE: u32 = 0;
}
