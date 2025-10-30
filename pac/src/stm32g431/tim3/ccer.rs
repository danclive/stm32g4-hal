#[doc = "Register `CCER` reader"]
pub type R = crate::R<CcerSpec>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CcerSpec>;
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable"]
pub type Cc1eR = crate::BitReader;
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable"]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - Capture/Compare 1 output Polarity"]
pub type Cc1pR = crate::BitReader;
#[doc = "Field `CC1P` writer - Capture/Compare 1 output Polarity"]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NE` reader - Capture/Compare 1 complementary output enable"]
pub type Cc1neR = crate::BitReader;
#[doc = "Field `CC1NE` writer - Capture/Compare 1 complementary output enable"]
pub type Cc1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NP` reader - Capture/Compare 1 output Polarity"]
pub type Cc1npR = crate::BitReader;
#[doc = "Field `CC1NP` writer - Capture/Compare 1 output Polarity"]
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output Polarity"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output Polarity"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NE` reader - Capture/Compare 2 complementary output enable"]
pub type Cc2neR = crate::BitReader;
#[doc = "Field `CC2NE` writer - Capture/Compare 2 complementary output enable"]
pub type Cc2neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 output Polarity"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 output Polarity"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Capture/Compare 3 output Polarity"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - Capture/Compare 3 output Polarity"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - Capture/Compare 3 complementary output enable"]
pub type Cc3neR = crate::BitReader;
#[doc = "Field `CC3NE` writer - Capture/Compare 3 complementary output enable"]
pub type Cc3neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Capture/Compare 3 output Polarity"]
pub type Cc3npR = crate::BitReader;
#[doc = "Field `CC3NP` writer - Capture/Compare 3 output Polarity"]
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable"]
pub type Cc4eR = crate::BitReader;
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable"]
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Capture/Compare 3 output Polarity"]
pub type Cc4pR = crate::BitReader;
#[doc = "Field `CC4P` writer - Capture/Compare 3 output Polarity"]
pub type Cc4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NE` reader - Capture/Compare 4 complementary output enable"]
pub type Cc4neR = crate::BitReader;
#[doc = "Field `CC4NE` writer - Capture/Compare 4 complementary output enable"]
pub type Cc4neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Capture/Compare 4 complementary output polarity"]
pub type Cc4npR = crate::BitReader;
#[doc = "Field `CC4NP` writer - Capture/Compare 4 complementary output polarity"]
pub type Cc4npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5E` reader - Capture/Compare 5 output enable"]
pub type Cc5eR = crate::BitReader;
#[doc = "Field `CC5E` writer - Capture/Compare 5 output enable"]
pub type Cc5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5P` reader - Capture/Compare 5 output polarity"]
pub type Cc5pR = crate::BitReader;
#[doc = "Field `CC5P` writer - Capture/Compare 5 output polarity"]
pub type Cc5pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6E` reader - Capture/Compare 6 output enable"]
pub type Cc6eR = crate::BitReader;
#[doc = "Field `CC6E` writer - Capture/Compare 6 output enable"]
pub type Cc6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6P` reader - Capture/Compare 6 output polarity"]
pub type Cc6pR = crate::BitReader;
#[doc = "Field `CC6P` writer - Capture/Compare 6 output polarity"]
pub type Cc6pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&self) -> Cc1neR {
        Cc1neR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2ne(&self) -> Cc2neR {
        Cc2neR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3ne(&self) -> Cc3neR {
        Cc3neR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Capture/Compare 4 complementary output enable"]
    #[inline(always)]
    pub fn cc4ne(&self) -> Cc4neR {
        Cc4neR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/Compare 4 complementary output polarity"]
    #[inline(always)]
    pub fn cc4np(&self) -> Cc4npR {
        Cc4npR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/Compare 5 output enable"]
    #[inline(always)]
    pub fn cc5e(&self) -> Cc5eR {
        Cc5eR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/Compare 5 output polarity"]
    #[inline(always)]
    pub fn cc5p(&self) -> Cc5pR {
        Cc5pR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture/Compare 6 output enable"]
    #[inline(always)]
    pub fn cc6e(&self) -> Cc6eR {
        Cc6eR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture/Compare 6 output polarity"]
    #[inline(always)]
    pub fn cc6p(&self) -> Cc6pR {
        Cc6pR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc6p", &self.cc6p())
            .field("cc6e", &self.cc6e())
            .field("cc5p", &self.cc5p())
            .field("cc5e", &self.cc5e())
            .field("cc4np", &self.cc4np())
            .field("cc4ne", &self.cc4ne())
            .field("cc4p", &self.cc4p())
            .field("cc4e", &self.cc4e())
            .field("cc3np", &self.cc3np())
            .field("cc3ne", &self.cc3ne())
            .field("cc3p", &self.cc3p())
            .field("cc3e", &self.cc3e())
            .field("cc2np", &self.cc2np())
            .field("cc2ne", &self.cc2ne())
            .field("cc2p", &self.cc2p())
            .field("cc2e", &self.cc2e())
            .field("cc1np", &self.cc1np())
            .field("cc1ne", &self.cc1ne())
            .field("cc1p", &self.cc1p())
            .field("cc1e", &self.cc1e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> Cc1eW<'_, CcerSpec> {
        Cc1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<'_, CcerSpec> {
        Cc1pW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&mut self) -> Cc1neW<'_, CcerSpec> {
        Cc1neW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> Cc1npW<'_, CcerSpec> {
        Cc1npW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<'_, CcerSpec> {
        Cc2eW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<'_, CcerSpec> {
        Cc2pW::new(self, 5)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2ne(&mut self) -> Cc2neW<'_, CcerSpec> {
        Cc2neW::new(self, 6)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> Cc2npW<'_, CcerSpec> {
        Cc2npW::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> Cc3eW<'_, CcerSpec> {
        Cc3eW::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> Cc3pW<'_, CcerSpec> {
        Cc3pW::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3ne(&mut self) -> Cc3neW<'_, CcerSpec> {
        Cc3neW::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> Cc3npW<'_, CcerSpec> {
        Cc3npW::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> Cc4eW<'_, CcerSpec> {
        Cc4eW::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> Cc4pW<'_, CcerSpec> {
        Cc4pW::new(self, 13)
    }
    #[doc = "Bit 14 - Capture/Compare 4 complementary output enable"]
    #[inline(always)]
    pub fn cc4ne(&mut self) -> Cc4neW<'_, CcerSpec> {
        Cc4neW::new(self, 14)
    }
    #[doc = "Bit 15 - Capture/Compare 4 complementary output polarity"]
    #[inline(always)]
    pub fn cc4np(&mut self) -> Cc4npW<'_, CcerSpec> {
        Cc4npW::new(self, 15)
    }
    #[doc = "Bit 16 - Capture/Compare 5 output enable"]
    #[inline(always)]
    pub fn cc5e(&mut self) -> Cc5eW<'_, CcerSpec> {
        Cc5eW::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/Compare 5 output polarity"]
    #[inline(always)]
    pub fn cc5p(&mut self) -> Cc5pW<'_, CcerSpec> {
        Cc5pW::new(self, 17)
    }
    #[doc = "Bit 20 - Capture/Compare 6 output enable"]
    #[inline(always)]
    pub fn cc6e(&mut self) -> Cc6eW<'_, CcerSpec> {
        Cc6eW::new(self, 20)
    }
    #[doc = "Bit 21 - Capture/Compare 6 output polarity"]
    #[inline(always)]
    pub fn cc6p(&mut self) -> Cc6pW<'_, CcerSpec> {
        Cc6pW::new(self, 21)
    }
}
#[doc = "capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerSpec;
impl crate::RegisterSpec for CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CcerSpec {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CcerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CcerSpec {}
