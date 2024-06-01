#[doc = "Register `FLTAR` reader"]
pub type R = crate::R<FltarSpec>;
#[doc = "Register `FLTAR` writer"]
pub type W = crate::W<FltarSpec>;
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type Flt1enR = crate::BitReader;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type Flt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type Flt2enR = crate::BitReader;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type Flt2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type Flt3enR = crate::BitReader;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type Flt3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type Flt4enR = crate::BitReader;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type Flt4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type Flt5enR = crate::BitReader;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub type Flt5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT6EN` reader - Fault 6 enable"]
pub type Flt6enR = crate::BitReader;
#[doc = "Field `FLT6EN` writer - Fault 6 enable"]
pub type Flt6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FltlckR = crate::BitReader;
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FltlckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> Flt1enR {
        Flt1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> Flt2enR {
        Flt2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> Flt3enR {
        Flt3enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> Flt4enR {
        Flt4enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> Flt5enR {
        Flt5enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    pub fn flt6en(&self) -> Flt6enR {
        Flt6enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FltlckR {
        FltlckR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTAR")
            .field("fltlck", &self.fltlck())
            .field("flt6en", &self.flt6en())
            .field("flt5en", &self.flt5en())
            .field("flt4en", &self.flt4en())
            .field("flt3en", &self.flt3en())
            .field("flt2en", &self.flt2en())
            .field("flt1en", &self.flt1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> Flt1enW<FltarSpec> {
        Flt1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> Flt2enW<FltarSpec> {
        Flt2enW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> Flt3enW<FltarSpec> {
        Flt3enW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> Flt4enW<FltarSpec> {
        Flt4enW::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> Flt5enW<FltarSpec> {
        Flt5enW::new(self, 4)
    }
    #[doc = "Bit 5 - Fault 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt6en(&mut self) -> Flt6enW<FltarSpec> {
        Flt6enW::new(self, 5)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FltlckW<FltarSpec> {
        FltlckW::new(self, 31)
    }
}
#[doc = "Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltarSpec;
impl crate::RegisterSpec for FltarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltar::R`](R) reader structure"]
impl crate::Readable for FltarSpec {}
#[doc = "`write(|w| ..)` method takes [`fltar::W`](W) writer structure"]
impl crate::Writable for FltarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTAR to value 0"]
impl crate::Resettable for FltarSpec {
    const RESET_VALUE: u32 = 0;
}
