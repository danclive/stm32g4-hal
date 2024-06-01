#[doc = "Register `BMCR` reader"]
pub type R = crate::R<BmcrSpec>;
#[doc = "Register `BMCR` writer"]
pub type W = crate::W<BmcrSpec>;
#[doc = "Field `BME` reader - Burst Mode enable"]
pub type BmeR = crate::BitReader;
#[doc = "Field `BME` writer - Burst Mode enable"]
pub type BmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMOM` reader - Burst Mode operating mode"]
pub type BmomR = crate::BitReader;
#[doc = "Field `BMOM` writer - Burst Mode operating mode"]
pub type BmomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMCLK` reader - Burst Mode Clock source"]
pub type BmclkR = crate::FieldReader;
#[doc = "Field `BMCLK` writer - Burst Mode Clock source"]
pub type BmclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BMPRSC` reader - Burst Mode Prescaler"]
pub type BmprscR = crate::FieldReader;
#[doc = "Field `BMPRSC` writer - Burst Mode Prescaler"]
pub type BmprscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BMPREN` reader - Burst Mode Preload Enable"]
pub type BmprenR = crate::BitReader;
#[doc = "Field `BMPREN` writer - Burst Mode Preload Enable"]
pub type BmprenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTBM` reader - Master Timer Burst Mode"]
pub type MtbmR = crate::BitReader;
#[doc = "Field `MTBM` writer - Master Timer Burst Mode"]
pub type MtbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TABM` reader - Timer A Burst Mode"]
pub type TabmR = crate::BitReader;
#[doc = "Field `TABM` writer - Timer A Burst Mode"]
pub type TabmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBBM` reader - Timer B Burst Mode"]
pub type TbbmR = crate::BitReader;
#[doc = "Field `TBBM` writer - Timer B Burst Mode"]
pub type TbbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBM` reader - Timer C Burst Mode"]
pub type TcbmR = crate::BitReader;
#[doc = "Field `TCBM` writer - Timer C Burst Mode"]
pub type TcbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBM` reader - Timer D Burst Mode"]
pub type TdbmR = crate::BitReader;
#[doc = "Field `TDBM` writer - Timer D Burst Mode"]
pub type TdbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEBM` reader - Timer E Burst Mode"]
pub type TebmR = crate::BitReader;
#[doc = "Field `TEBM` writer - Timer E Burst Mode"]
pub type TebmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFBM` reader - Timer f Burst Mode"]
pub type TfbmR = crate::BitReader;
#[doc = "Field `TFBM` writer - Timer f Burst Mode"]
pub type TfbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSTAT` reader - Burst Mode Status"]
pub type BmstatR = crate::BitReader;
#[doc = "Field `BMSTAT` writer - Burst Mode Status"]
pub type BmstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BmeR {
        BmeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BmomR {
        BmomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BmclkR {
        BmclkR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BmprscR {
        BmprscR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BmprenR {
        BmprenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MtbmR {
        MtbmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TabmR {
        TabmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TbbmR {
        TbbmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TcbmR {
        TcbmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TdbmR {
        TdbmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TebmR {
        TebmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer f Burst Mode"]
    #[inline(always)]
    pub fn tfbm(&self) -> TfbmR {
        TfbmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BmstatR {
        BmstatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMCR")
            .field("bmstat", &self.bmstat())
            .field("tfbm", &self.tfbm())
            .field("tebm", &self.tebm())
            .field("tdbm", &self.tdbm())
            .field("tcbm", &self.tcbm())
            .field("tbbm", &self.tbbm())
            .field("tabm", &self.tabm())
            .field("mtbm", &self.mtbm())
            .field("bmpren", &self.bmpren())
            .field("bmprsc", &self.bmprsc())
            .field("bmclk", &self.bmclk())
            .field("bmom", &self.bmom())
            .field("bme", &self.bme())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bme(&mut self) -> BmeW<BmcrSpec> {
        BmeW::new(self, 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmom(&mut self) -> BmomW<BmcrSpec> {
        BmomW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn bmclk(&mut self) -> BmclkW<BmcrSpec> {
        BmclkW::new(self, 2)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn bmprsc(&mut self) -> BmprscW<BmcrSpec> {
        BmprscW::new(self, 6)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmpren(&mut self) -> BmprenW<BmcrSpec> {
        BmprenW::new(self, 10)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mtbm(&mut self) -> MtbmW<BmcrSpec> {
        MtbmW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tabm(&mut self) -> TabmW<BmcrSpec> {
        TabmW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbbm(&mut self) -> TbbmW<BmcrSpec> {
        TbbmW::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tcbm(&mut self) -> TcbmW<BmcrSpec> {
        TcbmW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdbm(&mut self) -> TdbmW<BmcrSpec> {
        TdbmW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tebm(&mut self) -> TebmW<BmcrSpec> {
        TebmW::new(self, 21)
    }
    #[doc = "Bit 22 - Timer f Burst Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tfbm(&mut self) -> TfbmW<BmcrSpec> {
        TfbmW::new(self, 22)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    #[must_use]
    pub fn bmstat(&mut self) -> BmstatW<BmcrSpec> {
        BmstatW::new(self, 31)
    }
}
#[doc = "Burst Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmcrSpec;
impl crate::RegisterSpec for BmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcr::R`](R) reader structure"]
impl crate::Readable for BmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bmcr::W`](W) writer structure"]
impl crate::Writable for BmcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMCR to value 0"]
impl crate::Resettable for BmcrSpec {
    const RESET_VALUE: u32 = 0;
}
