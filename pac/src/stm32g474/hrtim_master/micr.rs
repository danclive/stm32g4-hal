#[doc = "Register `MICR` writer"]
pub type W = crate::W<MicrSpec>;
#[doc = "Field `MCMP1C` writer - Master Compare 1 Interrupt flag clear"]
pub type Mcmp1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2C` writer - Master Compare 2 Interrupt flag clear"]
pub type Mcmp2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3C` writer - Master Compare 3 Interrupt flag clear"]
pub type Mcmp3cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4C` writer - Master Compare 4 Interrupt flag clear"]
pub type Mcmp4cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPC` writer - Repetition Interrupt flag clear"]
pub type MrepcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCC` writer - Sync Input Interrupt flag clear"]
pub type SynccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUPDC` writer - Master update Interrupt flag clear"]
pub type MupdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MicrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Master Compare 1 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1c(&mut self) -> Mcmp1cW<MicrSpec> {
        Mcmp1cW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2c(&mut self) -> Mcmp2cW<MicrSpec> {
        Mcmp2cW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3c(&mut self) -> Mcmp3cW<MicrSpec> {
        Mcmp3cW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4c(&mut self) -> Mcmp4cW<MicrSpec> {
        Mcmp4cW::new(self, 3)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mrepc(&mut self) -> MrepcW<MicrSpec> {
        MrepcW::new(self, 4)
    }
    #[doc = "Bit 5 - Sync Input Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn syncc(&mut self) -> SynccW<MicrSpec> {
        SynccW::new(self, 5)
    }
    #[doc = "Bit 6 - Master update Interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn mupdc(&mut self) -> MupdcW<MicrSpec> {
        MupdcW::new(self, 6)
    }
}
#[doc = "Master Timer Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MicrSpec;
impl crate::RegisterSpec for MicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`micr::W`](W) writer structure"]
impl crate::Writable for MicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MicrSpec {
    const RESET_VALUE: u32 = 0;
}
