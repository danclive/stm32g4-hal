#[doc = "Register `PDMDLY` reader"]
pub type R = crate::R<PdmdlySpec>;
#[doc = "Register `PDMDLY` writer"]
pub type W = crate::W<PdmdlySpec>;
#[doc = "Field `DLYM1L` reader - DLYM1L"]
pub type Dlym1lR = crate::FieldReader;
#[doc = "Field `DLYM1L` writer - DLYM1L"]
pub type Dlym1lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM1R` reader - DLYM1R"]
pub type Dlym1rR = crate::FieldReader;
#[doc = "Field `DLYM1R` writer - DLYM1R"]
pub type Dlym1rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2L` reader - DLYM2L"]
pub type Dlym2lR = crate::FieldReader;
#[doc = "Field `DLYM2L` writer - DLYM2L"]
pub type Dlym2lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2R` reader - DLYM2R"]
pub type Dlym2rR = crate::FieldReader;
#[doc = "Field `DLYM2R` writer - DLYM2R"]
pub type Dlym2rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3L` reader - DLYM3L"]
pub type Dlym3lR = crate::FieldReader;
#[doc = "Field `DLYM3L` writer - DLYM3L"]
pub type Dlym3lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3R` reader - DLYM3R"]
pub type Dlym3rR = crate::FieldReader;
#[doc = "Field `DLYM3R` writer - DLYM3R"]
pub type Dlym3rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4L` reader - DLYM4L"]
pub type Dlym4lR = crate::FieldReader;
#[doc = "Field `DLYM4L` writer - DLYM4L"]
pub type Dlym4lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4R` reader - DLYM4R"]
pub type Dlym4rR = crate::FieldReader;
#[doc = "Field `DLYM4R` writer - DLYM4R"]
pub type Dlym4rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    pub fn dlym1l(&self) -> Dlym1lR {
        Dlym1lR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    pub fn dlym1r(&self) -> Dlym1rR {
        Dlym1rR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    pub fn dlym2l(&self) -> Dlym2lR {
        Dlym2lR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    pub fn dlym2r(&self) -> Dlym2rR {
        Dlym2rR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    pub fn dlym3l(&self) -> Dlym3lR {
        Dlym3lR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    pub fn dlym3r(&self) -> Dlym3rR {
        Dlym3rR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    pub fn dlym4l(&self) -> Dlym4lR {
        Dlym4lR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    pub fn dlym4r(&self) -> Dlym4rR {
        Dlym4rR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMDLY")
            .field("dlym1l", &self.dlym1l())
            .field("dlym1r", &self.dlym1r())
            .field("dlym2l", &self.dlym2l())
            .field("dlym2r", &self.dlym2r())
            .field("dlym3l", &self.dlym3l())
            .field("dlym3r", &self.dlym3r())
            .field("dlym4l", &self.dlym4l())
            .field("dlym4r", &self.dlym4r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1l(&mut self) -> Dlym1lW<PdmdlySpec> {
        Dlym1lW::new(self, 0)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1r(&mut self) -> Dlym1rW<PdmdlySpec> {
        Dlym1rW::new(self, 4)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2l(&mut self) -> Dlym2lW<PdmdlySpec> {
        Dlym2lW::new(self, 8)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2r(&mut self) -> Dlym2rW<PdmdlySpec> {
        Dlym2rW::new(self, 12)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3l(&mut self) -> Dlym3lW<PdmdlySpec> {
        Dlym3lW::new(self, 16)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3r(&mut self) -> Dlym3rW<PdmdlySpec> {
        Dlym3rW::new(self, 20)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4l(&mut self) -> Dlym4lW<PdmdlySpec> {
        Dlym4lW::new(self, 24)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4r(&mut self) -> Dlym4rW<PdmdlySpec> {
        Dlym4rW::new(self, 28)
    }
}
#[doc = "PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmdlySpec;
impl crate::RegisterSpec for PdmdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdmdly::R`](R) reader structure"]
impl crate::Readable for PdmdlySpec {}
#[doc = "`write(|w| ..)` method takes [`pdmdly::W`](W) writer structure"]
impl crate::Writable for PdmdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDMDLY to value 0"]
impl crate::Resettable for PdmdlySpec {
    const RESET_VALUE: u32 = 0;
}
