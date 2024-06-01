#[doc = "Register `FLTINR4` reader"]
pub type R = crate::R<Fltinr4Spec>;
#[doc = "Register `FLTINR4` writer"]
pub type W = crate::W<Fltinr4Spec>;
#[doc = "Field `FLT5BLKE` reader - FLT5BLKE"]
pub type Flt5blkeR = crate::BitReader;
#[doc = "Field `FLT5BLKE` writer - FLT5BLKE"]
pub type Flt5blkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5BLKS` reader - FLT5BLKS"]
pub type Flt5blksR = crate::BitReader;
#[doc = "Field `FLT5BLKS` writer - FLT5BLKS"]
pub type Flt5blksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5CNT` reader - FLT5CNT"]
pub type Flt5cntR = crate::FieldReader;
#[doc = "Field `FLT5CNT` writer - FLT5CNT"]
pub type Flt5cntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT5CRES` reader - FLT5CRES"]
pub type Flt5cresR = crate::BitReader;
#[doc = "Field `FLT5CRES` writer - FLT5CRES"]
pub type Flt5cresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5RSTM` reader - FLT5RSTM"]
pub type Flt5rstmR = crate::BitReader;
#[doc = "Field `FLT5RSTM` writer - FLT5RSTM"]
pub type Flt5rstmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6BLKE` reader - FLT6BLKE"]
pub type Flt6blkeR = crate::BitReader;
#[doc = "Field `FLT6BLKE` writer - FLT6BLKE"]
pub type Flt6blkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6BLKS` reader - FLT6BLKS"]
pub type Flt6blksR = crate::BitReader;
#[doc = "Field `FLT6BLKS` writer - FLT6BLKS"]
pub type Flt6blksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6CNT` reader - FLT6CNT"]
pub type Flt6cntR = crate::FieldReader;
#[doc = "Field `FLT6CNT` writer - FLT6CNT"]
pub type Flt6cntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT6CRES` reader - FLT6CRES"]
pub type Flt6cresR = crate::BitReader;
#[doc = "Field `FLT6CRES` writer - FLT6CRES"]
pub type Flt6cresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6RSTM` reader - FLT6RSTM"]
pub type Flt6rstmR = crate::BitReader;
#[doc = "Field `FLT6RSTM` writer - FLT6RSTM"]
pub type Flt6rstmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    pub fn flt5blke(&self) -> Flt5blkeR {
        Flt5blkeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    pub fn flt5blks(&self) -> Flt5blksR {
        Flt5blksR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    pub fn flt5cnt(&self) -> Flt5cntR {
        Flt5cntR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    pub fn flt5cres(&self) -> Flt5cresR {
        Flt5cresR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    pub fn flt5rstm(&self) -> Flt5rstmR {
        Flt5rstmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    pub fn flt6blke(&self) -> Flt6blkeR {
        Flt6blkeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    pub fn flt6blks(&self) -> Flt6blksR {
        Flt6blksR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    pub fn flt6cnt(&self) -> Flt6cntR {
        Flt6cntR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    pub fn flt6cres(&self) -> Flt6cresR {
        Flt6cresR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    pub fn flt6rstm(&self) -> Flt6rstmR {
        Flt6rstmR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR4")
            .field("flt6rstm", &self.flt6rstm())
            .field("flt6cres", &self.flt6cres())
            .field("flt6cnt", &self.flt6cnt())
            .field("flt6blks", &self.flt6blks())
            .field("flt6blke", &self.flt6blke())
            .field("flt5rstm", &self.flt5rstm())
            .field("flt5cres", &self.flt5cres())
            .field("flt5cnt", &self.flt5cnt())
            .field("flt5blks", &self.flt5blks())
            .field("flt5blke", &self.flt5blke())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt5blke(&mut self) -> Flt5blkeW<Fltinr4Spec> {
        Flt5blkeW::new(self, 0)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt5blks(&mut self) -> Flt5blksW<Fltinr4Spec> {
        Flt5blksW::new(self, 1)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt5cnt(&mut self) -> Flt5cntW<Fltinr4Spec> {
        Flt5cntW::new(self, 2)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt5cres(&mut self) -> Flt5cresW<Fltinr4Spec> {
        Flt5cresW::new(self, 6)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt5rstm(&mut self) -> Flt5rstmW<Fltinr4Spec> {
        Flt5rstmW::new(self, 7)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    #[must_use]
    pub fn flt6blke(&mut self) -> Flt6blkeW<Fltinr4Spec> {
        Flt6blkeW::new(self, 8)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    #[must_use]
    pub fn flt6blks(&mut self) -> Flt6blksW<Fltinr4Spec> {
        Flt6blksW::new(self, 9)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    #[must_use]
    pub fn flt6cnt(&mut self) -> Flt6cntW<Fltinr4Spec> {
        Flt6cntW::new(self, 10)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    #[must_use]
    pub fn flt6cres(&mut self) -> Flt6cresW<Fltinr4Spec> {
        Flt6cresW::new(self, 14)
    }
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    #[must_use]
    pub fn flt6rstm(&mut self) -> Flt6rstmW<Fltinr4Spec> {
        Flt6rstmW::new(self, 15)
    }
}
#[doc = "HRTIM Fault Input Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fltinr4Spec;
impl crate::RegisterSpec for Fltinr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr4::R`](R) reader structure"]
impl crate::Readable for Fltinr4Spec {}
#[doc = "`write(|w| ..)` method takes [`fltinr4::W`](W) writer structure"]
impl crate::Writable for Fltinr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR4 to value 0"]
impl crate::Resettable for Fltinr4Spec {
    const RESET_VALUE: u32 = 0;
}
