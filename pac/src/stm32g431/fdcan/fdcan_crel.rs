#[doc = "Register `FDCAN_CREL` reader"]
pub type R = crate::R<FdcanCrelSpec>;
#[doc = "Field `DAY` reader - 18"]
pub type DayR = crate::FieldReader;
#[doc = "Field `MON` reader - 12"]
pub type MonR = crate::FieldReader;
#[doc = "Field `YEAR` reader - 4"]
pub type YearR = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - 1"]
pub type SubstepR = crate::FieldReader;
#[doc = "Field `STEP` reader - 2"]
pub type StepR = crate::FieldReader;
#[doc = "Field `REL` reader - 3"]
pub type RelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 18"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 12"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 4"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 1"]
    #[inline(always)]
    pub fn substep(&self) -> SubstepR {
        SubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 2"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 3"]
    #[inline(always)]
    pub fn rel(&self) -> RelR {
        RelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CREL")
            .field("day", &self.day())
            .field("mon", &self.mon())
            .field("year", &self.year())
            .field("substep", &self.substep())
            .field("step", &self.step())
            .field("rel", &self.rel())
            .finish()
    }
}
#[doc = "FDCAN core release register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_crel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanCrelSpec;
impl crate::RegisterSpec for FdcanCrelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_crel::R`](R) reader structure"]
impl crate::Readable for FdcanCrelSpec {}
#[doc = "`reset()` method sets FDCAN_CREL to value 0x3214_1218"]
impl crate::Resettable for FdcanCrelSpec {
    const RESET_VALUE: u32 = 0x3214_1218;
}
