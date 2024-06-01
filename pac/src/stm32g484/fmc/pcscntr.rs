#[doc = "Register `PCSCNTR` reader"]
pub type R = crate::R<PcscntrSpec>;
#[doc = "Register `PCSCNTR` writer"]
pub type W = crate::W<PcscntrSpec>;
#[doc = "Field `CSCOUNT` reader - CSCOUNT"]
pub type CscountR = crate::FieldReader<u16>;
#[doc = "Field `CSCOUNT` writer - CSCOUNT"]
pub type CscountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNTB1EN` reader - CNTB1EN"]
pub type Cntb1enR = crate::BitReader;
#[doc = "Field `CNTB1EN` writer - CNTB1EN"]
pub type Cntb1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB2EN` reader - CNTB2EN"]
pub type Cntb2enR = crate::BitReader;
#[doc = "Field `CNTB2EN` writer - CNTB2EN"]
pub type Cntb2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB3EN` reader - CNTB3EN"]
pub type Cntb3enR = crate::BitReader;
#[doc = "Field `CNTB3EN` writer - CNTB3EN"]
pub type Cntb3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB4EN` reader - CNTB4EN"]
pub type Cntb4enR = crate::BitReader;
#[doc = "Field `CNTB4EN` writer - CNTB4EN"]
pub type Cntb4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    pub fn cscount(&self) -> CscountR {
        CscountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    pub fn cntb1en(&self) -> Cntb1enR {
        Cntb1enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    pub fn cntb2en(&self) -> Cntb2enR {
        Cntb2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    pub fn cntb3en(&self) -> Cntb3enR {
        Cntb3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    pub fn cntb4en(&self) -> Cntb4enR {
        Cntb4enR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSCNTR")
            .field("cscount", &self.cscount())
            .field("cntb1en", &self.cntb1en())
            .field("cntb2en", &self.cntb2en())
            .field("cntb3en", &self.cntb3en())
            .field("cntb4en", &self.cntb4en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn cscount(&mut self) -> CscountW<PcscntrSpec> {
        CscountW::new(self, 0)
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb1en(&mut self) -> Cntb1enW<PcscntrSpec> {
        Cntb1enW::new(self, 16)
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb2en(&mut self) -> Cntb2enW<PcscntrSpec> {
        Cntb2enW::new(self, 17)
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb3en(&mut self) -> Cntb3enW<PcscntrSpec> {
        Cntb3enW::new(self, 18)
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    #[must_use]
    pub fn cntb4en(&mut self) -> Cntb4enW<PcscntrSpec> {
        Cntb4enW::new(self, 19)
    }
}
#[doc = "PSRAM chip select counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcscntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcscntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcscntrSpec;
impl crate::RegisterSpec for PcscntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcscntr::R`](R) reader structure"]
impl crate::Readable for PcscntrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcscntr::W`](W) writer structure"]
impl crate::Writable for PcscntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCSCNTR to value 0"]
impl crate::Resettable for PcscntrSpec {
    const RESET_VALUE: u32 = 0;
}
