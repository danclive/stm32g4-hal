#[doc = "Register `APB2_FZ` reader"]
pub type R = crate::R<Apb2FzSpec>;
#[doc = "Register `APB2_FZ` writer"]
pub type W = crate::W<Apb2FzSpec>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DbgTim1StopR = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8_STOP` reader - TIM8 counter stopped when core is halted"]
pub type DbgTim8StopR = crate::BitReader;
#[doc = "Field `DBG_TIM8_STOP` writer - TIM8 counter stopped when core is halted"]
pub type DbgTim8StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted"]
pub type DbgTim15StopR = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted"]
pub type DbgTim15StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted"]
pub type DbgTim16StopR = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted"]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted"]
pub type DbgTim17StopR = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted"]
pub type DbgTim17StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM20_STOP` reader - TIM20counter stopped when core is halted"]
pub type DbgTim20StopR = crate::BitReader;
#[doc = "Field `DBG_TIM20_STOP` writer - TIM20counter stopped when core is halted"]
pub type DbgTim20StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM0_STOP` reader - DBG_HRTIM0_STOP"]
pub type DbgHrtim0StopR = crate::BitReader;
#[doc = "Field `DBG_HRTIM0_STOP` writer - DBG_HRTIM0_STOP"]
pub type DbgHrtim0StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM1_STOP` reader - DBG_HRTIM0_STOP"]
pub type DbgHrtim1StopR = crate::BitReader;
#[doc = "Field `DBG_HRTIM1_STOP` writer - DBG_HRTIM0_STOP"]
pub type DbgHrtim1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM2_STOP` reader - DBG_HRTIM0_STOP"]
pub type DbgHrtim2StopR = crate::BitReader;
#[doc = "Field `DBG_HRTIM2_STOP` writer - DBG_HRTIM0_STOP"]
pub type DbgHrtim2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM3_STOP` reader - DBG_HRTIM0_STOP"]
pub type DbgHrtim3StopR = crate::BitReader;
#[doc = "Field `DBG_HRTIM3_STOP` writer - DBG_HRTIM0_STOP"]
pub type DbgHrtim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DbgTim8StopR {
        DbgTim8StopR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DbgTim15StopR {
        DbgTim15StopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DbgTim17StopR {
        DbgTim17StopR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim20_stop(&self) -> DbgTim20StopR {
        DbgTim20StopR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&self) -> DbgHrtim0StopR {
        DbgHrtim0StopR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&self) -> DbgHrtim1StopR {
        DbgHrtim1StopR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&self) -> DbgHrtim2StopR {
        DbgHrtim2StopR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&self) -> DbgHrtim3StopR {
        DbgHrtim3StopR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_FZ")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim8_stop", &self.dbg_tim8_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .field("dbg_tim20_stop", &self.dbg_tim20_stop())
            .field("dbg_hrtim0_stop", &self.dbg_hrtim0_stop())
            .field("dbg_hrtim1_stop", &self.dbg_hrtim1_stop())
            .field("dbg_hrtim2_stop", &self.dbg_hrtim2_stop())
            .field("dbg_hrtim3_stop", &self.dbg_hrtim3_stop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<Apb2FzSpec> {
        DbgTim1StopW::new(self, 11)
    }
    #[doc = "Bit 13 - TIM8 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DbgTim8StopW<Apb2FzSpec> {
        DbgTim8StopW::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DbgTim15StopW<Apb2FzSpec> {
        DbgTim15StopW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<Apb2FzSpec> {
        DbgTim16StopW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DbgTim17StopW<Apb2FzSpec> {
        DbgTim17StopW::new(self, 18)
    }
    #[doc = "Bit 20 - TIM20counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim20_stop(&mut self) -> DbgTim20StopW<Apb2FzSpec> {
        DbgTim20StopW::new(self, 20)
    }
    #[doc = "Bit 26 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&mut self) -> DbgHrtim0StopW<Apb2FzSpec> {
        DbgHrtim0StopW::new(self, 26)
    }
    #[doc = "Bit 27 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&mut self) -> DbgHrtim1StopW<Apb2FzSpec> {
        DbgHrtim1StopW::new(self, 27)
    }
    #[doc = "Bit 28 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&mut self) -> DbgHrtim2StopW<Apb2FzSpec> {
        DbgHrtim2StopW::new(self, 28)
    }
    #[doc = "Bit 29 - DBG_HRTIM0_STOP"]
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&mut self) -> DbgHrtim3StopW<Apb2FzSpec> {
        DbgHrtim3StopW::new(self, 29)
    }
}
#[doc = "APB High Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2FzSpec;
impl crate::RegisterSpec for Apb2FzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2_fz::R`](R) reader structure"]
impl crate::Readable for Apb2FzSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2_fz::W`](W) writer structure"]
impl crate::Writable for Apb2FzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2_FZ to value 0"]
impl crate::Resettable for Apb2FzSpec {
    const RESET_VALUE: u32 = 0;
}
