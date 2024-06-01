#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `TAMP1NOER` reader - TAMP1NOER"]
pub type Tamp1noerR = crate::BitReader;
#[doc = "Field `TAMP1NOER` writer - TAMP1NOER"]
pub type Tamp1noerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2NOER` reader - TAMP2NOER"]
pub type Tamp2noerR = crate::BitReader;
#[doc = "Field `TAMP2NOER` writer - TAMP2NOER"]
pub type Tamp2noerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3NOER` reader - TAMP3NOER"]
pub type Tamp3noerR = crate::BitReader;
#[doc = "Field `TAMP3NOER` writer - TAMP3NOER"]
pub type Tamp3noerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1MSK` reader - TAMP1MSK"]
pub type Tamp1mskR = crate::BitReader;
#[doc = "Field `TAMP1MSK` writer - TAMP1MSK"]
pub type Tamp1mskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2MSK` reader - TAMP2MSK"]
pub type Tamp2mskR = crate::BitReader;
#[doc = "Field `TAMP2MSK` writer - TAMP2MSK"]
pub type Tamp2mskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3MSK` reader - TAMP3MSK"]
pub type Tamp3mskR = crate::BitReader;
#[doc = "Field `TAMP3MSK` writer - TAMP3MSK"]
pub type Tamp3mskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - TAMP1TRG"]
pub type Tamp1trgR = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - TAMP1TRG"]
pub type Tamp1trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2TRG` reader - TAMP2TRG"]
pub type Tamp2trgR = crate::BitReader;
#[doc = "Field `TAMP2TRG` writer - TAMP2TRG"]
pub type Tamp2trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3TRG` reader - TAMP3TRG"]
pub type Tamp3trgR = crate::BitReader;
#[doc = "Field `TAMP3TRG` writer - TAMP3TRG"]
pub type Tamp3trgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> Tamp1noerR {
        Tamp1noerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> Tamp2noerR {
        Tamp2noerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> Tamp3noerR {
        Tamp3noerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&self) -> Tamp1mskR {
        Tamp1mskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&self) -> Tamp2mskR {
        Tamp2mskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&self) -> Tamp3mskR {
        Tamp3mskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> Tamp1trgR {
        Tamp1trgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> Tamp2trgR {
        Tamp2trgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> Tamp3trgR {
        Tamp3trgR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tamp1noer", &self.tamp1noer())
            .field("tamp2noer", &self.tamp2noer())
            .field("tamp3noer", &self.tamp3noer())
            .field("tamp1msk", &self.tamp1msk())
            .field("tamp2msk", &self.tamp2msk())
            .field("tamp3msk", &self.tamp3msk())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp3trg", &self.tamp3trg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> Tamp1noerW<Cr2Spec> {
        Tamp1noerW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> Tamp2noerW<Cr2Spec> {
        Tamp2noerW::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noer(&mut self) -> Tamp3noerW<Cr2Spec> {
        Tamp3noerW::new(self, 2)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> Tamp1mskW<Cr2Spec> {
        Tamp1mskW::new(self, 16)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> Tamp2mskW<Cr2Spec> {
        Tamp2mskW::new(self, 17)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> Tamp3mskW<Cr2Spec> {
        Tamp3mskW::new(self, 18)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> Tamp1trgW<Cr2Spec> {
        Tamp1trgW::new(self, 24)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> Tamp2trgW<Cr2Spec> {
        Tamp2trgW::new(self, 25)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> Tamp3trgW<Cr2Spec> {
        Tamp3trgW::new(self, 26)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
