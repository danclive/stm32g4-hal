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
#[doc = "Field `CC2P` reader - Capture/Compare 2 output polarity"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output polarity"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 complementary output polarity"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 complementary output polarity"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - Capture/Compare 2 output polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc2np", &self.cc2np())
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
    pub fn cc1e(&mut self) -> Cc1eW<CcerSpec> {
        Cc1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<CcerSpec> {
        Cc1pW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&mut self) -> Cc1neW<CcerSpec> {
        Cc1neW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> Cc1npW<CcerSpec> {
        Cc1npW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<CcerSpec> {
        Cc2eW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<CcerSpec> {
        Cc2pW::new(self, 5)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> Cc2npW<CcerSpec> {
        Cc2npW::new(self, 7)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CcerSpec {
    const RESET_VALUE: u32 = 0;
}
