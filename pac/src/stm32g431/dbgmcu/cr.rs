#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBG_SLEEP` reader - Debug Sleep Mode"]
pub type DbgSleepR = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - Debug Sleep Mode"]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - Debug Stop Mode"]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - Debug Stop Mode"]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - Debug Standby Mode"]
pub type DbgStandbyR = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - Debug Standby Mode"]
pub type DbgStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - Trace pin assignment control"]
pub type TraceIoenR = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - Trace pin assignment control"]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - Trace pin assignment control"]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - Trace pin assignment control"]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Debug Sleep Mode"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DbgSleepR {
        DbgSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Stop Mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby Mode"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DbgStandbyR {
        DbgStandbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("trace_ioen", &self.trace_ioen())
            .field("trace_mode", &self.trace_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Debug Sleep Mode"]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<CrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Stop Mode"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DbgStopW<CrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby Mode"]
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DbgStandbyW<CrSpec> {
        DbgStandbyW::new(self, 2)
    }
    #[doc = "Bit 5 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TraceIoenW<CrSpec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TraceModeW<CrSpec> {
        TraceModeW::new(self, 6)
    }
}
#[doc = "Debug MCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
