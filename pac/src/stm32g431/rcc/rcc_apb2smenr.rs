#[doc = "Register `RCC_APB2SMENR` reader"]
pub type R = crate::R<RccApb2smenrSpec>;
#[doc = "Register `RCC_APB2SMENR` writer"]
pub type W = crate::W<RccApb2smenrSpec>;
#[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscfgsmen {
    #[doc = "0: SYSCFG + COMP + VREFBUF + OPAMP clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF + OPAMP clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Syscfgsmen> for bool {
    #[inline(always)]
    fn from(variant: Syscfgsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SyscfgsmenR = crate::BitReader<Syscfgsmen>;
impl SyscfgsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscfgsmen {
        match self.bits {
            false => Syscfgsmen::B0x0,
            true => Syscfgsmen::B0x1,
        }
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syscfgsmen::B0x0
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syscfgsmen::B0x1
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type SyscfgsmenW<'a, REG> = crate::BitWriter<'a, REG, Syscfgsmen>;
impl<'a, REG> SyscfgsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgsmen::B0x0)
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgsmen::B0x1)
    }
}
#[doc = "TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1smen {
    #[doc = "0: TIM1 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM1P timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim1smen> for bool {
    #[inline(always)]
    fn from(variant: Tim1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim1smenR = crate::BitReader<Tim1smen>;
impl Tim1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1smen {
        match self.bits {
            false => Tim1smen::B0x0,
            true => Tim1smen::B0x1,
        }
    }
    #[doc = "TIM1 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1smen::B0x0
    }
    #[doc = "TIM1P timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1smen::B0x1
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim1smenW<'a, REG> = crate::BitWriter<'a, REG, Tim1smen>;
impl<'a, REG> Tim1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM1 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1smen::B0x0)
    }
    #[doc = "TIM1P timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1smen::B0x1)
    }
}
#[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1smen {
    #[doc = "0: SPI1 clocks disabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SPI1 clocks enabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Spi1smen> for bool {
    #[inline(always)]
    fn from(variant: Spi1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1SMEN` reader - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi1smenR = crate::BitReader<Spi1smen>;
impl Spi1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1smen {
        match self.bits {
            false => Spi1smen::B0x0,
            true => Spi1smen::B0x1,
        }
    }
    #[doc = "SPI1 clocks disabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi1smen::B0x0
    }
    #[doc = "SPI1 clocks enabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi1smen::B0x1
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi1smenW<'a, REG> = crate::BitWriter<'a, REG, Spi1smen>;
impl<'a, REG> Spi1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI1 clocks disabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1smen::B0x0)
    }
    #[doc = "SPI1 clocks enabled by the clock gating during&lt;sup>(1)&lt;/sup> Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1smen::B0x1)
    }
}
#[doc = "TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim8smen {
    #[doc = "0: TIM8 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM8 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim8smen> for bool {
    #[inline(always)]
    fn from(variant: Tim8smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM8SMEN` reader - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim8smenR = crate::BitReader<Tim8smen>;
impl Tim8smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim8smen {
        match self.bits {
            false => Tim8smen::B0x0,
            true => Tim8smen::B0x1,
        }
    }
    #[doc = "TIM8 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim8smen::B0x0
    }
    #[doc = "TIM8 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim8smen::B0x1
    }
}
#[doc = "Field `TIM8SMEN` writer - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim8smenW<'a, REG> = crate::BitWriter<'a, REG, Tim8smen>;
impl<'a, REG> Tim8smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM8 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim8smen::B0x0)
    }
    #[doc = "TIM8 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim8smen::B0x1)
    }
}
#[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1smen {
    #[doc = "0: USART1clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: USART1clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Usart1smen> for bool {
    #[inline(always)]
    fn from(variant: Usart1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1SMEN` reader - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart1smenR = crate::BitReader<Usart1smen>;
impl Usart1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1smen {
        match self.bits {
            false => Usart1smen::B0x0,
            true => Usart1smen::B0x1,
        }
    }
    #[doc = "USART1clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1smen::B0x0
    }
    #[doc = "USART1clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1smen::B0x1
    }
}
#[doc = "Field `USART1SMEN` writer - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Usart1smenW<'a, REG> = crate::BitWriter<'a, REG, Usart1smen>;
impl<'a, REG> Usart1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART1clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1smen::B0x0)
    }
    #[doc = "USART1clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1smen::B0x1)
    }
}
#[doc = "SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi4smen {
    #[doc = "0: SPI4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SPI4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    B0x1 = 1,
}
impl From<Spi4smen> for bool {
    #[inline(always)]
    fn from(variant: Spi4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI4SMEN` reader - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi4smenR = crate::BitReader<Spi4smen>;
impl Spi4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi4smen {
        match self.bits {
            false => Spi4smen::B0x0,
            true => Spi4smen::B0x1,
        }
    }
    #[doc = "SPI4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi4smen::B0x0
    }
    #[doc = "SPI4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi4smen::B0x1
    }
}
#[doc = "Field `SPI4SMEN` writer - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Spi4smenW<'a, REG> = crate::BitWriter<'a, REG, Spi4smen>;
impl<'a, REG> Spi4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI4 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi4smen::B0x0)
    }
    #[doc = "SPI4 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi4smen::B0x1)
    }
}
#[doc = "TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15smen {
    #[doc = "0: TIM15 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM15 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    B0x1 = 1,
}
impl From<Tim15smen> for bool {
    #[inline(always)]
    fn from(variant: Tim15smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim15smenR = crate::BitReader<Tim15smen>;
impl Tim15smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15smen {
        match self.bits {
            false => Tim15smen::B0x0,
            true => Tim15smen::B0x1,
        }
    }
    #[doc = "TIM15 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15smen::B0x0
    }
    #[doc = "TIM15 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15smen::B0x1
    }
}
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim15smenW<'a, REG> = crate::BitWriter<'a, REG, Tim15smen>;
impl<'a, REG> Tim15smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM15 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15smen::B0x0)
    }
    #[doc = "TIM15 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15smen::B0x1)
    }
}
#[doc = "TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim16smen {
    #[doc = "0: TIM16 timer clocks disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM16 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim16smen> for bool {
    #[inline(always)]
    fn from(variant: Tim16smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim16smenR = crate::BitReader<Tim16smen>;
impl Tim16smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim16smen {
        match self.bits {
            false => Tim16smen::B0x0,
            true => Tim16smen::B0x1,
        }
    }
    #[doc = "TIM16 timer clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim16smen::B0x0
    }
    #[doc = "TIM16 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim16smen::B0x1
    }
}
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim16smenW<'a, REG> = crate::BitWriter<'a, REG, Tim16smen>;
impl<'a, REG> Tim16smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM16 timer clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16smen::B0x0)
    }
    #[doc = "TIM16 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16smen::B0x1)
    }
}
#[doc = "TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim17smen {
    #[doc = "0: TIM17 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM17 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim17smen> for bool {
    #[inline(always)]
    fn from(variant: Tim17smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM17SMEN` reader - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim17smenR = crate::BitReader<Tim17smen>;
impl Tim17smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim17smen {
        match self.bits {
            false => Tim17smen::B0x0,
            true => Tim17smen::B0x1,
        }
    }
    #[doc = "TIM17 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim17smen::B0x0
    }
    #[doc = "TIM17 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim17smen::B0x1
    }
}
#[doc = "Field `TIM17SMEN` writer - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim17smenW<'a, REG> = crate::BitWriter<'a, REG, Tim17smen>;
impl<'a, REG> Tim17smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM17 timer clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim17smen::B0x0)
    }
    #[doc = "TIM17 timer clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim17smen::B0x1)
    }
}
#[doc = "TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim20smen {
    #[doc = "0: TIM20 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: TIM20 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Tim20smen> for bool {
    #[inline(always)]
    fn from(variant: Tim20smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM20SMEN` reader - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim20smenR = crate::BitReader<Tim20smen>;
impl Tim20smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim20smen {
        match self.bits {
            false => Tim20smen::B0x0,
            true => Tim20smen::B0x1,
        }
    }
    #[doc = "TIM20 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim20smen::B0x0
    }
    #[doc = "TIM20 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim20smen::B0x1
    }
}
#[doc = "Field `TIM20SMEN` writer - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Tim20smenW<'a, REG> = crate::BitWriter<'a, REG, Tim20smen>;
impl<'a, REG> Tim20smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM20 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim20smen::B0x0)
    }
    #[doc = "TIM20 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim20smen::B0x1)
    }
}
#[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sai1smen {
    #[doc = "0: SAI1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SAI1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Sai1smen> for bool {
    #[inline(always)]
    fn from(variant: Sai1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAI1SMEN` reader - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Sai1smenR = crate::BitReader<Sai1smen>;
impl Sai1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sai1smen {
        match self.bits {
            false => Sai1smen::B0x0,
            true => Sai1smen::B0x1,
        }
    }
    #[doc = "SAI1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sai1smen::B0x0
    }
    #[doc = "SAI1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sai1smen::B0x1
    }
}
#[doc = "Field `SAI1SMEN` writer - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Sai1smenW<'a, REG> = crate::BitWriter<'a, REG, Sai1smen>;
impl<'a, REG> Sai1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAI1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1smen::B0x0)
    }
    #[doc = "SAI1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1smen::B0x1)
    }
}
#[doc = "HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrtim1smen {
    #[doc = "0: HRTIM1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: HRTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Hrtim1smen> for bool {
    #[inline(always)]
    fn from(variant: Hrtim1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRTIM1SMEN` reader - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Hrtim1smenR = crate::BitReader<Hrtim1smen>;
impl Hrtim1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrtim1smen {
        match self.bits {
            false => Hrtim1smen::B0x0,
            true => Hrtim1smen::B0x1,
        }
    }
    #[doc = "HRTIM1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hrtim1smen::B0x0
    }
    #[doc = "HRTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hrtim1smen::B0x1
    }
}
#[doc = "Field `HRTIM1SMEN` writer - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Hrtim1smenW<'a, REG> = crate::BitWriter<'a, REG, Hrtim1smen>;
impl<'a, REG> Hrtim1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRTIM1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrtim1smen::B0x0)
    }
    #[doc = "HRTIM1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrtim1smen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SyscfgsmenR {
        SyscfgsmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim1smen(&self) -> Tim1smenR {
        Tim1smenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi1smen(&self) -> Spi1smenR {
        Spi1smenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim8smen(&self) -> Tim8smenR {
        Tim8smenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn usart1smen(&self) -> Usart1smenR {
        Usart1smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn spi4smen(&self) -> Spi4smenR {
        Spi4smenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim15smen(&self) -> Tim15smenR {
        Tim15smenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim16smen(&self) -> Tim16smenR {
        Tim16smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim17smen(&self) -> Tim17smenR {
        Tim17smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn tim20smen(&self) -> Tim20smenR {
        Tim20smenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sai1smen(&self) -> Sai1smenR {
        Sai1smenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1smen(&self) -> Hrtim1smenR {
        Hrtim1smenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB2SMENR")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("tim1smen", &self.tim1smen())
            .field("spi1smen", &self.spi1smen())
            .field("tim8smen", &self.tim8smen())
            .field("usart1smen", &self.usart1smen())
            .field("spi4smen", &self.spi4smen())
            .field("tim15smen", &self.tim15smen())
            .field("tim16smen", &self.tim16smen())
            .field("tim17smen", &self.tim17smen())
            .field("tim20smen", &self.tim20smen())
            .field("sai1smen", &self.sai1smen())
            .field("hrtim1smen", &self.hrtim1smen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SyscfgsmenW<RccApb2smenrSpec> {
        SyscfgsmenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> Tim1smenW<RccApb2smenrSpec> {
        Tim1smenW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> Spi1smenW<RccApb2smenrSpec> {
        Spi1smenW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> Tim8smenW<RccApb2smenrSpec> {
        Tim8smenW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> Usart1smenW<RccApb2smenrSpec> {
        Usart1smenW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI4 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi4smen(&mut self) -> Spi4smenW<RccApb2smenrSpec> {
        Spi4smenW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> Tim15smenW<RccApb2smenrSpec> {
        Tim15smenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> Tim16smenW<RccApb2smenrSpec> {
        Tim16smenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> Tim17smenW<RccApb2smenrSpec> {
        Tim17smenW::new(self, 18)
    }
    #[doc = "Bit 20 - TIM20 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim20smen(&mut self) -> Tim20smenW<RccApb2smenrSpec> {
        Tim20smenW::new(self, 20)
    }
    #[doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> Sai1smenW<RccApb2smenrSpec> {
        Sai1smenW::new(self, 21)
    }
    #[doc = "Bit 26 - HRTIM1 timer clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn hrtim1smen(&mut self) -> Hrtim1smenW<RccApb2smenrSpec> {
        Hrtim1smenW::new(self, 26)
    }
}
#[doc = "APB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb2smenrSpec;
impl crate::RegisterSpec for RccApb2smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2smenr::R`](R) reader structure"]
impl crate::Readable for RccApb2smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2smenr::W`](W) writer structure"]
impl crate::Writable for RccApb2smenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB2SMENR to value 0x0437_f801"]
impl crate::Resettable for RccApb2smenrSpec {
    const RESET_VALUE: u32 = 0x0437_f801;
}
