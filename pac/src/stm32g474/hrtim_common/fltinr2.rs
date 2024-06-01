#[doc = "Register `FLTINR2` reader"]
pub type R = crate::R<Fltinr2Spec>;
#[doc = "Register `FLTINR2` writer"]
pub type W = crate::W<Fltinr2Spec>;
#[doc = "Field `FLT5E` reader - FLT5E"]
pub type Flt5eR = crate::BitReader;
#[doc = "Field `FLT5E` writer - FLT5E"]
pub type Flt5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5P` reader - FLT5P"]
pub type Flt5pR = crate::BitReader;
#[doc = "Field `FLT5P` writer - FLT5P"]
pub type Flt5pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5SRC` reader - FLT5SRC"]
pub type Flt5srcR = crate::BitReader;
#[doc = "Field `FLT5SRC` writer - FLT5SRC"]
pub type Flt5srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5F` reader - FLT5F"]
pub type Flt5fR = crate::FieldReader;
#[doc = "Field `FLT5F` writer - FLT5F"]
pub type Flt5fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT5LCK` reader - FLT5LCK"]
pub type Flt5lckR = crate::BitReader;
#[doc = "Field `FLT5LCK` writer - FLT5LCK"]
pub type Flt5lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6E` reader - FLT6E"]
pub type Flt6eR = crate::BitReader;
#[doc = "Field `FLT6E` writer - FLT6E"]
pub type Flt6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6P` reader - FLT6P"]
pub type Flt6pR = crate::BitReader;
#[doc = "Field `FLT6P` writer - FLT6P"]
pub type Flt6pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6SRC_0` reader - FLT6F"]
pub type Flt6src0R = crate::BitReader;
#[doc = "Field `FLT6SRC_0` writer - FLT6F"]
pub type Flt6src0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6F` reader - FLT6F"]
pub type Flt6fR = crate::FieldReader;
#[doc = "Field `FLT6F` writer - FLT6F"]
pub type Flt6fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT6LCK` reader - FLT6LCK"]
pub type Flt6lckR = crate::BitReader;
#[doc = "Field `FLT6LCK` writer - FLT6LCK"]
pub type Flt6lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1SRC_1` reader - FLT1SRC_1"]
pub type Flt1src1R = crate::BitReader;
#[doc = "Field `FLT1SRC_1` writer - FLT1SRC_1"]
pub type Flt1src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2SRC_1` reader - FLT2SRC_1"]
pub type Flt2src1R = crate::BitReader;
#[doc = "Field `FLT2SRC_1` writer - FLT2SRC_1"]
pub type Flt2src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3SRC_1` reader - FLT3SRC_1"]
pub type Flt3src1R = crate::BitReader;
#[doc = "Field `FLT3SRC_1` writer - FLT3SRC_1"]
pub type Flt3src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4SRC_1` reader - FLT4SRC_1"]
pub type Flt4src1R = crate::BitReader;
#[doc = "Field `FLT4SRC_1` writer - FLT4SRC_1"]
pub type Flt4src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5SRC_1` reader - FLT5SRC_1"]
pub type Flt5src1R = crate::BitReader;
#[doc = "Field `FLT5SRC_1` writer - FLT5SRC_1"]
pub type Flt5src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6SRC_1` reader - FLT6SRC"]
pub type Flt6src1R = crate::BitReader;
#[doc = "Field `FLT6SRC_1` writer - FLT6SRC"]
pub type Flt6src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTSD` reader - FLTSD"]
pub type FltsdR = crate::FieldReader;
#[doc = "Field `FLTSD` writer - FLTSD"]
pub type FltsdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> Flt5eR {
        Flt5eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> Flt5pR {
        Flt5pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> Flt5srcR {
        Flt5srcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> Flt5fR {
        Flt5fR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> Flt5lckR {
        Flt5lckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&self) -> Flt6eR {
        Flt6eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&self) -> Flt6pR {
        Flt6pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&self) -> Flt6src0R {
        Flt6src0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&self) -> Flt6fR {
        Flt6fR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&self) -> Flt6lckR {
        Flt6lckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&self) -> Flt1src1R {
        Flt1src1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&self) -> Flt2src1R {
        Flt2src1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&self) -> Flt3src1R {
        Flt3src1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&self) -> Flt4src1R {
        Flt4src1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&self) -> Flt5src1R {
        Flt5src1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&self) -> Flt6src1R {
        Flt6src1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FltsdR {
        FltsdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR2")
            .field("fltsd", &self.fltsd())
            .field("flt6src_1", &self.flt6src_1())
            .field("flt5src_1", &self.flt5src_1())
            .field("flt4src_1", &self.flt4src_1())
            .field("flt3src_1", &self.flt3src_1())
            .field("flt2src_1", &self.flt2src_1())
            .field("flt1src_1", &self.flt1src_1())
            .field("flt6lck", &self.flt6lck())
            .field("flt6f", &self.flt6f())
            .field("flt6src_0", &self.flt6src_0())
            .field("flt6p", &self.flt6p())
            .field("flt6e", &self.flt6e())
            .field("flt5lck", &self.flt5lck())
            .field("flt5f", &self.flt5f())
            .field("flt5src", &self.flt5src())
            .field("flt5p", &self.flt5p())
            .field("flt5e", &self.flt5e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    #[must_use]
    pub fn flt5e(&mut self) -> Flt5eW<Fltinr2Spec> {
        Flt5eW::new(self, 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    #[must_use]
    pub fn flt5p(&mut self) -> Flt5pW<Fltinr2Spec> {
        Flt5pW::new(self, 1)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src(&mut self) -> Flt5srcW<Fltinr2Spec> {
        Flt5srcW::new(self, 2)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    #[must_use]
    pub fn flt5f(&mut self) -> Flt5fW<Fltinr2Spec> {
        Flt5fW::new(self, 3)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt5lck(&mut self) -> Flt5lckW<Fltinr2Spec> {
        Flt5lckW::new(self, 7)
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    #[must_use]
    pub fn flt6e(&mut self) -> Flt6eW<Fltinr2Spec> {
        Flt6eW::new(self, 8)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    #[must_use]
    pub fn flt6p(&mut self) -> Flt6pW<Fltinr2Spec> {
        Flt6pW::new(self, 9)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    #[must_use]
    pub fn flt6src_0(&mut self) -> Flt6src0W<Fltinr2Spec> {
        Flt6src0W::new(self, 10)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    #[must_use]
    pub fn flt6f(&mut self) -> Flt6fW<Fltinr2Spec> {
        Flt6fW::new(self, 11)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt6lck(&mut self) -> Flt6lckW<Fltinr2Spec> {
        Flt6lckW::new(self, 15)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt1src_1(&mut self) -> Flt1src1W<Fltinr2Spec> {
        Flt1src1W::new(self, 16)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt2src_1(&mut self) -> Flt2src1W<Fltinr2Spec> {
        Flt2src1W::new(self, 17)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt3src_1(&mut self) -> Flt3src1W<Fltinr2Spec> {
        Flt3src1W::new(self, 18)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt4src_1(&mut self) -> Flt4src1W<Fltinr2Spec> {
        Flt4src1W::new(self, 19)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src_1(&mut self) -> Flt5src1W<Fltinr2Spec> {
        Flt5src1W::new(self, 20)
    }
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt6src_1(&mut self) -> Flt6src1W<Fltinr2Spec> {
        Flt6src1W::new(self, 21)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    #[must_use]
    pub fn fltsd(&mut self) -> FltsdW<Fltinr2Spec> {
        FltsdW::new(self, 24)
    }
}
#[doc = "HRTIM Fault Input Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fltinr2Spec;
impl crate::RegisterSpec for Fltinr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr2::R`](R) reader structure"]
impl crate::Readable for Fltinr2Spec {}
#[doc = "`write(|w| ..)` method takes [`fltinr2::W`](W) writer structure"]
impl crate::Writable for Fltinr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR2 to value 0"]
impl crate::Resettable for Fltinr2Spec {
    const RESET_VALUE: u32 = 0;
}
