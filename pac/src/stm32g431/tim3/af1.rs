#[doc = "Register `AF1` reader"]
pub type R = crate::R<Af1Spec>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<Af1Spec>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type Bkcmp1eR = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type Bkcmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type Bkcmp2eR = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type Bkcmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3E` reader - BRK COMP3 enable"]
pub type Bkcmp3eR = crate::BitReader;
#[doc = "Field `BKCMP3E` writer - BRK COMP3 enable"]
pub type Bkcmp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP4E` reader - BRK COMP4 enable"]
pub type Bkcmp4eR = crate::BitReader;
#[doc = "Field `BKCMP4E` writer - BRK COMP4 enable"]
pub type Bkcmp4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP5E` reader - BRK COMP5 enable"]
pub type Bkcmp5eR = crate::BitReader;
#[doc = "Field `BKCMP5E` writer - BRK COMP5 enable"]
pub type Bkcmp5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP6E` reader - BRK COMP6 enable"]
pub type Bkcmp6eR = crate::BitReader;
#[doc = "Field `BKCMP6E` writer - BRK COMP6 enable"]
pub type Bkcmp6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP7E` reader - BRK COMP7 enable"]
pub type Bkcmp7eR = crate::BitReader;
#[doc = "Field `BKCMP7E` writer - BRK COMP7 enable"]
pub type Bkcmp7eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BkinpR = crate::BitReader;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type Bkcmp1pR = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type Bkcmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub type Bkcmp2pR = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub type Bkcmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP3P` reader - BRK COMP3 input polarity"]
pub type Bkcmp3pR = crate::BitReader;
#[doc = "Field `BKCMP3P` writer - BRK COMP3 input polarity"]
pub type Bkcmp3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP4P` reader - BRK COMP4 input polarity"]
pub type Bkcmp4pR = crate::BitReader;
#[doc = "Field `BKCMP4P` writer - BRK COMP4 input polarity"]
pub type Bkcmp4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRSEL` reader - ETR source selection"]
pub type EtrselR = crate::FieldReader;
#[doc = "Field `ETRSEL` writer - ETR source selection"]
pub type EtrselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> Bkcmp1eR {
        Bkcmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> Bkcmp2eR {
        Bkcmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRK COMP3 enable"]
    #[inline(always)]
    pub fn bkcmp3e(&self) -> Bkcmp3eR {
        Bkcmp3eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRK COMP4 enable"]
    #[inline(always)]
    pub fn bkcmp4e(&self) -> Bkcmp4eR {
        Bkcmp4eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK COMP5 enable"]
    #[inline(always)]
    pub fn bkcmp5e(&self) -> Bkcmp5eR {
        Bkcmp5eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK COMP6 enable"]
    #[inline(always)]
    pub fn bkcmp6e(&self) -> Bkcmp6eR {
        Bkcmp6eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRK COMP7 enable"]
    #[inline(always)]
    pub fn bkcmp7e(&self) -> Bkcmp7eR {
        Bkcmp7eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> Bkcmp1pR {
        Bkcmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> Bkcmp2pR {
        Bkcmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity"]
    #[inline(always)]
    pub fn bkcmp3p(&self) -> Bkcmp3pR {
        Bkcmp3pR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BRK COMP4 input polarity"]
    #[inline(always)]
    pub fn bkcmp4p(&self) -> Bkcmp4pR {
        Bkcmp4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> EtrselR {
        EtrselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF1")
            .field("etrsel", &self.etrsel())
            .field("bkcmp4p", &self.bkcmp4p())
            .field("bkcmp3p", &self.bkcmp3p())
            .field("bkcmp2p", &self.bkcmp2p())
            .field("bkcmp1p", &self.bkcmp1p())
            .field("bkinp", &self.bkinp())
            .field("bkcmp7e", &self.bkcmp7e())
            .field("bkcmp6e", &self.bkcmp6e())
            .field("bkcmp5e", &self.bkcmp5e())
            .field("bkcmp4e", &self.bkcmp4e())
            .field("bkcmp3e", &self.bkcmp3e())
            .field("bkcmp2e", &self.bkcmp2e())
            .field("bkcmp1e", &self.bkcmp1e())
            .field("bkine", &self.bkine())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<Af1Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> Bkcmp1eW<Af1Spec> {
        Bkcmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> Bkcmp2eW<Af1Spec> {
        Bkcmp2eW::new(self, 2)
    }
    #[doc = "Bit 3 - BRK COMP3 enable"]
    #[inline(always)]
    pub fn bkcmp3e(&mut self) -> Bkcmp3eW<Af1Spec> {
        Bkcmp3eW::new(self, 3)
    }
    #[doc = "Bit 4 - BRK COMP4 enable"]
    #[inline(always)]
    pub fn bkcmp4e(&mut self) -> Bkcmp4eW<Af1Spec> {
        Bkcmp4eW::new(self, 4)
    }
    #[doc = "Bit 5 - BRK COMP5 enable"]
    #[inline(always)]
    pub fn bkcmp5e(&mut self) -> Bkcmp5eW<Af1Spec> {
        Bkcmp5eW::new(self, 5)
    }
    #[doc = "Bit 6 - BRK COMP6 enable"]
    #[inline(always)]
    pub fn bkcmp6e(&mut self) -> Bkcmp6eW<Af1Spec> {
        Bkcmp6eW::new(self, 6)
    }
    #[doc = "Bit 7 - BRK COMP7 enable"]
    #[inline(always)]
    pub fn bkcmp7e(&mut self) -> Bkcmp7eW<Af1Spec> {
        Bkcmp7eW::new(self, 7)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BkinpW<Af1Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> Bkcmp1pW<Af1Spec> {
        Bkcmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> Bkcmp2pW<Af1Spec> {
        Bkcmp2pW::new(self, 11)
    }
    #[doc = "Bit 12 - BRK COMP3 input polarity"]
    #[inline(always)]
    pub fn bkcmp3p(&mut self) -> Bkcmp3pW<Af1Spec> {
        Bkcmp3pW::new(self, 12)
    }
    #[doc = "Bit 13 - BRK COMP4 input polarity"]
    #[inline(always)]
    pub fn bkcmp4p(&mut self) -> Bkcmp4pW<Af1Spec> {
        Bkcmp4pW::new(self, 13)
    }
    #[doc = "Bits 14:17 - ETR source selection"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> EtrselW<Af1Spec> {
        EtrselW::new(self, 14)
    }
}
#[doc = "TIM alternate function option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af1Spec;
impl crate::RegisterSpec for Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for Af1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for Af1Spec {
    const RESET_VALUE: u32 = 0;
}
