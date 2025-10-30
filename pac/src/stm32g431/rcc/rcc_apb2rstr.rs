#[doc = "Register `RCC_APB2RSTR` reader"]
pub type R = crate::R<RccApb2rstrSpec>;
#[doc = "Register `RCC_APB2RSTR` writer"]
pub type W = crate::W<RccApb2rstrSpec>;
#[doc = "SYSCFG + COMP + OPAMP + VREFBUF reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscfgrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SYSCFG + COMP + OPAMP + VREFBUF"]
    B0x1 = 1,
}
impl From<Syscfgrst> for bool {
    #[inline(always)]
    fn from(variant: Syscfgrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG + COMP + OPAMP + VREFBUF reset"]
pub type SyscfgrstR = crate::BitReader<Syscfgrst>;
impl SyscfgrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscfgrst {
        match self.bits {
            false => Syscfgrst::B0x0,
            true => Syscfgrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syscfgrst::B0x0
    }
    #[doc = "Reset SYSCFG + COMP + OPAMP + VREFBUF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syscfgrst::B0x1
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG + COMP + OPAMP + VREFBUF reset"]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG, Syscfgrst>;
impl<'a, REG> SyscfgrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgrst::B0x0)
    }
    #[doc = "Reset SYSCFG + COMP + OPAMP + VREFBUF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgrst::B0x1)
    }
}
#[doc = "TIM1 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM1 timer"]
    B0x1 = 1,
}
impl From<Tim1rst> for bool {
    #[inline(always)]
    fn from(variant: Tim1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 timer reset Set and cleared by software."]
pub type Tim1rstR = crate::BitReader<Tim1rst>;
impl Tim1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1rst {
        match self.bits {
            false => Tim1rst::B0x0,
            true => Tim1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1rst::B0x0
    }
    #[doc = "Reset TIM1 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1rst::B0x1
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 timer reset Set and cleared by software."]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG, Tim1rst>;
impl<'a, REG> Tim1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1rst::B0x0)
    }
    #[doc = "Reset TIM1 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1rst::B0x1)
    }
}
#[doc = "SPI1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI1"]
    B0x1 = 1,
}
impl From<Spi1rst> for bool {
    #[inline(always)]
    fn from(variant: Spi1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 reset Set and cleared by software."]
pub type Spi1rstR = crate::BitReader<Spi1rst>;
impl Spi1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1rst {
        match self.bits {
            false => Spi1rst::B0x0,
            true => Spi1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi1rst::B0x0
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi1rst::B0x1
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 reset Set and cleared by software."]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG, Spi1rst>;
impl<'a, REG> Spi1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1rst::B0x0)
    }
    #[doc = "Reset SPI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1rst::B0x1)
    }
}
#[doc = "TIM8 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim8rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM8 timer"]
    B0x1 = 1,
}
impl From<Tim8rst> for bool {
    #[inline(always)]
    fn from(variant: Tim8rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM8RST` reader - TIM8 timer reset Set and cleared by software."]
pub type Tim8rstR = crate::BitReader<Tim8rst>;
impl Tim8rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim8rst {
        match self.bits {
            false => Tim8rst::B0x0,
            true => Tim8rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim8rst::B0x0
    }
    #[doc = "Reset TIM8 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim8rst::B0x1
    }
}
#[doc = "Field `TIM8RST` writer - TIM8 timer reset Set and cleared by software."]
pub type Tim8rstW<'a, REG> = crate::BitWriter<'a, REG, Tim8rst>;
impl<'a, REG> Tim8rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim8rst::B0x0)
    }
    #[doc = "Reset TIM8 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim8rst::B0x1)
    }
}
#[doc = "USART1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset USART1"]
    B0x1 = 1,
}
impl From<Usart1rst> for bool {
    #[inline(always)]
    fn from(variant: Usart1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1RST` reader - USART1 reset Set and cleared by software."]
pub type Usart1rstR = crate::BitReader<Usart1rst>;
impl Usart1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1rst {
        match self.bits {
            false => Usart1rst::B0x0,
            true => Usart1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1rst::B0x0
    }
    #[doc = "Reset USART1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1rst::B0x1
    }
}
#[doc = "Field `USART1RST` writer - USART1 reset Set and cleared by software."]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG, Usart1rst>;
impl<'a, REG> Usart1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1rst::B0x0)
    }
    #[doc = "Reset USART1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1rst::B0x1)
    }
}
#[doc = "SPI4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SPI4"]
    B0x1 = 1,
}
impl From<Spi4rst> for bool {
    #[inline(always)]
    fn from(variant: Spi4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI4RST` reader - SPI4 reset Set and cleared by software."]
pub type Spi4rstR = crate::BitReader<Spi4rst>;
impl Spi4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi4rst {
        match self.bits {
            false => Spi4rst::B0x0,
            true => Spi4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi4rst::B0x0
    }
    #[doc = "Reset SPI4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi4rst::B0x1
    }
}
#[doc = "Field `SPI4RST` writer - SPI4 reset Set and cleared by software."]
pub type Spi4rstW<'a, REG> = crate::BitWriter<'a, REG, Spi4rst>;
impl<'a, REG> Spi4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi4rst::B0x0)
    }
    #[doc = "Reset SPI4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi4rst::B0x1)
    }
}
#[doc = "TIM15 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM15 timer"]
    B0x1 = 1,
}
impl From<Tim15rst> for bool {
    #[inline(always)]
    fn from(variant: Tim15rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15RST` reader - TIM15 timer reset Set and cleared by software."]
pub type Tim15rstR = crate::BitReader<Tim15rst>;
impl Tim15rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15rst {
        match self.bits {
            false => Tim15rst::B0x0,
            true => Tim15rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15rst::B0x0
    }
    #[doc = "Reset TIM15 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15rst::B0x1
    }
}
#[doc = "Field `TIM15RST` writer - TIM15 timer reset Set and cleared by software."]
pub type Tim15rstW<'a, REG> = crate::BitWriter<'a, REG, Tim15rst>;
impl<'a, REG> Tim15rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15rst::B0x0)
    }
    #[doc = "Reset TIM15 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15rst::B0x1)
    }
}
#[doc = "TIM16 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim16rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM16 timer"]
    B0x1 = 1,
}
impl From<Tim16rst> for bool {
    #[inline(always)]
    fn from(variant: Tim16rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16RST` reader - TIM16 timer reset Set and cleared by software."]
pub type Tim16rstR = crate::BitReader<Tim16rst>;
impl Tim16rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim16rst {
        match self.bits {
            false => Tim16rst::B0x0,
            true => Tim16rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim16rst::B0x0
    }
    #[doc = "Reset TIM16 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim16rst::B0x1
    }
}
#[doc = "Field `TIM16RST` writer - TIM16 timer reset Set and cleared by software."]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG, Tim16rst>;
impl<'a, REG> Tim16rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16rst::B0x0)
    }
    #[doc = "Reset TIM16 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16rst::B0x1)
    }
}
#[doc = "TIM17 timer reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim17rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM17 timer"]
    B0x1 = 1,
}
impl From<Tim17rst> for bool {
    #[inline(always)]
    fn from(variant: Tim17rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM17RST` reader - TIM17 timer reset Set and cleared by software."]
pub type Tim17rstR = crate::BitReader<Tim17rst>;
impl Tim17rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim17rst {
        match self.bits {
            false => Tim17rst::B0x0,
            true => Tim17rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim17rst::B0x0
    }
    #[doc = "Reset TIM17 timer"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim17rst::B0x1
    }
}
#[doc = "Field `TIM17RST` writer - TIM17 timer reset Set and cleared by software."]
pub type Tim17rstW<'a, REG> = crate::BitWriter<'a, REG, Tim17rst>;
impl<'a, REG> Tim17rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim17rst::B0x0)
    }
    #[doc = "Reset TIM17 timer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim17rst::B0x1)
    }
}
#[doc = "TIM20 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim20rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset TIM20"]
    B0x1 = 1,
}
impl From<Tim20rst> for bool {
    #[inline(always)]
    fn from(variant: Tim20rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM20RST` reader - TIM20 reset Set and cleared by software."]
pub type Tim20rstR = crate::BitReader<Tim20rst>;
impl Tim20rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim20rst {
        match self.bits {
            false => Tim20rst::B0x0,
            true => Tim20rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim20rst::B0x0
    }
    #[doc = "Reset TIM20"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim20rst::B0x1
    }
}
#[doc = "Field `TIM20RST` writer - TIM20 reset Set and cleared by software."]
pub type Tim20rstW<'a, REG> = crate::BitWriter<'a, REG, Tim20rst>;
impl<'a, REG> Tim20rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim20rst::B0x0)
    }
    #[doc = "Reset TIM20"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim20rst::B0x1)
    }
}
#[doc = "Serial audio interface 1 (SAI1) reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sai1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset SAI1"]
    B0x1 = 1,
}
impl From<Sai1rst> for bool {
    #[inline(always)]
    fn from(variant: Sai1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
pub type Sai1rstR = crate::BitReader<Sai1rst>;
impl Sai1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sai1rst {
        match self.bits {
            false => Sai1rst::B0x0,
            true => Sai1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sai1rst::B0x0
    }
    #[doc = "Reset SAI1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sai1rst::B0x1
    }
}
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
pub type Sai1rstW<'a, REG> = crate::BitWriter<'a, REG, Sai1rst>;
impl<'a, REG> Sai1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1rst::B0x0)
    }
    #[doc = "Reset SAI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1rst::B0x1)
    }
}
#[doc = "HRTIM1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrtim1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset HRTIM1"]
    B0x1 = 1,
}
impl From<Hrtim1rst> for bool {
    #[inline(always)]
    fn from(variant: Hrtim1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRTIM1RST` reader - HRTIM1 reset Set and cleared by software."]
pub type Hrtim1rstR = crate::BitReader<Hrtim1rst>;
impl Hrtim1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrtim1rst {
        match self.bits {
            false => Hrtim1rst::B0x0,
            true => Hrtim1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hrtim1rst::B0x0
    }
    #[doc = "Reset HRTIM1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hrtim1rst::B0x1
    }
}
#[doc = "Field `HRTIM1RST` writer - HRTIM1 reset Set and cleared by software."]
pub type Hrtim1rstW<'a, REG> = crate::BitWriter<'a, REG, Hrtim1rst>;
impl<'a, REG> Hrtim1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrtim1rst::B0x0)
    }
    #[doc = "Reset HRTIM1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrtim1rst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG + COMP + OPAMP + VREFBUF reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        Tim8rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi4rst(&self) -> Spi4rstR {
        Spi4rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim15rst(&self) -> Tim15rstR {
        Tim15rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim17rst(&self) -> Tim17rstR {
        Tim17rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim20rst(&self) -> Tim20rstR {
        Tim20rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
    #[inline(always)]
    pub fn sai1rst(&self) -> Sai1rstR {
        Sai1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIM1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1rst(&self) -> Hrtim1rstR {
        Hrtim1rstR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB2RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("spi4rst", &self.spi4rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("tim20rst", &self.tim20rst())
            .field("sai1rst", &self.sai1rst())
            .field("hrtim1rst", &self.hrtim1rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG + COMP + OPAMP + VREFBUF reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<'_, RccApb2rstrSpec> {
        SyscfgrstW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> Tim1rstW<'_, RccApb2rstrSpec> {
        Tim1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<'_, RccApb2rstrSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> Tim8rstW<'_, RccApb2rstrSpec> {
        Tim8rstW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<'_, RccApb2rstrSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> Spi4rstW<'_, RccApb2rstrSpec> {
        Spi4rstW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> Tim15rstW<'_, RccApb2rstrSpec> {
        Tim15rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> Tim16rstW<'_, RccApb2rstrSpec> {
        Tim16rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> Tim17rstW<'_, RccApb2rstrSpec> {
        Tim17rstW::new(self, 18)
    }
    #[doc = "Bit 20 - TIM20 reset Set and cleared by software."]
    #[inline(always)]
    pub fn tim20rst(&mut self) -> Tim20rstW<'_, RccApb2rstrSpec> {
        Tim20rstW::new(self, 20)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset Set and cleared by software."]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> Sai1rstW<'_, RccApb2rstrSpec> {
        Sai1rstW::new(self, 21)
    }
    #[doc = "Bit 26 - HRTIM1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1rst(&mut self) -> Hrtim1rstW<'_, RccApb2rstrSpec> {
        Hrtim1rstW::new(self, 26)
    }
}
#[doc = "APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb2rstrSpec;
impl crate::RegisterSpec for RccApb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2rstr::R`](R) reader structure"]
impl crate::Readable for RccApb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2rstr::W`](W) writer structure"]
impl crate::Writable for RccApb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_APB2RSTR to value 0"]
impl crate::Resettable for RccApb2rstrSpec {}
