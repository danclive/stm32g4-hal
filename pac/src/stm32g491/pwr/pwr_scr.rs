#[doc = "Register `PWR_SCR` writer"]
pub type W = crate::W<PwrScrSpec>;
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
pub type Cwuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
pub type Cwuf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
pub type Cwuf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
pub type Cwuf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
pub type Cwuf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` writer - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
pub type CsbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PwrScrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> Cwuf1W<PwrScrSpec> {
        Cwuf1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> Cwuf2W<PwrScrSpec> {
        Cwuf2W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> Cwuf3W<PwrScrSpec> {
        Cwuf3W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> Cwuf4W<PwrScrSpec> {
        Cwuf4W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> Cwuf5W<PwrScrSpec> {
        Cwuf5W::new(self, 4)
    }
    #[doc = "Bit 8 - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CsbfW<PwrScrSpec> {
        CsbfW::new(self, 8)
    }
}
#[doc = "Power status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrScrSpec;
impl crate::RegisterSpec for PwrScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pwr_scr::W`](W) writer structure"]
impl crate::Writable for PwrScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_SCR to value 0"]
impl crate::Resettable for PwrScrSpec {
    const RESET_VALUE: u32 = 0;
}
