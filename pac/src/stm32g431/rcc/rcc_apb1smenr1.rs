#[doc = "Register `RCC_APB1SMENR1` reader"]
pub type R = crate::R<RccApb1smenr1Spec>;
#[doc = "Register `RCC_APB1SMENR1` writer"]
pub type W = crate::W<RccApb1smenr1Spec>;
#[doc = "TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2smen {
    #[doc = "0: TIM2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim2smen> for bool {
    #[inline(always)]
    fn from(variant: Tim2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim2smenR = crate::BitReader<Tim2smen>;
impl Tim2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2smen {
        match self.bits {
            false => Tim2smen::B0x0,
            true => Tim2smen::B0x1,
        }
    }
    #[doc = "TIM2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim2smen::B0x0
    }
    #[doc = "TIM2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim2smen::B0x1
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim2smenW<'a, REG> = crate::BitWriter<'a, REG, Tim2smen>;
impl<'a, REG> Tim2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2smen::B0x0)
    }
    #[doc = "TIM2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2smen::B0x1)
    }
}
#[doc = "TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3smen {
    #[doc = "0: TIM3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim3smen> for bool {
    #[inline(always)]
    fn from(variant: Tim3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim3smenR = crate::BitReader<Tim3smen>;
impl Tim3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3smen {
        match self.bits {
            false => Tim3smen::B0x0,
            true => Tim3smen::B0x1,
        }
    }
    #[doc = "TIM3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim3smen::B0x0
    }
    #[doc = "TIM3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim3smen::B0x1
    }
}
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim3smenW<'a, REG> = crate::BitWriter<'a, REG, Tim3smen>;
impl<'a, REG> Tim3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3smen::B0x0)
    }
    #[doc = "TIM3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3smen::B0x1)
    }
}
#[doc = "TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim4smen {
    #[doc = "0: TIM4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim4smen> for bool {
    #[inline(always)]
    fn from(variant: Tim4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim4smenR = crate::BitReader<Tim4smen>;
impl Tim4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim4smen {
        match self.bits {
            false => Tim4smen::B0x0,
            true => Tim4smen::B0x1,
        }
    }
    #[doc = "TIM4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim4smen::B0x0
    }
    #[doc = "TIM4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim4smen::B0x1
    }
}
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim4smenW<'a, REG> = crate::BitWriter<'a, REG, Tim4smen>;
impl<'a, REG> Tim4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim4smen::B0x0)
    }
    #[doc = "TIM4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim4smen::B0x1)
    }
}
#[doc = "TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim5smen {
    #[doc = "0: TIM5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim5smen> for bool {
    #[inline(always)]
    fn from(variant: Tim5smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim5smenR = crate::BitReader<Tim5smen>;
impl Tim5smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim5smen {
        match self.bits {
            false => Tim5smen::B0x0,
            true => Tim5smen::B0x1,
        }
    }
    #[doc = "TIM5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim5smen::B0x0
    }
    #[doc = "TIM5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim5smen::B0x1
    }
}
#[doc = "Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim5smenW<'a, REG> = crate::BitWriter<'a, REG, Tim5smen>;
impl<'a, REG> Tim5smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim5smen::B0x0)
    }
    #[doc = "TIM5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim5smen::B0x1)
    }
}
#[doc = "TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim6smen {
    #[doc = "0: TIM6 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM6 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim6smen> for bool {
    #[inline(always)]
    fn from(variant: Tim6smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim6smenR = crate::BitReader<Tim6smen>;
impl Tim6smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim6smen {
        match self.bits {
            false => Tim6smen::B0x0,
            true => Tim6smen::B0x1,
        }
    }
    #[doc = "TIM6 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim6smen::B0x0
    }
    #[doc = "TIM6 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim6smen::B0x1
    }
}
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim6smenW<'a, REG> = crate::BitWriter<'a, REG, Tim6smen>;
impl<'a, REG> Tim6smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM6 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6smen::B0x0)
    }
    #[doc = "TIM6 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6smen::B0x1)
    }
}
#[doc = "TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim7smen {
    #[doc = "0: TIM7 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM7 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim7smen> for bool {
    #[inline(always)]
    fn from(variant: Tim7smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim7smenR = crate::BitReader<Tim7smen>;
impl Tim7smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim7smen {
        match self.bits {
            false => Tim7smen::B0x0,
            true => Tim7smen::B0x1,
        }
    }
    #[doc = "TIM7 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim7smen::B0x0
    }
    #[doc = "TIM7 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim7smen::B0x1
    }
}
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim7smenW<'a, REG> = crate::BitWriter<'a, REG, Tim7smen>;
impl<'a, REG> Tim7smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7smen::B0x0)
    }
    #[doc = "TIM7 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7smen::B0x1)
    }
}
#[doc = "CRS timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crssmen {
    #[doc = "0: CRS clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: CRS clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Crssmen> for bool {
    #[inline(always)]
    fn from(variant: Crssmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSSMEN` reader - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CrssmenR = crate::BitReader<Crssmen>;
impl CrssmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crssmen {
        match self.bits {
            false => Crssmen::B0x0,
            true => Crssmen::B0x1,
        }
    }
    #[doc = "CRS clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crssmen::B0x0
    }
    #[doc = "CRS clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crssmen::B0x1
    }
}
#[doc = "Field `CRSSMEN` writer - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CrssmenW<'a, REG> = crate::BitWriter<'a, REG, Crssmen>;
impl<'a, REG> CrssmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRS clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crssmen::B0x0)
    }
    #[doc = "CRS clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crssmen::B0x1)
    }
}
#[doc = "RTC APB clock enable during Sleep and Stop modes Set and cleared by software\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcapbsmen {
    #[doc = "0: RTC APB clock disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: RTC APB clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Rtcapbsmen> for bool {
    #[inline(always)]
    fn from(variant: Rtcapbsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
pub type RtcapbsmenR = crate::BitReader<Rtcapbsmen>;
impl RtcapbsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcapbsmen {
        match self.bits {
            false => Rtcapbsmen::B0x0,
            true => Rtcapbsmen::B0x1,
        }
    }
    #[doc = "RTC APB clock disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcapbsmen::B0x0
    }
    #[doc = "RTC APB clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcapbsmen::B0x1
    }
}
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
pub type RtcapbsmenW<'a, REG> = crate::BitWriter<'a, REG, Rtcapbsmen>;
impl<'a, REG> RtcapbsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC APB clock disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapbsmen::B0x0)
    }
    #[doc = "RTC APB clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapbsmen::B0x1)
    }
}
#[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdgsmen {
    #[doc = "0: Window watchdog clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: Window watchdog clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Wwdgsmen> for bool {
    #[inline(always)]
    fn from(variant: Wwdgsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
pub type WwdgsmenR = crate::BitReader<Wwdgsmen>;
impl WwdgsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdgsmen {
        match self.bits {
            false => Wwdgsmen::B0x0,
            true => Wwdgsmen::B0x1,
        }
    }
    #[doc = "Window watchdog clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wwdgsmen::B0x0
    }
    #[doc = "Window watchdog clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wwdgsmen::B0x1
    }
}
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
pub type WwdgsmenW<'a, REG> = crate::BitWriter<'a, REG, Wwdgsmen>;
impl<'a, REG> WwdgsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window watchdog clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgsmen::B0x0)
    }
    #[doc = "Window watchdog clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgsmen::B0x1)
    }
}
#[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi2smen {
    #[doc = "0: SPI2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SPI2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Spi2smen> for bool {
    #[inline(always)]
    fn from(variant: Spi2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi2smenR = crate::BitReader<Spi2smen>;
impl Spi2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi2smen {
        match self.bits {
            false => Spi2smen::B0x0,
            true => Spi2smen::B0x1,
        }
    }
    #[doc = "SPI2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi2smen::B0x0
    }
    #[doc = "SPI2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi2smen::B0x1
    }
}
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi2smenW<'a, REG> = crate::BitWriter<'a, REG, Spi2smen>;
impl<'a, REG> Spi2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2smen::B0x0)
    }
    #[doc = "SPI2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2smen::B0x1)
    }
}
#[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi3smen {
    #[doc = "0: SPI3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SPI3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Spi3smen> for bool {
    #[inline(always)]
    fn from(variant: Spi3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi3smenR = crate::BitReader<Spi3smen>;
impl Spi3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi3smen {
        match self.bits {
            false => Spi3smen::B0x0,
            true => Spi3smen::B0x1,
        }
    }
    #[doc = "SPI3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi3smen::B0x0
    }
    #[doc = "SPI3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi3smen::B0x1
    }
}
#[doc = "Field `SPI3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi3smenW<'a, REG> = crate::BitWriter<'a, REG, Spi3smen>;
impl<'a, REG> Spi3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3smen::B0x0)
    }
    #[doc = "SPI3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3smen::B0x1)
    }
}
#[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart2smen {
    #[doc = "0: USART2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: USART2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Usart2smen> for bool {
    #[inline(always)]
    fn from(variant: Usart2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart2smenR = crate::BitReader<Usart2smen>;
impl Usart2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2smen {
        match self.bits {
            false => Usart2smen::B0x0,
            true => Usart2smen::B0x1,
        }
    }
    #[doc = "USART2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2smen::B0x0
    }
    #[doc = "USART2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2smen::B0x1
    }
}
#[doc = "Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart2smenW<'a, REG> = crate::BitWriter<'a, REG, Usart2smen>;
impl<'a, REG> Usart2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2smen::B0x0)
    }
    #[doc = "USART2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2smen::B0x1)
    }
}
#[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart3smen {
    #[doc = "0: USART3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: USART3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Usart3smen> for bool {
    #[inline(always)]
    fn from(variant: Usart3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart3smenR = crate::BitReader<Usart3smen>;
impl Usart3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3smen {
        match self.bits {
            false => Usart3smen::B0x0,
            true => Usart3smen::B0x1,
        }
    }
    #[doc = "USART3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3smen::B0x0
    }
    #[doc = "USART3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3smen::B0x1
    }
}
#[doc = "Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart3smenW<'a, REG> = crate::BitWriter<'a, REG, Usart3smen>;
impl<'a, REG> Usart3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3smen::B0x0)
    }
    #[doc = "USART3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3smen::B0x1)
    }
}
#[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart4smen {
    #[doc = "0: UART4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: UART4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Uart4smen> for bool {
    #[inline(always)]
    fn from(variant: Uart4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Uart4smenR = crate::BitReader<Uart4smen>;
impl Uart4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart4smen {
        match self.bits {
            false => Uart4smen::B0x0,
            true => Uart4smen::B0x1,
        }
    }
    #[doc = "UART4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart4smen::B0x0
    }
    #[doc = "UART4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart4smen::B0x1
    }
}
#[doc = "Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Uart4smenW<'a, REG> = crate::BitWriter<'a, REG, Uart4smen>;
impl<'a, REG> Uart4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART4 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4smen::B0x0)
    }
    #[doc = "UART4 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4smen::B0x1)
    }
}
#[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart5smen {
    #[doc = "0: UART5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: UART5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Uart5smen> for bool {
    #[inline(always)]
    fn from(variant: Uart5smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Uart5smenR = crate::BitReader<Uart5smen>;
impl Uart5smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart5smen {
        match self.bits {
            false => Uart5smen::B0x0,
            true => Uart5smen::B0x1,
        }
    }
    #[doc = "UART5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart5smen::B0x0
    }
    #[doc = "UART5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart5smen::B0x1
    }
}
#[doc = "Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Uart5smenW<'a, REG> = crate::BitWriter<'a, REG, Uart5smen>;
impl<'a, REG> Uart5smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART5 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5smen::B0x0)
    }
    #[doc = "UART5 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5smen::B0x1)
    }
}
#[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1smen {
    #[doc = "0: I2C1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: I2C1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<I2c1smen> for bool {
    #[inline(always)]
    fn from(variant: I2c1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c1smenR = crate::BitReader<I2c1smen>;
impl I2c1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1smen {
        match self.bits {
            false => I2c1smen::B0x0,
            true => I2c1smen::B0x1,
        }
    }
    #[doc = "I2C1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1smen::B0x0
    }
    #[doc = "I2C1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1smen::B0x1
    }
}
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c1smenW<'a, REG> = crate::BitWriter<'a, REG, I2c1smen>;
impl<'a, REG> I2c1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C1 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1smen::B0x0)
    }
    #[doc = "I2C1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1smen::B0x1)
    }
}
#[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2smen {
    #[doc = "0: I2C2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: I2C2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<I2c2smen> for bool {
    #[inline(always)]
    fn from(variant: I2c2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c2smenR = crate::BitReader<I2c2smen>;
impl I2c2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2smen {
        match self.bits {
            false => I2c2smen::B0x0,
            true => I2c2smen::B0x1,
        }
    }
    #[doc = "I2C2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2smen::B0x0
    }
    #[doc = "I2C2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2smen::B0x1
    }
}
#[doc = "Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c2smenW<'a, REG> = crate::BitWriter<'a, REG, I2c2smen>;
impl<'a, REG> I2c2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C2 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2smen::B0x0)
    }
    #[doc = "I2C2 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2smen::B0x1)
    }
}
#[doc = "USB device clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbsmen {
    #[doc = "0: USB device clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: USB device clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Usbsmen> for bool {
    #[inline(always)]
    fn from(variant: Usbsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSMEN` reader - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UsbsmenR = crate::BitReader<Usbsmen>;
impl UsbsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbsmen {
        match self.bits {
            false => Usbsmen::B0x0,
            true => Usbsmen::B0x1,
        }
    }
    #[doc = "USB device clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usbsmen::B0x0
    }
    #[doc = "USB device clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usbsmen::B0x1
    }
}
#[doc = "Field `USBSMEN` writer - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type UsbsmenW<'a, REG> = crate::BitWriter<'a, REG, Usbsmen>;
impl<'a, REG> UsbsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB device clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsmen::B0x0)
    }
    #[doc = "USB device clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsmen::B0x1)
    }
}
#[doc = "FDCAN clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdcansmen {
    #[doc = "0: FDCAN clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: FDCAN clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Fdcansmen> for bool {
    #[inline(always)]
    fn from(variant: Fdcansmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCANSMEN` reader - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FdcansmenR = crate::BitReader<Fdcansmen>;
impl FdcansmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdcansmen {
        match self.bits {
            false => Fdcansmen::B0x0,
            true => Fdcansmen::B0x1,
        }
    }
    #[doc = "FDCAN clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fdcansmen::B0x0
    }
    #[doc = "FDCAN clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fdcansmen::B0x1
    }
}
#[doc = "Field `FDCANSMEN` writer - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FdcansmenW<'a, REG> = crate::BitWriter<'a, REG, Fdcansmen>;
impl<'a, REG> FdcansmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FDCAN clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcansmen::B0x0)
    }
    #[doc = "FDCAN clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcansmen::B0x1)
    }
}
#[doc = "Power interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrsmen {
    #[doc = "0: Power interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: Power interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Pwrsmen> for bool {
    #[inline(always)]
    fn from(variant: Pwrsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type PwrsmenR = crate::BitReader<Pwrsmen>;
impl PwrsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrsmen {
        match self.bits {
            false => Pwrsmen::B0x0,
            true => Pwrsmen::B0x1,
        }
    }
    #[doc = "Power interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwrsmen::B0x0
    }
    #[doc = "Power interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwrsmen::B0x1
    }
}
#[doc = "Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type PwrsmenW<'a, REG> = crate::BitWriter<'a, REG, Pwrsmen>;
impl<'a, REG> PwrsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsmen::B0x0)
    }
    #[doc = "Power interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsmen::B0x1)
    }
}
#[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3smen {
    #[doc = "0: I2C3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: I2C3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<I2c3smen> for bool {
    #[inline(always)]
    fn from(variant: I2c3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c3smenR = crate::BitReader<I2c3smen>;
impl I2c3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3smen {
        match self.bits {
            false => I2c3smen::B0x0,
            true => I2c3smen::B0x1,
        }
    }
    #[doc = "I2C3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3smen::B0x0
    }
    #[doc = "I2C3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3smen::B0x1
    }
}
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type I2c3smenW<'a, REG> = crate::BitWriter<'a, REG, I2c3smen>;
impl<'a, REG> I2c3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C3 clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3smen::B0x0)
    }
    #[doc = "I2C3 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3smen::B0x1)
    }
}
#[doc = "Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim1smen {
    #[doc = "0: LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: LPTIM1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Lptim1smen> for bool {
    #[inline(always)]
    fn from(variant: Lptim1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Lptim1smenR = crate::BitReader<Lptim1smen>;
impl Lptim1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1smen {
        match self.bits {
            false => Lptim1smen::B0x0,
            true => Lptim1smen::B0x1,
        }
    }
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1smen::B0x0
    }
    #[doc = "LPTIM1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1smen::B0x1
    }
}
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Lptim1smenW<'a, REG> = crate::BitWriter<'a, REG, Lptim1smen>;
impl<'a, REG> Lptim1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1smen::B0x0)
    }
    #[doc = "LPTIM1 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1smen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim2smen(&self) -> Tim2smenR {
        Tim2smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim3smen(&self) -> Tim3smenR {
        Tim3smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim4smen(&self) -> Tim4smenR {
        Tim4smenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim5smen(&self) -> Tim5smenR {
        Tim5smenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim6smen(&self) -> Tim6smenR {
        Tim6smenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim7smen(&self) -> Tim7smenR {
        Tim7smenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn crssmen(&self) -> CrssmenR {
        CrssmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RtcapbsmenR {
        RtcapbsmenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WwdgsmenR {
        WwdgsmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi2smen(&self) -> Spi2smenR {
        Spi2smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi3smen(&self) -> Spi3smenR {
        Spi3smenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart2smen(&self) -> Usart2smenR {
        Usart2smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart3smen(&self) -> Usart3smenR {
        Usart3smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn uart4smen(&self) -> Uart4smenR {
        Uart4smenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn uart5smen(&self) -> Uart5smenR {
        Uart5smenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2c1smenR {
        I2c1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2c2smenR {
        I2c2smenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usbsmen(&self) -> UsbsmenR {
        UsbsmenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn fdcansmen(&self) -> FdcansmenR {
        FdcansmenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PwrsmenR {
        PwrsmenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2c3smenR {
        I2c3smenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1smen(&self) -> Lptim1smenR {
        Lptim1smenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("tim4smen", &self.tim4smen())
            .field("tim5smen", &self.tim5smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("crssmen", &self.crssmen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("spi2smen", &self.spi2smen())
            .field("spi3smen", &self.spi3smen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("uart4smen", &self.uart4smen())
            .field("uart5smen", &self.uart5smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("usbsmen", &self.usbsmen())
            .field("fdcansmen", &self.fdcansmen())
            .field("pwrsmen", &self.pwrsmen())
            .field("i2c3smen", &self.i2c3smen())
            .field("lptim1smen", &self.lptim1smen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim2smen(&mut self) -> Tim2smenW<RccApb1smenr1Spec> {
        Tim2smenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim3smen(&mut self) -> Tim3smenW<RccApb1smenr1Spec> {
        Tim3smenW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim4smen(&mut self) -> Tim4smenW<RccApb1smenr1Spec> {
        Tim4smenW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim5smen(&mut self) -> Tim5smenW<RccApb1smenr1Spec> {
        Tim5smenW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim6smen(&mut self) -> Tim6smenW<RccApb1smenr1Spec> {
        Tim6smenW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim7smen(&mut self) -> Tim7smenW<RccApb1smenr1Spec> {
        Tim7smenW::new(self, 5)
    }
    #[doc = "Bit 8 - CRS timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn crssmen(&mut self) -> CrssmenW<RccApb1smenr1Spec> {
        CrssmenW::new(self, 8)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RtcapbsmenW<RccApb1smenr1Spec> {
        RtcapbsmenW::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WwdgsmenW<RccApb1smenr1Spec> {
        WwdgsmenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi2smen(&mut self) -> Spi2smenW<RccApb1smenr1Spec> {
        Spi2smenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi3smen(&mut self) -> Spi3smenW<RccApb1smenr1Spec> {
        Spi3smenW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart2smen(&mut self) -> Usart2smenW<RccApb1smenr1Spec> {
        Usart2smenW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart3smen(&mut self) -> Usart3smenW<RccApb1smenr1Spec> {
        Usart3smenW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn uart4smen(&mut self) -> Uart4smenW<RccApb1smenr1Spec> {
        Uart4smenW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn uart5smen(&mut self) -> Uart5smenW<RccApb1smenr1Spec> {
        Uart5smenW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2c1smenW<RccApb1smenr1Spec> {
        I2c1smenW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2c2smenW<RccApb1smenr1Spec> {
        I2c2smenW::new(self, 22)
    }
    #[doc = "Bit 23 - USB device clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usbsmen(&mut self) -> UsbsmenW<RccApb1smenr1Spec> {
        UsbsmenW::new(self, 23)
    }
    #[doc = "Bit 25 - FDCAN clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn fdcansmen(&mut self) -> FdcansmenW<RccApb1smenr1Spec> {
        FdcansmenW::new(self, 25)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PwrsmenW<RccApb1smenr1Spec> {
        PwrsmenW::new(self, 28)
    }
    #[doc = "Bit 30 - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2c3smenW<RccApb1smenr1Spec> {
        I2c3smenW::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> Lptim1smenW<RccApb1smenr1Spec> {
        Lptim1smenW::new(self, 31)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb1smenr1Spec;
impl crate::RegisterSpec for RccApb1smenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1smenr1::R`](R) reader structure"]
impl crate::Readable for RccApb1smenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1smenr1::W`](W) writer structure"]
impl crate::Writable for RccApb1smenr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB1SMENR1 to value 0xd2fe_cd3f"]
impl crate::Resettable for RccApb1smenr1Spec {
    const RESET_VALUE: u32 = 0xd2fe_cd3f;
}
