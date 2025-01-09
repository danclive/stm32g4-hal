#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag"]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag"]
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag"]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag"]
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag"]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag"]
pub type Cc3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag"]
pub type Cc4ifR = crate::BitReader;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag"]
pub type Cc4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type ComifR = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type ComifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BifR = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2IF` reader - Break 2 interrupt flag"]
pub type B2ifR = crate::BitReader;
#[doc = "Field `B2IF` writer - Break 2 interrupt flag"]
pub type B2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag"]
pub type Cc2ofR = crate::BitReader;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag"]
pub type Cc2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag"]
pub type Cc3ofR = crate::BitReader;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag"]
pub type Cc3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag"]
pub type Cc4ofR = crate::BitReader;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag"]
pub type Cc4ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIF` reader - System Break interrupt flag"]
pub type SbifR = crate::BitReader;
#[doc = "Field `SBIF` writer - System Break interrupt flag"]
pub type SbifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag"]
pub type Cc5ifR = crate::BitReader;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag"]
pub type Cc5ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IF` reader - Compare 6 interrupt flag"]
pub type Cc6ifR = crate::BitReader;
#[doc = "Field `CC6IF` writer - Compare 6 interrupt flag"]
pub type Cc6ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXF` reader - Index interrupt flag"]
pub type IdxfR = crate::BitReader;
#[doc = "Field `IDXF` writer - Index interrupt flag"]
pub type IdxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRF` reader - Direction Change interrupt flag"]
pub type DirfR = crate::BitReader;
#[doc = "Field `DIRF` writer - Direction Change interrupt flag"]
pub type DirfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRF` reader - Index Error interrupt flag"]
pub type IerrfR = crate::BitReader;
#[doc = "Field `IERRF` writer - Index Error interrupt flag"]
pub type IerrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRF` reader - Transition Error interrupt flag"]
pub type TerrfR = crate::BitReader;
#[doc = "Field `TERRF` writer - Transition Error interrupt flag"]
pub type TerrfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> ComifR {
        ComifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BifR {
        BifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag"]
    #[inline(always)]
    pub fn b2if(&self) -> B2ifR {
        B2ifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&self) -> Cc4ofR {
        Cc4ofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - System Break interrupt flag"]
    #[inline(always)]
    pub fn sbif(&self) -> SbifR {
        SbifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&self) -> Cc5ifR {
        Cc5ifR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn cc6if(&self) -> Cc6ifR {
        Cc6ifR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt flag"]
    #[inline(always)]
    pub fn idxf(&self) -> IdxfR {
        IdxfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction Change interrupt flag"]
    #[inline(always)]
    pub fn dirf(&self) -> DirfR {
        DirfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index Error interrupt flag"]
    #[inline(always)]
    pub fn ierrf(&self) -> IerrfR {
        IerrfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition Error interrupt flag"]
    #[inline(always)]
    pub fn terrf(&self) -> TerrfR {
        TerrfR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("terrf", &self.terrf())
            .field("ierrf", &self.ierrf())
            .field("dirf", &self.dirf())
            .field("idxf", &self.idxf())
            .field("cc6if", &self.cc6if())
            .field("cc5if", &self.cc5if())
            .field("sbif", &self.sbif())
            .field("cc4of", &self.cc4of())
            .field("cc3of", &self.cc3of())
            .field("cc2of", &self.cc2of())
            .field("cc1of", &self.cc1of())
            .field("b2if", &self.b2if())
            .field("bif", &self.bif())
            .field("tif", &self.tif())
            .field("comif", &self.comif())
            .field("cc4if", &self.cc4if())
            .field("cc3if", &self.cc3if())
            .field("cc2if", &self.cc2if())
            .field("cc1if", &self.cc1if())
            .field("uif", &self.uif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<SrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<SrSpec> {
        Cc1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<SrSpec> {
        Cc2ifW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<SrSpec> {
        Cc3ifW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> Cc4ifW<SrSpec> {
        Cc4ifW::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&mut self) -> ComifW<SrSpec> {
        ComifW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<SrSpec> {
        TifW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&mut self) -> BifW<SrSpec> {
        BifW::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag"]
    #[inline(always)]
    pub fn b2if(&mut self) -> B2ifW<SrSpec> {
        B2ifW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<SrSpec> {
        Cc1ofW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> Cc2ofW<SrSpec> {
        Cc2ofW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> Cc3ofW<SrSpec> {
        Cc3ofW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> Cc4ofW<SrSpec> {
        Cc4ofW::new(self, 12)
    }
    #[doc = "Bit 13 - System Break interrupt flag"]
    #[inline(always)]
    pub fn sbif(&mut self) -> SbifW<SrSpec> {
        SbifW::new(self, 13)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&mut self) -> Cc5ifW<SrSpec> {
        Cc5ifW::new(self, 16)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag"]
    #[inline(always)]
    pub fn cc6if(&mut self) -> Cc6ifW<SrSpec> {
        Cc6ifW::new(self, 17)
    }
    #[doc = "Bit 20 - Index interrupt flag"]
    #[inline(always)]
    pub fn idxf(&mut self) -> IdxfW<SrSpec> {
        IdxfW::new(self, 20)
    }
    #[doc = "Bit 21 - Direction Change interrupt flag"]
    #[inline(always)]
    pub fn dirf(&mut self) -> DirfW<SrSpec> {
        DirfW::new(self, 21)
    }
    #[doc = "Bit 22 - Index Error interrupt flag"]
    #[inline(always)]
    pub fn ierrf(&mut self) -> IerrfW<SrSpec> {
        IerrfW::new(self, 22)
    }
    #[doc = "Bit 23 - Transition Error interrupt flag"]
    #[inline(always)]
    pub fn terrf(&mut self) -> TerrfW<SrSpec> {
        TerrfW::new(self, 23)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
