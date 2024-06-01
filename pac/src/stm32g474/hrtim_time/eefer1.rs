#[doc = "Register `EEFER1` reader"]
pub type R = crate::R<Eefer1Spec>;
#[doc = "Register `EEFER1` writer"]
pub type W = crate::W<Eefer1Spec>;
#[doc = "Field `EE1LTCH` reader - External Event 1 latch"]
pub type Ee1ltchR = crate::BitReader;
#[doc = "Field `EE1LTCH` writer - External Event 1 latch"]
pub type Ee1ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE1FLTR` reader - External Event 1 filter"]
pub type Ee1fltrR = crate::FieldReader;
#[doc = "Field `EE1FLTR` writer - External Event 1 filter"]
pub type Ee1fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE2LTCH` reader - External Event 2 latch"]
pub type Ee2ltchR = crate::BitReader;
#[doc = "Field `EE2LTCH` writer - External Event 2 latch"]
pub type Ee2ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE2FLTR` reader - External Event 2 filter"]
pub type Ee2fltrR = crate::FieldReader;
#[doc = "Field `EE2FLTR` writer - External Event 2 filter"]
pub type Ee2fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE3LTCH` reader - External Event 3 latch"]
pub type Ee3ltchR = crate::BitReader;
#[doc = "Field `EE3LTCH` writer - External Event 3 latch"]
pub type Ee3ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE3FLTR` reader - External Event 3 filter"]
pub type Ee3fltrR = crate::FieldReader;
#[doc = "Field `EE3FLTR` writer - External Event 3 filter"]
pub type Ee3fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE4LTCH` reader - External Event 4 latch"]
pub type Ee4ltchR = crate::BitReader;
#[doc = "Field `EE4LTCH` writer - External Event 4 latch"]
pub type Ee4ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE4FLTR` reader - External Event 4 filter"]
pub type Ee4fltrR = crate::FieldReader;
#[doc = "Field `EE4FLTR` writer - External Event 4 filter"]
pub type Ee4fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE5LTCH` reader - External Event 5 latch"]
pub type Ee5ltchR = crate::BitReader;
#[doc = "Field `EE5LTCH` writer - External Event 5 latch"]
pub type Ee5ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE5FLTR` reader - External Event 5 filter"]
pub type Ee5fltrR = crate::FieldReader;
#[doc = "Field `EE5FLTR` writer - External Event 5 filter"]
pub type Ee5fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&self) -> Ee1ltchR {
        Ee1ltchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&self) -> Ee1fltrR {
        Ee1fltrR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&self) -> Ee2ltchR {
        Ee2ltchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&self) -> Ee2fltrR {
        Ee2fltrR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&self) -> Ee3ltchR {
        Ee3ltchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&self) -> Ee3fltrR {
        Ee3fltrR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&self) -> Ee4ltchR {
        Ee4ltchR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&self) -> Ee4fltrR {
        Ee4fltrR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&self) -> Ee5ltchR {
        Ee5ltchR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&self) -> Ee5fltrR {
        Ee5fltrR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEFER1")
            .field("ee5fltr", &self.ee5fltr())
            .field("ee5ltch", &self.ee5ltch())
            .field("ee4fltr", &self.ee4fltr())
            .field("ee4ltch", &self.ee4ltch())
            .field("ee3fltr", &self.ee3fltr())
            .field("ee3ltch", &self.ee3ltch())
            .field("ee2fltr", &self.ee2fltr())
            .field("ee2ltch", &self.ee2ltch())
            .field("ee1fltr", &self.ee1fltr())
            .field("ee1ltch", &self.ee1ltch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee1ltch(&mut self) -> Ee1ltchW<Eefer1Spec> {
        Ee1ltchW::new(self, 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee1fltr(&mut self) -> Ee1fltrW<Eefer1Spec> {
        Ee1fltrW::new(self, 1)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee2ltch(&mut self) -> Ee2ltchW<Eefer1Spec> {
        Ee2ltchW::new(self, 6)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee2fltr(&mut self) -> Ee2fltrW<Eefer1Spec> {
        Ee2fltrW::new(self, 7)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee3ltch(&mut self) -> Ee3ltchW<Eefer1Spec> {
        Ee3ltchW::new(self, 12)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee3fltr(&mut self) -> Ee3fltrW<Eefer1Spec> {
        Ee3fltrW::new(self, 13)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee4ltch(&mut self) -> Ee4ltchW<Eefer1Spec> {
        Ee4ltchW::new(self, 18)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee4fltr(&mut self) -> Ee4fltrW<Eefer1Spec> {
        Ee4fltrW::new(self, 19)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee5ltch(&mut self) -> Ee5ltchW<Eefer1Spec> {
        Ee5ltchW::new(self, 24)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee5fltr(&mut self) -> Ee5fltrW<Eefer1Spec> {
        Ee5fltrW::new(self, 25)
    }
}
#[doc = "Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eefer1Spec;
impl crate::RegisterSpec for Eefer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefer1::R`](R) reader structure"]
impl crate::Readable for Eefer1Spec {}
#[doc = "`write(|w| ..)` method takes [`eefer1::W`](W) writer structure"]
impl crate::Writable for Eefer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEFER1 to value 0"]
impl crate::Resettable for Eefer1Spec {
    const RESET_VALUE: u32 = 0;
}
