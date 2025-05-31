#[doc = "Register `FDCAN_RWD` reader"]
pub type R = crate::R<FdcanRwdSpec>;
#[doc = "Register `FDCAN_RWD` writer"]
pub type W = crate::W<FdcanRwdSpec>;
#[doc = "Field `WDC` reader - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
pub type WdcR = crate::FieldReader;
#[doc = "Field `WDC` writer - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
pub type WdcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDV` reader - Watchdog value Actual message RAM watchdog counter value."]
pub type WdvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub fn wdc(&self) -> WdcR {
        WdcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog value Actual message RAM watchdog counter value."]
    #[inline(always)]
    pub fn wdv(&self) -> WdvR {
        WdvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RWD")
            .field("wdc", &self.wdc())
            .field("wdv", &self.wdv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub fn wdc(&mut self) -> WdcW<FdcanRwdSpec> {
        WdcW::new(self, 0)
    }
}
#[doc = "FDCAN RAM watchdog register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRwdSpec;
impl crate::RegisterSpec for FdcanRwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rwd::R`](R) reader structure"]
impl crate::Readable for FdcanRwdSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rwd::W`](W) writer structure"]
impl crate::Writable for FdcanRwdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RWD to value 0"]
impl crate::Resettable for FdcanRwdSpec {}
