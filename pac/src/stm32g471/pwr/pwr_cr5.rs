#[doc = "Register `PWR_CR5` reader"]
pub type R = crate::R<PwrCr5Spec>;
#[doc = "Register `PWR_CR5` writer"]
pub type W = crate::W<PwrCr5Spec>;
#[doc = "Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R1mode {
    #[doc = "0: Main regulator in range 1 boost mode."]
    B0x0 = 0,
    #[doc = "1: Main regulator in range 1 normal mode."]
    B0x1 = 1,
}
impl From<R1mode> for bool {
    #[inline(always)]
    fn from(variant: R1mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R1MODE` reader - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
pub type R1modeR = crate::BitReader<R1mode>;
impl R1modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> R1mode {
        match self.bits {
            false => R1mode::B0x0,
            true => R1mode::B0x1,
        }
    }
    #[doc = "Main regulator in range 1 boost mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == R1mode::B0x0
    }
    #[doc = "Main regulator in range 1 normal mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == R1mode::B0x1
    }
}
#[doc = "Field `R1MODE` writer - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
pub type R1modeW<'a, REG> = crate::BitWriter<'a, REG, R1mode>;
impl<'a, REG> R1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main regulator in range 1 boost mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(R1mode::B0x0)
    }
    #[doc = "Main regulator in range 1 normal mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(R1mode::B0x1)
    }
}
impl R {
    #[doc = "Bit 8 - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
    #[inline(always)]
    pub fn r1mode(&self) -> R1modeR {
        R1modeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR5")
            .field("r1mode", &self.r1mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Main regular range 1 mode This bit is only valid for the main regulator in range 1 and has no effect on range 2. It is recommended to reset this bit when the system frequency is greater than 150 MHz. Refer to"]
    #[inline(always)]
    #[must_use]
    pub fn r1mode(&mut self) -> R1modeW<PwrCr5Spec> {
        R1modeW::new(self, 8)
    }
}
#[doc = "Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrCr5Spec;
impl crate::RegisterSpec for PwrCr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr5::R`](R) reader structure"]
impl crate::Readable for PwrCr5Spec {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr5::W`](W) writer structure"]
impl crate::Writable for PwrCr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR5 to value 0x0100"]
impl crate::Resettable for PwrCr5Spec {
    const RESET_VALUE: u32 = 0x0100;
}
