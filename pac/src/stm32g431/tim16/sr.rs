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
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type ComifR = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type ComifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BifR = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> ComifR {
        ComifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BifR {
        BifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("cc1of", &self.cc1of())
            .field("bif", &self.bif())
            .field("comif", &self.comif())
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
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&mut self) -> ComifW<SrSpec> {
        ComifW::new(self, 5)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&mut self) -> BifW<SrSpec> {
        BifW::new(self, 7)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<SrSpec> {
        Cc1ofW::new(self, 9)
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
