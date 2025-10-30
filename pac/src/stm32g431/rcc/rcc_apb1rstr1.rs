#[doc = "Register `RCC_APB1RSTR1` reader"]
pub type R = crate::R<RccApb1rstr1Spec>;
#[doc = "Register `RCC_APB1RSTR1` writer"]
pub type W = crate::W<RccApb1rstr1Spec>;
#[doc = "TIM2 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM2"]
    B0x1 = 1,
}
impl From<Tim2rst> for bool {
    #[inline(always)]
    fn from(variant: Tim2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 timer reset Set and cleared by software."]
pub type Tim2rstR = crate::BitReader<Tim2rst>;
impl Tim2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2rst {
        match self.bits {
            false => Tim2rst::B0x0,
            true => Tim2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim2rst::B0x0
    }
    #[doc = "Reset TIM2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim2rst::B0x1
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 timer reset Set and cleared by software."]
pub type Tim2rstW<'a, REG> = crate::BitWriter<'a, REG, Tim2rst>;
impl<'a, REG> Tim2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2rst::B0x0)
    }
    #[doc = "Reset TIM2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2rst::B0x1)
    }
}
#[doc = "TIM3 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM3"]
    B0x1 = 1,
}
impl From<Tim3rst> for bool {
    #[inline(always)]
    fn from(variant: Tim3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3RST` reader - TIM3 timer reset Set and cleared by software."]
pub type Tim3rstR = crate::BitReader<Tim3rst>;
impl Tim3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3rst {
        match self.bits {
            false => Tim3rst::B0x0,
            true => Tim3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim3rst::B0x0
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim3rst::B0x1
    }
}
#[doc = "Field `TIM3RST` writer - TIM3 timer reset Set and cleared by software."]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG, Tim3rst>;
impl<'a, REG> Tim3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3rst::B0x0)
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3rst::B0x1)
    }
}
#[doc = "TIM3 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM3"]
    B0x1 = 1,
}
impl From<Tim4rst> for bool {
    #[inline(always)]
    fn from(variant: Tim4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM4RST` reader - TIM3 timer reset Set and cleared by software."]
pub type Tim4rstR = crate::BitReader<Tim4rst>;
impl Tim4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim4rst {
        match self.bits {
            false => Tim4rst::B0x0,
            true => Tim4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim4rst::B0x0
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim4rst::B0x1
    }
}
#[doc = "Field `TIM4RST` writer - TIM3 timer reset Set and cleared by software."]
pub type Tim4rstW<'a, REG> = crate::BitWriter<'a, REG, Tim4rst>;
impl<'a, REG> Tim4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim4rst::B0x0)
    }
    #[doc = "Reset TIM3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim4rst::B0x1)
    }
}
#[doc = "TIM5 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim5rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM5"]
    B0x1 = 1,
}
impl From<Tim5rst> for bool {
    #[inline(always)]
    fn from(variant: Tim5rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5RST` reader - TIM5 timer reset Set and cleared by software."]
pub type Tim5rstR = crate::BitReader<Tim5rst>;
impl Tim5rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim5rst {
        match self.bits {
            false => Tim5rst::B0x0,
            true => Tim5rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim5rst::B0x0
    }
    #[doc = "Reset TIM5"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim5rst::B0x1
    }
}
#[doc = "Field `TIM5RST` writer - TIM5 timer reset Set and cleared by software."]
pub type Tim5rstW<'a, REG> = crate::BitWriter<'a, REG, Tim5rst>;
impl<'a, REG> Tim5rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim5rst::B0x0)
    }
    #[doc = "Reset TIM5"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim5rst::B0x1)
    }
}
#[doc = "TIM6 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim6rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM7"]
    B0x1 = 1,
}
impl From<Tim6rst> for bool {
    #[inline(always)]
    fn from(variant: Tim6rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6RST` reader - TIM6 timer reset Set and cleared by software."]
pub type Tim6rstR = crate::BitReader<Tim6rst>;
impl Tim6rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim6rst {
        match self.bits {
            false => Tim6rst::B0x0,
            true => Tim6rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim6rst::B0x0
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim6rst::B0x1
    }
}
#[doc = "Field `TIM6RST` writer - TIM6 timer reset Set and cleared by software."]
pub type Tim6rstW<'a, REG> = crate::BitWriter<'a, REG, Tim6rst>;
impl<'a, REG> Tim6rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6rst::B0x0)
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6rst::B0x1)
    }
}
#[doc = "TIM7 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim7rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM7"]
    B0x1 = 1,
}
impl From<Tim7rst> for bool {
    #[inline(always)]
    fn from(variant: Tim7rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7RST` reader - TIM7 timer reset Set and cleared by software."]
pub type Tim7rstR = crate::BitReader<Tim7rst>;
impl Tim7rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim7rst {
        match self.bits {
            false => Tim7rst::B0x0,
            true => Tim7rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim7rst::B0x0
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim7rst::B0x1
    }
}
#[doc = "Field `TIM7RST` writer - TIM7 timer reset Set and cleared by software."]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG, Tim7rst>;
impl<'a, REG> Tim7rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7rst::B0x0)
    }
    #[doc = "Reset TIM7"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7rst::B0x1)
    }
}
#[doc = "CRS reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crsrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset CRS"]
    B0x1 = 1,
}
impl From<Crsrst> for bool {
    #[inline(always)]
    fn from(variant: Crsrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSRST` reader - CRS reset Set and cleared by software."]
pub type CrsrstR = crate::BitReader<Crsrst>;
impl CrsrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crsrst {
        match self.bits {
            false => Crsrst::B0x0,
            true => Crsrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crsrst::B0x0
    }
    #[doc = "Reset CRS"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crsrst::B0x1
    }
}
#[doc = "Field `CRSRST` writer - CRS reset Set and cleared by software."]
pub type CrsrstW<'a, REG> = crate::BitWriter<'a, REG, Crsrst>;
impl<'a, REG> CrsrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crsrst::B0x0)
    }
    #[doc = "Reset CRS"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crsrst::B0x1)
    }
}
#[doc = "SPI2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI2"]
    B0x1 = 1,
}
impl From<Spi2rst> for bool {
    #[inline(always)]
    fn from(variant: Spi2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2RST` reader - SPI2 reset Set and cleared by software."]
pub type Spi2rstR = crate::BitReader<Spi2rst>;
impl Spi2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi2rst {
        match self.bits {
            false => Spi2rst::B0x0,
            true => Spi2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi2rst::B0x0
    }
    #[doc = "Reset SPI2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi2rst::B0x1
    }
}
#[doc = "Field `SPI2RST` writer - SPI2 reset Set and cleared by software."]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG, Spi2rst>;
impl<'a, REG> Spi2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2rst::B0x0)
    }
    #[doc = "Reset SPI2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2rst::B0x1)
    }
}
#[doc = "SPI3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI3"]
    B0x1 = 1,
}
impl From<Spi3rst> for bool {
    #[inline(always)]
    fn from(variant: Spi3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3RST` reader - SPI3 reset Set and cleared by software."]
pub type Spi3rstR = crate::BitReader<Spi3rst>;
impl Spi3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi3rst {
        match self.bits {
            false => Spi3rst::B0x0,
            true => Spi3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi3rst::B0x0
    }
    #[doc = "Reset SPI3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi3rst::B0x1
    }
}
#[doc = "Field `SPI3RST` writer - SPI3 reset Set and cleared by software."]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG, Spi3rst>;
impl<'a, REG> Spi3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3rst::B0x0)
    }
    #[doc = "Reset SPI3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3rst::B0x1)
    }
}
#[doc = "USART2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART2"]
    B0x1 = 1,
}
impl From<Usart2rst> for bool {
    #[inline(always)]
    fn from(variant: Usart2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2RST` reader - USART2 reset Set and cleared by software."]
pub type Usart2rstR = crate::BitReader<Usart2rst>;
impl Usart2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2rst {
        match self.bits {
            false => Usart2rst::B0x0,
            true => Usart2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2rst::B0x0
    }
    #[doc = "Reset USART2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2rst::B0x1
    }
}
#[doc = "Field `USART2RST` writer - USART2 reset Set and cleared by software."]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG, Usart2rst>;
impl<'a, REG> Usart2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2rst::B0x0)
    }
    #[doc = "Reset USART2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2rst::B0x1)
    }
}
#[doc = "USART3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART3"]
    B0x1 = 1,
}
impl From<Usart3rst> for bool {
    #[inline(always)]
    fn from(variant: Usart3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3RST` reader - USART3 reset Set and cleared by software."]
pub type Usart3rstR = crate::BitReader<Usart3rst>;
impl Usart3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3rst {
        match self.bits {
            false => Usart3rst::B0x0,
            true => Usart3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3rst::B0x0
    }
    #[doc = "Reset USART3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3rst::B0x1
    }
}
#[doc = "Field `USART3RST` writer - USART3 reset Set and cleared by software."]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG, Usart3rst>;
impl<'a, REG> Usart3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3rst::B0x0)
    }
    #[doc = "Reset USART3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3rst::B0x1)
    }
}
#[doc = "UART4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset UART4"]
    B0x1 = 1,
}
impl From<Uart4rst> for bool {
    #[inline(always)]
    fn from(variant: Uart4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4RST` reader - UART4 reset Set and cleared by software."]
pub type Uart4rstR = crate::BitReader<Uart4rst>;
impl Uart4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart4rst {
        match self.bits {
            false => Uart4rst::B0x0,
            true => Uart4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart4rst::B0x0
    }
    #[doc = "Reset UART4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart4rst::B0x1
    }
}
#[doc = "Field `UART4RST` writer - UART4 reset Set and cleared by software."]
pub type Uart4rstW<'a, REG> = crate::BitWriter<'a, REG, Uart4rst>;
impl<'a, REG> Uart4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4rst::B0x0)
    }
    #[doc = "Reset UART4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4rst::B0x1)
    }
}
#[doc = "UART5 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart5rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset UART5"]
    B0x1 = 1,
}
impl From<Uart5rst> for bool {
    #[inline(always)]
    fn from(variant: Uart5rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART5RST` reader - UART5 reset Set and cleared by software."]
pub type Uart5rstR = crate::BitReader<Uart5rst>;
impl Uart5rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart5rst {
        match self.bits {
            false => Uart5rst::B0x0,
            true => Uart5rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart5rst::B0x0
    }
    #[doc = "Reset UART5"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart5rst::B0x1
    }
}
#[doc = "Field `UART5RST` writer - UART5 reset Set and cleared by software."]
pub type Uart5rstW<'a, REG> = crate::BitWriter<'a, REG, Uart5rst>;
impl<'a, REG> Uart5rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5rst::B0x0)
    }
    #[doc = "Reset UART5"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5rst::B0x1)
    }
}
#[doc = "I2C1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C1"]
    B0x1 = 1,
}
impl From<I2c1rst> for bool {
    #[inline(always)]
    fn from(variant: I2c1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1RST` reader - I2C1 reset Set and cleared by software."]
pub type I2c1rstR = crate::BitReader<I2c1rst>;
impl I2c1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1rst {
        match self.bits {
            false => I2c1rst::B0x0,
            true => I2c1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1rst::B0x0
    }
    #[doc = "Reset I2C1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1rst::B0x1
    }
}
#[doc = "Field `I2C1RST` writer - I2C1 reset Set and cleared by software."]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG, I2c1rst>;
impl<'a, REG> I2c1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1rst::B0x0)
    }
    #[doc = "Reset I2C1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1rst::B0x1)
    }
}
#[doc = "I2C2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C2"]
    B0x1 = 1,
}
impl From<I2c2rst> for bool {
    #[inline(always)]
    fn from(variant: I2c2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2RST` reader - I2C2 reset Set and cleared by software."]
pub type I2c2rstR = crate::BitReader<I2c2rst>;
impl I2c2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2rst {
        match self.bits {
            false => I2c2rst::B0x0,
            true => I2c2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2rst::B0x0
    }
    #[doc = "Reset I2C2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2rst::B0x1
    }
}
#[doc = "Field `I2C2RST` writer - I2C2 reset Set and cleared by software."]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG, I2c2rst>;
impl<'a, REG> I2c2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2rst::B0x0)
    }
    #[doc = "Reset I2C2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2rst::B0x1)
    }
}
#[doc = "USB device reset Set and reset by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USB device"]
    B0x1 = 1,
}
impl From<Usbrst> for bool {
    #[inline(always)]
    fn from(variant: Usbrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRST` reader - USB device reset Set and reset by software."]
pub type UsbrstR = crate::BitReader<Usbrst>;
impl UsbrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrst {
        match self.bits {
            false => Usbrst::B0x0,
            true => Usbrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usbrst::B0x0
    }
    #[doc = "Reset USB device"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usbrst::B0x1
    }
}
#[doc = "Field `USBRST` writer - USB device reset Set and reset by software."]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG, Usbrst>;
impl<'a, REG> UsbrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::B0x0)
    }
    #[doc = "Reset USB device"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::B0x1)
    }
}
#[doc = "FDCAN reset Set and reset by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdcanrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset the FDCAN"]
    B0x1 = 1,
}
impl From<Fdcanrst> for bool {
    #[inline(always)]
    fn from(variant: Fdcanrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCANRST` reader - FDCAN reset Set and reset by software."]
pub type FdcanrstR = crate::BitReader<Fdcanrst>;
impl FdcanrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdcanrst {
        match self.bits {
            false => Fdcanrst::B0x0,
            true => Fdcanrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fdcanrst::B0x0
    }
    #[doc = "Reset the FDCAN"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fdcanrst::B0x1
    }
}
#[doc = "Field `FDCANRST` writer - FDCAN reset Set and reset by software."]
pub type FdcanrstW<'a, REG> = crate::BitWriter<'a, REG, Fdcanrst>;
impl<'a, REG> FdcanrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcanrst::B0x0)
    }
    #[doc = "Reset the FDCAN"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcanrst::B0x1)
    }
}
#[doc = "Power interface reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset PWR"]
    B0x1 = 1,
}
impl From<Pwrrst> for bool {
    #[inline(always)]
    fn from(variant: Pwrrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRRST` reader - Power interface reset Set and cleared by software."]
pub type PwrrstR = crate::BitReader<Pwrrst>;
impl PwrrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrrst {
        match self.bits {
            false => Pwrrst::B0x0,
            true => Pwrrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwrrst::B0x0
    }
    #[doc = "Reset PWR"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwrrst::B0x1
    }
}
#[doc = "Field `PWRRST` writer - Power interface reset Set and cleared by software."]
pub type PwrrstW<'a, REG> = crate::BitWriter<'a, REG, Pwrrst>;
impl<'a, REG> PwrrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrrst::B0x0)
    }
    #[doc = "Reset PWR"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrrst::B0x1)
    }
}
#[doc = "I2C3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C3 interface"]
    B0x1 = 1,
}
impl From<I2c3rst> for bool {
    #[inline(always)]
    fn from(variant: I2c3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3RST` reader - I2C3 reset Set and cleared by software."]
pub type I2c3rstR = crate::BitReader<I2c3rst>;
impl I2c3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3rst {
        match self.bits {
            false => I2c3rst::B0x0,
            true => I2c3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3rst::B0x0
    }
    #[doc = "Reset I2C3 interface"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3rst::B0x1
    }
}
#[doc = "Field `I2C3RST` writer - I2C3 reset Set and cleared by software."]
pub type I2c3rstW<'a, REG> = crate::BitWriter<'a, REG, I2c3rst>;
impl<'a, REG> I2c3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3rst::B0x0)
    }
    #[doc = "Reset I2C3 interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3rst::B0x1)
    }
}
#[doc = "Low Power Timer 1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPTIM1"]
    B0x1 = 1,
}
impl From<Lptim1rst> for bool {
    #[inline(always)]
    fn from(variant: Lptim1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset Set and cleared by software."]
pub type Lptim1rstR = crate::BitReader<Lptim1rst>;
impl Lptim1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1rst {
        match self.bits {
            false => Lptim1rst::B0x0,
            true => Lptim1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1rst::B0x0
    }
    #[doc = "Reset LPTIM1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1rst::B0x1
    }
}
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset Set and cleared by software."]
pub type Lptim1rstW<'a, REG> = crate::BitWriter<'a, REG, Lptim1rst>;
impl<'a, REG> Lptim1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1rst::B0x0)
    }
    #[doc = "Reset LPTIM1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1rst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim2rst(&self) -> Tim2rstR {
        Tim2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim4rst(&self) -> Tim4rstR {
        Tim4rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim5rst(&self) -> Tim5rstR {
        Tim5rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim6rst(&self) -> Tim6rstR {
        Tim6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRS reset Set and cleared by software."]
    #[inline(always)]
    pub fn crsrst(&self) -> CrsrstR {
        CrsrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn uart4rst(&self) -> Uart4rstR {
        Uart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 reset Set and cleared by software."]
    #[inline(always)]
    pub fn uart5rst(&self) -> Uart5rstR {
        Uart5rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB device reset Set and reset by software."]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FdcanrstR {
        FdcanrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset Set and cleared by software."]
    #[inline(always)]
    pub fn pwrrst(&self) -> PwrrstR {
        PwrrstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2c3rstR {
        I2c3rstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> Lptim1rstR {
        Lptim1rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1RSTR1")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("crsrst", &self.crsrst())
            .field("spi2rst", &self.spi2rst())
            .field("spi3rst", &self.spi3rst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("uart4rst", &self.uart4rst())
            .field("uart5rst", &self.uart5rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("usbrst", &self.usbrst())
            .field("fdcanrst", &self.fdcanrst())
            .field("pwrrst", &self.pwrrst())
            .field("i2c3rst", &self.i2c3rst())
            .field("lptim1rst", &self.lptim1rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> Tim2rstW<'_, RccApb1rstr1Spec> {
        Tim2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> Tim3rstW<'_, RccApb1rstr1Spec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM3 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> Tim4rstW<'_, RccApb1rstr1Spec> {
        Tim4rstW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> Tim5rstW<'_, RccApb1rstr1Spec> {
        Tim5rstW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> Tim6rstW<'_, RccApb1rstr1Spec> {
        Tim6rstW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> Tim7rstW<'_, RccApb1rstr1Spec> {
        Tim7rstW::new(self, 5)
    }
    #[doc = "Bit 8 - CRS reset Set and cleared by software."]
    #[inline(always)]
    pub fn crsrst(&mut self) -> CrsrstW<'_, RccApb1rstr1Spec> {
        CrsrstW::new(self, 8)
    }
    #[doc = "Bit 14 - SPI2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> Spi2rstW<'_, RccApb1rstr1Spec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> Spi3rstW<'_, RccApb1rstr1Spec> {
        Spi3rstW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> Usart2rstW<'_, RccApb1rstr1Spec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> Usart3rstW<'_, RccApb1rstr1Spec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> Uart4rstW<'_, RccApb1rstr1Spec> {
        Uart4rstW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 reset Set and cleared by software."]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> Uart5rstW<'_, RccApb1rstr1Spec> {
        Uart5rstW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2c1rstW<'_, RccApb1rstr1Spec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2c2rstW<'_, RccApb1rstr1Spec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - USB device reset Set and reset by software."]
    #[inline(always)]
    pub fn usbrst(&mut self) -> UsbrstW<'_, RccApb1rstr1Spec> {
        UsbrstW::new(self, 23)
    }
    #[doc = "Bit 25 - FDCAN reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FdcanrstW<'_, RccApb1rstr1Spec> {
        FdcanrstW::new(self, 25)
    }
    #[doc = "Bit 28 - Power interface reset Set and cleared by software."]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PwrrstW<'_, RccApb1rstr1Spec> {
        PwrrstW::new(self, 28)
    }
    #[doc = "Bit 30 - I2C3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2c3rstW<'_, RccApb1rstr1Spec> {
        I2c3rstW::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> Lptim1rstW<'_, RccApb1rstr1Spec> {
        Lptim1rstW::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb1rstr1Spec;
impl crate::RegisterSpec for RccApb1rstr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1rstr1::R`](R) reader structure"]
impl crate::Readable for RccApb1rstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1rstr1::W`](W) writer structure"]
impl crate::Writable for RccApb1rstr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_APB1RSTR1 to value 0"]
impl crate::Resettable for RccApb1rstr1Spec {}
