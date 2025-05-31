#[doc = "Register `CALR` reader"]
pub type R = crate::R<CalrSpec>;
#[doc = "Register `CALR` writer"]
pub type W = crate::W<CalrSpec>;
#[doc = "Field `CALM` reader - Calibration minus"]
pub type CalmR = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus"]
pub type CalmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period"]
pub type Calw16R = crate::BitReader;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period"]
pub type Calw16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period"]
pub type Calw8R = crate::BitReader;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period"]
pub type Calw8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm"]
pub type CalpR = crate::BitReader;
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm"]
pub type CalpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Calibration minus"]
    #[inline(always)]
    pub fn calm(&self) -> CalmR {
        CalmR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub fn calw16(&self) -> Calw16R {
        Calw16R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub fn calw8(&self) -> Calw8R {
        Calw8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub fn calp(&self) -> CalpR {
        CalpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALR")
            .field("calp", &self.calp())
            .field("calw8", &self.calw8())
            .field("calw16", &self.calw16())
            .field("calm", &self.calm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus"]
    #[inline(always)]
    pub fn calm(&mut self) -> CalmW<CalrSpec> {
        CalmW::new(self, 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub fn calw16(&mut self) -> Calw16W<CalrSpec> {
        Calw16W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub fn calw8(&mut self) -> Calw8W<CalrSpec> {
        Calw8W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub fn calp(&mut self) -> CalpW<CalrSpec> {
        CalpW::new(self, 15)
    }
}
#[doc = "calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`calr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalrSpec;
impl crate::RegisterSpec for CalrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calr::R`](R) reader structure"]
impl crate::Readable for CalrSpec {}
#[doc = "`write(|w| ..)` method takes [`calr::W`](W) writer structure"]
impl crate::Writable for CalrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CalrSpec {}
