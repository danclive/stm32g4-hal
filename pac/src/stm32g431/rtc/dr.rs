#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DU` reader - Date units in BCD format"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units in BCD format"]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MU` reader - Month units in BCD format"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MU` writer - Month units in BCD format"]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub type MtR = crate::BitReader;
#[doc = "Field `MT` writer - Month tens in BCD format"]
pub type MtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDU` reader - Week day units"]
pub type WduR = crate::FieldReader;
#[doc = "Field `WDU` writer - Week day units"]
pub type WduW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `YU` reader - Year units in BCD format"]
pub type YuR = crate::FieldReader;
#[doc = "Field `YU` writer - Year units in BCD format"]
pub type YuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YT` reader - Year tens in BCD format"]
pub type YtR = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens in BCD format"]
pub type YtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&self) -> WduR {
        WduR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn yu(&self) -> YuR {
        YuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn yt(&self) -> YtR {
        YtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("yt", &self.yt())
            .field("yu", &self.yu())
            .field("wdu", &self.wdu())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DuW<DrSpec> {
        DuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<DrSpec> {
        DtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MuW<DrSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MtW<DrSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    #[must_use]
    pub fn wdu(&mut self) -> WduW<DrSpec> {
        WduW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YuW<DrSpec> {
        YuW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YtW<DrSpec> {
        YtW::new(self, 20)
    }
}
#[doc = "date register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0x2101"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0x2101;
}
