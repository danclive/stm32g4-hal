#[doc = "Register `RCC_CR` reader"]
pub type R = crate::R<RccCrSpec>;
#[doc = "Register `RCC_CR` writer"]
pub type W = crate::W<RccCrSpec>;
#[doc = "HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsion {
    #[doc = "0: HSI16 oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator ON"]
    B0x1 = 1,
}
impl From<Hsion> for bool {
    #[inline(always)]
    fn from(variant: Hsion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
pub type HsionR = crate::BitReader<Hsion>;
impl HsionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsion {
        match self.bits {
            false => Hsion::B0x0,
            true => Hsion::B0x1,
        }
    }
    #[doc = "HSI16 oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsion::B0x0
    }
    #[doc = "HSI16 oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsion::B0x1
    }
}
#[doc = "Field `HSION` writer - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG, Hsion>;
impl<'a, REG> HsionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI16 oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsion::B0x0)
    }
    #[doc = "HSI16 oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsion::B0x1)
    }
}
#[doc = "HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsikeron {
    #[doc = "0: No effect on HSI16 oscillator."]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator is forced ON even in Stop mode."]
    B0x1 = 1,
}
impl From<Hsikeron> for bool {
    #[inline(always)]
    fn from(variant: Hsikeron) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
pub type HsikeronR = crate::BitReader<Hsikeron>;
impl HsikeronR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsikeron {
        match self.bits {
            false => Hsikeron::B0x0,
            true => Hsikeron::B0x1,
        }
    }
    #[doc = "No effect on HSI16 oscillator."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsikeron::B0x0
    }
    #[doc = "HSI16 oscillator is forced ON even in Stop mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsikeron::B0x1
    }
}
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
pub type HsikeronW<'a, REG> = crate::BitWriter<'a, REG, Hsikeron>;
impl<'a, REG> HsikeronW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect on HSI16 oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsikeron::B0x0)
    }
    #[doc = "HSI16 oscillator is forced ON even in Stop mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsikeron::B0x1)
    }
}
#[doc = "HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsirdy {
    #[doc = "0: HSI16 oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator ready"]
    B0x1 = 1,
}
impl From<Hsirdy> for bool {
    #[inline(always)]
    fn from(variant: Hsirdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
pub type HsirdyR = crate::BitReader<Hsirdy>;
impl HsirdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsirdy {
        match self.bits {
            false => Hsirdy::B0x0,
            true => Hsirdy::B0x1,
        }
    }
    #[doc = "HSI16 oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsirdy::B0x0
    }
    #[doc = "HSI16 oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsirdy::B0x1
    }
}
#[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hseon {
    #[doc = "0: HSE oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: HSE oscillator ON"]
    B0x1 = 1,
}
impl From<Hseon> for bool {
    #[inline(always)]
    fn from(variant: Hseon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEON` reader - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
pub type HseonR = crate::BitReader<Hseon>;
impl HseonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hseon {
        match self.bits {
            false => Hseon::B0x0,
            true => Hseon::B0x1,
        }
    }
    #[doc = "HSE oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hseon::B0x0
    }
    #[doc = "HSE oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hseon::B0x1
    }
}
#[doc = "Field `HSEON` writer - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG, Hseon>;
impl<'a, REG> HseonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hseon::B0x0)
    }
    #[doc = "HSE oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hseon::B0x1)
    }
}
#[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hserdy {
    #[doc = "0: HSE oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: HSE oscillator ready"]
    B0x1 = 1,
}
impl From<Hserdy> for bool {
    #[inline(always)]
    fn from(variant: Hserdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDY` reader - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
pub type HserdyR = crate::BitReader<Hserdy>;
impl HserdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hserdy {
        match self.bits {
            false => Hserdy::B0x0,
            true => Hserdy::B0x1,
        }
    }
    #[doc = "HSE oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hserdy::B0x0
    }
    #[doc = "HSE oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hserdy::B0x1
    }
}
#[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsebyp {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    B0x0 = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    B0x1 = 1,
}
impl From<Hsebyp> for bool {
    #[inline(always)]
    fn from(variant: Hsebyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HsebypR = crate::BitReader<Hsebyp>;
impl HsebypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsebyp {
        match self.bits {
            false => Hsebyp::B0x0,
            true => Hsebyp::B0x1,
        }
    }
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsebyp::B0x0
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsebyp::B0x1
    }
}
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG, Hsebyp>;
impl<'a, REG> HsebypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsebyp::B0x0)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsebyp::B0x1)
    }
}
#[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csson {
    #[doc = "0: Clock security system OFF (clock detector OFF)"]
    B0x0 = 0,
    #[doc = "1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    B0x1 = 1,
}
impl From<Csson> for bool {
    #[inline(always)]
    fn from(variant: Csson) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` reader - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
pub type CssonR = crate::BitReader<Csson>;
impl CssonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csson {
        match self.bits {
            false => Csson::B0x0,
            true => Csson::B0x1,
        }
    }
    #[doc = "Clock security system OFF (clock detector OFF)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Csson::B0x0
    }
    #[doc = "Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Csson::B0x1
    }
}
#[doc = "Field `CSSON` writer - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
pub type CssonW<'a, REG> = crate::BitWriter<'a, REG, Csson>;
impl<'a, REG> CssonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock security system OFF (clock detector OFF)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Csson::B0x0)
    }
    #[doc = "Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Csson::B0x1)
    }
}
#[doc = "Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllon {
    #[doc = "0: PLL OFF"]
    B0x0 = 0,
    #[doc = "1: PLL ON"]
    B0x1 = 1,
}
impl From<Pllon> for bool {
    #[inline(always)]
    fn from(variant: Pllon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLON` reader - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
pub type PllonR = crate::BitReader<Pllon>;
impl PllonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllon {
        match self.bits {
            false => Pllon::B0x0,
            true => Pllon::B0x1,
        }
    }
    #[doc = "PLL OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllon::B0x0
    }
    #[doc = "PLL ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllon::B0x1
    }
}
#[doc = "Field `PLLON` writer - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG, Pllon>;
impl<'a, REG> PllonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllon::B0x0)
    }
    #[doc = "PLL ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllon::B0x1)
    }
}
#[doc = "Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllrdy {
    #[doc = "0: PLL unlocked"]
    B0x0 = 0,
    #[doc = "1: PLL locked"]
    B0x1 = 1,
}
impl From<Pllrdy> for bool {
    #[inline(always)]
    fn from(variant: Pllrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked."]
pub type PllrdyR = crate::BitReader<Pllrdy>;
impl PllrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllrdy {
        match self.bits {
            false => Pllrdy::B0x0,
            true => Pllrdy::B0x1,
        }
    }
    #[doc = "PLL unlocked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pllrdy::B0x0
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pllrdy::B0x1
    }
}
impl R {
    #[doc = "Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    pub fn hsikeron(&self) -> HsikeronR {
        HsikeronR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after 6 HSI16 clock cycles."]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after 6 HSE clock cycles."]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    pub fn csson(&self) -> CssonR {
        CssonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag Set by hardware to indicate that the main PLL is locked."]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CR")
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("hsirdy", &self.hsirdy())
            .field("hseon", &self.hseon())
            .field("hserdy", &self.hserdy())
            .field("hsebyp", &self.hsebyp())
            .field("csson", &self.csson())
            .field("pllon", &self.pllon())
            .field("pllrdy", &self.pllrdy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - HSI16 clock enable Set and cleared by software. Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI16 oscillator ON when STOPWUCK=1 or HSIASFS = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI16 is used directly or indirectly as system clock."]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HsionW<RccCrSpec> {
        HsionW::new(self, 8)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels. Set and cleared by software to force HSI16 ON even in Stop modes. The HSI16 can only feed USARTs and I&lt;sup>2&lt;/sup>Cs peripherals configured with HSI16 as kernel clock. Keeping the HSI16 ON in Stop mode allows to avoid slowing down the communication speed because of the HSI16 startup time. This bit has no effect on HSION value."]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HsikeronW<RccCrSpec> {
        HsikeronW::new(self, 9)
    }
    #[doc = "Bit 16 - HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HseonW<RccCrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HsebypW<RccCrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CssonW<RccCrSpec> {
        CssonW::new(self, 19)
    }
    #[doc = "Bit 24 - Main PLL enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL clock is used as the system clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PllonW<RccCrSpec> {
        PllonW::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCrSpec;
impl crate::RegisterSpec for RccCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cr::R`](R) reader structure"]
impl crate::Readable for RccCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_cr::W`](W) writer structure"]
impl crate::Writable for RccCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CR to value 0x63"]
impl crate::Resettable for RccCrSpec {
    const RESET_VALUE: u32 = 0x63;
}
