#[doc = "Register `APB1L_FZ` reader"]
pub type R = crate::R<Apb1lFzSpec>;
#[doc = "Register `APB1L_FZ` writer"]
pub type W = crate::W<Apb1lFzSpec>;
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DbgTimer2StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DbgTimer2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted"]
pub type DbgTim3StopR = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - TIM4 counter stopped when core is halted"]
pub type DbgTim4StopR = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - TIM4 counter stopped when core is halted"]
pub type DbgTim4StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - TIM5 counter stopped when core is halted"]
pub type DbgTim5StopR = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - TIM5 counter stopped when core is halted"]
pub type DbgTim5StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DbgTimer6StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DbgTimer6StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted"]
pub type DbgTim7StopR = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted"]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopR = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopR = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c1StopR = crate::BitReader;
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c2StopR = crate::BitReader;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c3StopR = crate::BitReader;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout mode stopped when core is halted"]
pub type DbgI2c3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIMER_STOP` reader - LPTIM1 counter stopped when core is halted"]
pub type DbgLptimerStopR = crate::BitReader;
#[doc = "Field `DBG_LPTIMER_STOP` writer - LPTIM1 counter stopped when core is halted"]
pub type DbgLptimerStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DbgTimer2StopR {
        DbgTimer2StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DbgTim4StopR {
        DbgTim4StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DbgTim5StopR {
        DbgTim5StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DbgTimer6StopR {
        DbgTimer6StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DbgI2c1StopR {
        DbgI2c1StopR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DbgI2c2StopR {
        DbgI2c2StopR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DbgI2c3StopR {
        DbgI2c3StopR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptimer_stop(&self) -> DbgLptimerStopR {
        DbgLptimerStopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1L_FZ")
            .field("dbg_timer2_stop", &self.dbg_timer2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_tim4_stop", &self.dbg_tim4_stop())
            .field("dbg_tim5_stop", &self.dbg_tim5_stop())
            .field("dbg_timer6_stop", &self.dbg_timer6_stop())
            .field("dbg_tim7_stop", &self.dbg_tim7_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
            .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
            .field("dbg_lptimer_stop", &self.dbg_lptimer_stop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DbgTimer2StopW<Apb1lFzSpec> {
        DbgTimer2StopW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<Apb1lFzSpec> {
        DbgTim3StopW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DbgTim4StopW<Apb1lFzSpec> {
        DbgTim4StopW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DbgTim5StopW<Apb1lFzSpec> {
        DbgTim5StopW::new(self, 3)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DbgTimer6StopW<Apb1lFzSpec> {
        DbgTimer6StopW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<Apb1lFzSpec> {
        DbgTim7StopW::new(self, 5)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<Apb1lFzSpec> {
        DbgRtcStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<Apb1lFzSpec> {
        DbgWwdgStopW::new(self, 11)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<Apb1lFzSpec> {
        DbgIwdgStopW::new(self, 12)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DbgI2c1StopW<Apb1lFzSpec> {
        DbgI2c1StopW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DbgI2c2StopW<Apb1lFzSpec> {
        DbgI2c2StopW::new(self, 22)
    }
    #[doc = "Bit 30 - I2C3 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DbgI2c3StopW<Apb1lFzSpec> {
        DbgI2c3StopW::new(self, 30)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptimer_stop(&mut self) -> DbgLptimerStopW<Apb1lFzSpec> {
        DbgLptimerStopW::new(self, 31)
    }
}
#[doc = "APB Low Freeze Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1l_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1l_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1lFzSpec;
impl crate::RegisterSpec for Apb1lFzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1l_fz::R`](R) reader structure"]
impl crate::Readable for Apb1lFzSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1l_fz::W`](W) writer structure"]
impl crate::Writable for Apb1lFzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1L_FZ to value 0"]
impl crate::Resettable for Apb1lFzSpec {
    const RESET_VALUE: u32 = 0;
}
