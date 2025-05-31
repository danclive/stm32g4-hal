#[doc = "Register `AF2` reader"]
pub type R = crate::R<Af2Spec>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<Af2Spec>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type Bk2cmp1eR = crate::BitReader;
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type Bk2cmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type Bk2cmp2eR = crate::BitReader;
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type Bk2cmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP3E` reader - BRK2 COMP3 enable"]
pub type Bk2cmp3eR = crate::BitReader;
#[doc = "Field `BK2CMP3E` writer - BRK2 COMP3 enable"]
pub type Bk2cmp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP4E` reader - BRK2 COMP4 enable"]
pub type Bk2cmp4eR = crate::BitReader;
#[doc = "Field `BK2CMP4E` writer - BRK2 COMP4 enable"]
pub type Bk2cmp4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP5E` reader - BRK2 COMP5 enable"]
pub type Bk2cmp5eR = crate::BitReader;
#[doc = "Field `BK2CMP5E` writer - BRK2 COMP5 enable"]
pub type Bk2cmp5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP6E` reader - BRK2 COMP6 enable"]
pub type Bk2cmp6eR = crate::BitReader;
#[doc = "Field `BK2CMP6E` writer - BRK2 COMP6 enable"]
pub type Bk2cmp6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP7E` reader - BRK2 COMP7 enable"]
pub type Bk2cmp7eR = crate::BitReader;
#[doc = "Field `BK2CMP7E` writer - BRK2 COMP7 enable"]
pub type Bk2cmp7eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - BRK2 BKIN input polarity"]
pub type Bk2inpR = crate::BitReader;
#[doc = "Field `BK2INP` writer - BRK2 BKIN input polarity"]
pub type Bk2inpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub type Bk2cmp1pR = crate::BitReader;
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub type Bk2cmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type Bk2cmp2pR = crate::BitReader;
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type Bk2cmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP3P` reader - BRK2 COMP3 input polarity"]
pub type Bk2cmp3pR = crate::BitReader;
#[doc = "Field `BK2CMP3P` writer - BRK2 COMP3 input polarity"]
pub type Bk2cmp3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP4P` reader - BRK2 COMP4 input polarity"]
pub type Bk2cmp4pR = crate::BitReader;
#[doc = "Field `BK2CMP4P` writer - BRK2 COMP4 input polarity"]
pub type Bk2cmp4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCRSEL` reader - OCREF_CLR source selection"]
pub type OcrselR = crate::FieldReader;
#[doc = "Field `OCRSEL` writer - OCREF_CLR source selection"]
pub type OcrselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> Bk2cmp1eR {
        Bk2cmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> Bk2cmp2eR {
        Bk2cmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn bk2cmp3e(&self) -> Bk2cmp3eR {
        Bk2cmp3eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn bk2cmp4e(&self) -> Bk2cmp4eR {
        Bk2cmp4eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn bk2cmp5e(&self) -> Bk2cmp5eR {
        Bk2cmp5eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn bk2cmp6e(&self) -> Bk2cmp6eR {
        Bk2cmp6eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn bk2cmp7e(&self) -> Bk2cmp7eR {
        Bk2cmp7eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> Bk2inpR {
        Bk2inpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> Bk2cmp1pR {
        Bk2cmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> Bk2cmp2pR {
        Bk2cmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn bk2cmp3p(&self) -> Bk2cmp3pR {
        Bk2cmp3pR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn bk2cmp4p(&self) -> Bk2cmp4pR {
        Bk2cmp4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&self) -> OcrselR {
        OcrselR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("ocrsel", &self.ocrsel())
            .field("bk2cmp4p", &self.bk2cmp4p())
            .field("bk2cmp3p", &self.bk2cmp3p())
            .field("bk2cmp2p", &self.bk2cmp2p())
            .field("bk2cmp1p", &self.bk2cmp1p())
            .field("bk2inp", &self.bk2inp())
            .field("bk2cmp7e", &self.bk2cmp7e())
            .field("bk2cmp6e", &self.bk2cmp6e())
            .field("bk2cmp5e", &self.bk2cmp5e())
            .field("bk2cmp4e", &self.bk2cmp4e())
            .field("bk2cmp3e", &self.bk2cmp3e())
            .field("bk2cmp2e", &self.bk2cmp2e())
            .field("bk2cmp1e", &self.bk2cmp1e())
            .field("bkine", &self.bkine())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<Af2Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> Bk2cmp1eW<Af2Spec> {
        Bk2cmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> Bk2cmp2eW<Af2Spec> {
        Bk2cmp2eW::new(self, 2)
    }
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn bk2cmp3e(&mut self) -> Bk2cmp3eW<Af2Spec> {
        Bk2cmp3eW::new(self, 3)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn bk2cmp4e(&mut self) -> Bk2cmp4eW<Af2Spec> {
        Bk2cmp4eW::new(self, 4)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn bk2cmp5e(&mut self) -> Bk2cmp5eW<Af2Spec> {
        Bk2cmp5eW::new(self, 5)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn bk2cmp6e(&mut self) -> Bk2cmp6eW<Af2Spec> {
        Bk2cmp6eW::new(self, 6)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn bk2cmp7e(&mut self) -> Bk2cmp7eW<Af2Spec> {
        Bk2cmp7eW::new(self, 7)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> Bk2inpW<Af2Spec> {
        Bk2inpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> Bk2cmp1pW<Af2Spec> {
        Bk2cmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> Bk2cmp2pW<Af2Spec> {
        Bk2cmp2pW::new(self, 11)
    }
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn bk2cmp3p(&mut self) -> Bk2cmp3pW<Af2Spec> {
        Bk2cmp3pW::new(self, 12)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn bk2cmp4p(&mut self) -> Bk2cmp4pW<Af2Spec> {
        Bk2cmp4pW::new(self, 13)
    }
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OcrselW<Af2Spec> {
        OcrselW::new(self, 16)
    }
}
#[doc = "TIM alternate function option register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af2Spec;
impl crate::RegisterSpec for Af2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for Af2Spec {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for Af2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for Af2Spec {}
