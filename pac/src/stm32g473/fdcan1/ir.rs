#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `RF0N` reader - RF0N"]
pub type Rf0nR = crate::BitReader;
#[doc = "Field `RF0N` writer - RF0N"]
pub type Rf0nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0F` reader - RF0F"]
pub type Rf0fR = crate::BitReader;
#[doc = "Field `RF0F` writer - RF0F"]
pub type Rf0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - RF0L"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - RF0L"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1N` reader - RF1N"]
pub type Rf1nR = crate::BitReader;
#[doc = "Field `RF1N` writer - RF1N"]
pub type Rf1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1F` reader - RF1F"]
pub type Rf1fR = crate::BitReader;
#[doc = "Field `RF1F` writer - RF1F"]
pub type Rf1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - RF1L"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - RF1L"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM` reader - HPM"]
pub type HpmR = crate::BitReader;
#[doc = "Field `HPM` writer - HPM"]
pub type HpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - TC"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCF` reader - TCF"]
pub type TcfR = crate::BitReader;
#[doc = "Field `TCF` writer - TCF"]
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - TFE"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - TFE"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFN` reader - TEFN"]
pub type TefnR = crate::BitReader;
#[doc = "Field `TEFN` writer - TEFN"]
pub type TefnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFF` reader - TEFF"]
pub type TeffR = crate::BitReader;
#[doc = "Field `TEFF` writer - TEFF"]
pub type TeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - TEFL"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - TEFL"]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSW` reader - TSW"]
pub type TswR = crate::BitReader;
#[doc = "Field `TSW` writer - TSW"]
pub type TswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAF` reader - MRAF"]
pub type MrafR = crate::BitReader;
#[doc = "Field `MRAF` writer - MRAF"]
pub type MrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOO` reader - TOO"]
pub type TooR = crate::BitReader;
#[doc = "Field `TOO` writer - TOO"]
pub type TooW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELO` reader - ELO"]
pub type EloR = crate::BitReader;
#[doc = "Field `ELO` writer - ELO"]
pub type EloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - EP"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - EP"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - EW"]
pub type EwR = crate::BitReader;
#[doc = "Field `EW` writer - EW"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - BO"]
pub type BoR = crate::BitReader;
#[doc = "Field `BO` writer - BO"]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDI` reader - WDI"]
pub type WdiR = crate::BitReader;
#[doc = "Field `WDI` writer - WDI"]
pub type WdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEA` reader - PEA"]
pub type PeaR = crate::BitReader;
#[doc = "Field `PEA` writer - PEA"]
pub type PeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - PED"]
pub type PedR = crate::BitReader;
#[doc = "Field `PED` writer - PED"]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARA` reader - ARA"]
pub type AraR = crate::BitReader;
#[doc = "Field `ARA` writer - ARA"]
pub type AraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RF0N"]
    #[inline(always)]
    pub fn rf0n(&self) -> Rf0nR {
        Rf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RF0F"]
    #[inline(always)]
    pub fn rf0f(&self) -> Rf0fR {
        Rf0fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RF1N"]
    #[inline(always)]
    pub fn rf1n(&self) -> Rf1nR {
        Rf1nR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RF1F"]
    #[inline(always)]
    pub fn rf1f(&self) -> Rf1fR {
        Rf1fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HPM"]
    #[inline(always)]
    pub fn hpm(&self) -> HpmR {
        HpmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TEFN"]
    #[inline(always)]
    pub fn tefn(&self) -> TefnR {
        TefnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TEFF"]
    #[inline(always)]
    pub fn teff(&self) -> TeffR {
        TeffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSW"]
    #[inline(always)]
    pub fn tsw(&self) -> TswR {
        TswR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MRAF"]
    #[inline(always)]
    pub fn mraf(&self) -> MrafR {
        MrafR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TOO"]
    #[inline(always)]
    pub fn too(&self) -> TooR {
        TooR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELO"]
    #[inline(always)]
    pub fn elo(&self) -> EloR {
        EloR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WDI"]
    #[inline(always)]
    pub fn wdi(&self) -> WdiR {
        WdiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PEA"]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PED"]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ARA"]
    #[inline(always)]
    pub fn ara(&self) -> AraR {
        AraR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("rf0n", &self.rf0n())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RF0N"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> Rf0nW<IrSpec> {
        Rf0nW::new(self, 0)
    }
    #[doc = "Bit 1 - RF0F"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> Rf0fW<IrSpec> {
        Rf0fW::new(self, 1)
    }
    #[doc = "Bit 2 - RF0L"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> Rf0lW<IrSpec> {
        Rf0lW::new(self, 2)
    }
    #[doc = "Bit 3 - RF1N"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> Rf1nW<IrSpec> {
        Rf1nW::new(self, 3)
    }
    #[doc = "Bit 4 - RF1F"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> Rf1fW<IrSpec> {
        Rf1fW::new(self, 4)
    }
    #[doc = "Bit 5 - RF1L"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> Rf1lW<IrSpec> {
        Rf1lW::new(self, 5)
    }
    #[doc = "Bit 6 - HPM"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HpmW<IrSpec> {
        HpmW::new(self, 6)
    }
    #[doc = "Bit 7 - TC"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<IrSpec> {
        TcW::new(self, 7)
    }
    #[doc = "Bit 8 - TCF"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TcfW<IrSpec> {
        TcfW::new(self, 8)
    }
    #[doc = "Bit 9 - TFE"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<IrSpec> {
        TfeW::new(self, 9)
    }
    #[doc = "Bit 10 - TEFN"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TefnW<IrSpec> {
        TefnW::new(self, 10)
    }
    #[doc = "Bit 11 - TEFF"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TeffW<IrSpec> {
        TeffW::new(self, 11)
    }
    #[doc = "Bit 12 - TEFL"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TeflW<IrSpec> {
        TeflW::new(self, 12)
    }
    #[doc = "Bit 13 - TSW"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TswW<IrSpec> {
        TswW::new(self, 13)
    }
    #[doc = "Bit 14 - MRAF"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MrafW<IrSpec> {
        MrafW::new(self, 14)
    }
    #[doc = "Bit 15 - TOO"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TooW<IrSpec> {
        TooW::new(self, 15)
    }
    #[doc = "Bit 16 - ELO"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> EloW<IrSpec> {
        EloW::new(self, 16)
    }
    #[doc = "Bit 17 - EP"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EpW<IrSpec> {
        EpW::new(self, 17)
    }
    #[doc = "Bit 18 - EW"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EwW<IrSpec> {
        EwW::new(self, 18)
    }
    #[doc = "Bit 19 - BO"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BoW<IrSpec> {
        BoW::new(self, 19)
    }
    #[doc = "Bit 20 - WDI"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WdiW<IrSpec> {
        WdiW::new(self, 20)
    }
    #[doc = "Bit 21 - PEA"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PeaW<IrSpec> {
        PeaW::new(self, 21)
    }
    #[doc = "Bit 22 - PED"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PedW<IrSpec> {
        PedW::new(self, 22)
    }
    #[doc = "Bit 23 - ARA"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> AraW<IrSpec> {
        AraW::new(self, 23)
    }
}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of IE controls whether an interrupt is generated. The configuration of ILS controls on which interrupt line an interrupt is signaled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {
    const RESET_VALUE: u32 = 0;
}
