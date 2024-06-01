#[doc = "Register `FLTINR1` reader"]
pub type R = crate::R<Fltinr1Spec>;
#[doc = "Register `FLTINR1` writer"]
pub type W = crate::W<Fltinr1Spec>;
#[doc = "Field `FLT1E` reader - FLT1E"]
pub type Flt1eR = crate::BitReader;
#[doc = "Field `FLT1E` writer - FLT1E"]
pub type Flt1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1P` reader - FLT1P"]
pub type Flt1pR = crate::BitReader;
#[doc = "Field `FLT1P` writer - FLT1P"]
pub type Flt1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1SRC` reader - FLT1SRC"]
pub type Flt1srcR = crate::BitReader;
#[doc = "Field `FLT1SRC` writer - FLT1SRC"]
pub type Flt1srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1F` reader - FLT1F"]
pub type Flt1fR = crate::FieldReader;
#[doc = "Field `FLT1F` writer - FLT1F"]
pub type Flt1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT1LCK` reader - FLT1LCK"]
pub type Flt1lckR = crate::BitReader;
#[doc = "Field `FLT1LCK` writer - FLT1LCK"]
pub type Flt1lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2E` reader - FLT2E"]
pub type Flt2eR = crate::BitReader;
#[doc = "Field `FLT2E` writer - FLT2E"]
pub type Flt2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2P` reader - FLT2P"]
pub type Flt2pR = crate::BitReader;
#[doc = "Field `FLT2P` writer - FLT2P"]
pub type Flt2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2SRC` reader - FLT2SRC"]
pub type Flt2srcR = crate::BitReader;
#[doc = "Field `FLT2SRC` writer - FLT2SRC"]
pub type Flt2srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2F` reader - FLT2F"]
pub type Flt2fR = crate::FieldReader;
#[doc = "Field `FLT2F` writer - FLT2F"]
pub type Flt2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT2LCK` reader - FLT2LCK"]
pub type Flt2lckR = crate::BitReader;
#[doc = "Field `FLT2LCK` writer - FLT2LCK"]
pub type Flt2lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3E` reader - FLT3E"]
pub type Flt3eR = crate::BitReader;
#[doc = "Field `FLT3E` writer - FLT3E"]
pub type Flt3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3P` reader - FLT3P"]
pub type Flt3pR = crate::BitReader;
#[doc = "Field `FLT3P` writer - FLT3P"]
pub type Flt3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3SRC` reader - FLT3SRC"]
pub type Flt3srcR = crate::BitReader;
#[doc = "Field `FLT3SRC` writer - FLT3SRC"]
pub type Flt3srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3F` reader - FLT3F"]
pub type Flt3fR = crate::FieldReader;
#[doc = "Field `FLT3F` writer - FLT3F"]
pub type Flt3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT3LCK` reader - FLT3LCK"]
pub type Flt3lckR = crate::BitReader;
#[doc = "Field `FLT3LCK` writer - FLT3LCK"]
pub type Flt3lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4E` reader - FLT4E"]
pub type Flt4eR = crate::BitReader;
#[doc = "Field `FLT4E` writer - FLT4E"]
pub type Flt4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4P` reader - FLT4P"]
pub type Flt4pR = crate::BitReader;
#[doc = "Field `FLT4P` writer - FLT4P"]
pub type Flt4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4SRC` reader - FLT4SRC"]
pub type Flt4srcR = crate::BitReader;
#[doc = "Field `FLT4SRC` writer - FLT4SRC"]
pub type Flt4srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4F` reader - FLT4F"]
pub type Flt4fR = crate::FieldReader;
#[doc = "Field `FLT4F` writer - FLT4F"]
pub type Flt4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT4LCK` reader - FLT4LCK"]
pub type Flt4lckR = crate::BitReader;
#[doc = "Field `FLT4LCK` writer - FLT4LCK"]
pub type Flt4lckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&self) -> Flt1eR {
        Flt1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&self) -> Flt1pR {
        Flt1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&self) -> Flt1srcR {
        Flt1srcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&self) -> Flt1fR {
        Flt1fR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&self) -> Flt1lckR {
        Flt1lckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&self) -> Flt2eR {
        Flt2eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&self) -> Flt2pR {
        Flt2pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&self) -> Flt2srcR {
        Flt2srcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&self) -> Flt2fR {
        Flt2fR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&self) -> Flt2lckR {
        Flt2lckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&self) -> Flt3eR {
        Flt3eR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&self) -> Flt3pR {
        Flt3pR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&self) -> Flt3srcR {
        Flt3srcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&self) -> Flt3fR {
        Flt3fR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&self) -> Flt3lckR {
        Flt3lckR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&self) -> Flt4eR {
        Flt4eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&self) -> Flt4pR {
        Flt4pR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&self) -> Flt4srcR {
        Flt4srcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&self) -> Flt4fR {
        Flt4fR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&self) -> Flt4lckR {
        Flt4lckR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR1")
            .field("flt4lck", &self.flt4lck())
            .field("flt4f", &self.flt4f())
            .field("flt4src", &self.flt4src())
            .field("flt4p", &self.flt4p())
            .field("flt4e", &self.flt4e())
            .field("flt3lck", &self.flt3lck())
            .field("flt3f", &self.flt3f())
            .field("flt3src", &self.flt3src())
            .field("flt3p", &self.flt3p())
            .field("flt3e", &self.flt3e())
            .field("flt2lck", &self.flt2lck())
            .field("flt2f", &self.flt2f())
            .field("flt2src", &self.flt2src())
            .field("flt2p", &self.flt2p())
            .field("flt2e", &self.flt2e())
            .field("flt1lck", &self.flt1lck())
            .field("flt1f", &self.flt1f())
            .field("flt1src", &self.flt1src())
            .field("flt1p", &self.flt1p())
            .field("flt1e", &self.flt1e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    #[must_use]
    pub fn flt1e(&mut self) -> Flt1eW<Fltinr1Spec> {
        Flt1eW::new(self, 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    #[must_use]
    pub fn flt1p(&mut self) -> Flt1pW<Fltinr1Spec> {
        Flt1pW::new(self, 1)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt1src(&mut self) -> Flt1srcW<Fltinr1Spec> {
        Flt1srcW::new(self, 2)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    #[must_use]
    pub fn flt1f(&mut self) -> Flt1fW<Fltinr1Spec> {
        Flt1fW::new(self, 3)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt1lck(&mut self) -> Flt1lckW<Fltinr1Spec> {
        Flt1lckW::new(self, 7)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    #[must_use]
    pub fn flt2e(&mut self) -> Flt2eW<Fltinr1Spec> {
        Flt2eW::new(self, 8)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    #[must_use]
    pub fn flt2p(&mut self) -> Flt2pW<Fltinr1Spec> {
        Flt2pW::new(self, 9)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt2src(&mut self) -> Flt2srcW<Fltinr1Spec> {
        Flt2srcW::new(self, 10)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    #[must_use]
    pub fn flt2f(&mut self) -> Flt2fW<Fltinr1Spec> {
        Flt2fW::new(self, 11)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt2lck(&mut self) -> Flt2lckW<Fltinr1Spec> {
        Flt2lckW::new(self, 15)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    #[must_use]
    pub fn flt3e(&mut self) -> Flt3eW<Fltinr1Spec> {
        Flt3eW::new(self, 16)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    #[must_use]
    pub fn flt3p(&mut self) -> Flt3pW<Fltinr1Spec> {
        Flt3pW::new(self, 17)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt3src(&mut self) -> Flt3srcW<Fltinr1Spec> {
        Flt3srcW::new(self, 18)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    #[must_use]
    pub fn flt3f(&mut self) -> Flt3fW<Fltinr1Spec> {
        Flt3fW::new(self, 19)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt3lck(&mut self) -> Flt3lckW<Fltinr1Spec> {
        Flt3lckW::new(self, 23)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    #[must_use]
    pub fn flt4e(&mut self) -> Flt4eW<Fltinr1Spec> {
        Flt4eW::new(self, 24)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    #[must_use]
    pub fn flt4p(&mut self) -> Flt4pW<Fltinr1Spec> {
        Flt4pW::new(self, 25)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt4src(&mut self) -> Flt4srcW<Fltinr1Spec> {
        Flt4srcW::new(self, 26)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    #[must_use]
    pub fn flt4f(&mut self) -> Flt4fW<Fltinr1Spec> {
        Flt4fW::new(self, 27)
    }
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt4lck(&mut self) -> Flt4lckW<Fltinr1Spec> {
        Flt4lckW::new(self, 31)
    }
}
#[doc = "HRTIM Fault Input Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fltinr1Spec;
impl crate::RegisterSpec for Fltinr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr1::R`](R) reader structure"]
impl crate::Readable for Fltinr1Spec {}
#[doc = "`write(|w| ..)` method takes [`fltinr1::W`](W) writer structure"]
impl crate::Writable for Fltinr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR1 to value 0"]
impl crate::Resettable for Fltinr1Spec {
    const RESET_VALUE: u32 = 0;
}
