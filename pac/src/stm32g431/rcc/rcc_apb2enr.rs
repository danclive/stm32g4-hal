#[doc = "Register `RCC_APB2ENR` reader"]
pub type R = crate::R<RccApb2enrSpec>;
#[doc = "Register `RCC_APB2ENR` writer"]
pub type W = crate::W<RccApb2enrSpec>;
#[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syscfgen {
    #[doc = "0: SYSCFG + COMP + VREFBUF + OPAMP clock disabled"]
    B0x0 = 0,
    #[doc = "1: SYSCFG + COMP + VREFBUF + OPAMP clock enabled"]
    B0x1 = 1,
}
impl From<Syscfgen> for bool {
    #[inline(always)]
    fn from(variant: Syscfgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
pub type SyscfgenR = crate::BitReader<Syscfgen>;
impl SyscfgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syscfgen {
        match self.bits {
            false => Syscfgen::B0x0,
            true => Syscfgen::B0x1,
        }
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Syscfgen::B0x0
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Syscfgen::B0x1
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG, Syscfgen>;
impl<'a, REG> SyscfgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgen::B0x0)
    }
    #[doc = "SYSCFG + COMP + VREFBUF + OPAMP clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Syscfgen::B0x1)
    }
}
#[doc = "TIM1 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim1en {
    #[doc = "0: TIM1 timer clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM1P timer clock enabled"]
    B0x1 = 1,
}
impl From<Tim1en> for bool {
    #[inline(always)]
    fn from(variant: Tim1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable Set and cleared by software."]
pub type Tim1enR = crate::BitReader<Tim1en>;
impl Tim1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim1en {
        match self.bits {
            false => Tim1en::B0x0,
            true => Tim1en::B0x1,
        }
    }
    #[doc = "TIM1 timer clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim1en::B0x0
    }
    #[doc = "TIM1P timer clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim1en::B0x1
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable Set and cleared by software."]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG, Tim1en>;
impl<'a, REG> Tim1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM1 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1en::B0x0)
    }
    #[doc = "TIM1P timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim1en::B0x1)
    }
}
#[doc = "SPI1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1en {
    #[doc = "0: SPI1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: SPI1 clock enabled"]
    B0x1 = 1,
}
impl From<Spi1en> for bool {
    #[inline(always)]
    fn from(variant: Spi1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and cleared by software."]
pub type Spi1enR = crate::BitReader<Spi1en>;
impl Spi1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1en {
        match self.bits {
            false => Spi1en::B0x0,
            true => Spi1en::B0x1,
        }
    }
    #[doc = "SPI1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi1en::B0x0
    }
    #[doc = "SPI1 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi1en::B0x1
    }
}
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and cleared by software."]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG, Spi1en>;
impl<'a, REG> Spi1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1en::B0x0)
    }
    #[doc = "SPI1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1en::B0x1)
    }
}
#[doc = "TIM8 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim8en {
    #[doc = "0: TIM8 timer clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM8 timer clock enabled"]
    B0x1 = 1,
}
impl From<Tim8en> for bool {
    #[inline(always)]
    fn from(variant: Tim8en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM8EN` reader - TIM8 timer clock enable Set and cleared by software."]
pub type Tim8enR = crate::BitReader<Tim8en>;
impl Tim8enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim8en {
        match self.bits {
            false => Tim8en::B0x0,
            true => Tim8en::B0x1,
        }
    }
    #[doc = "TIM8 timer clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim8en::B0x0
    }
    #[doc = "TIM8 timer clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim8en::B0x1
    }
}
#[doc = "Field `TIM8EN` writer - TIM8 timer clock enable Set and cleared by software."]
pub type Tim8enW<'a, REG> = crate::BitWriter<'a, REG, Tim8en>;
impl<'a, REG> Tim8enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM8 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim8en::B0x0)
    }
    #[doc = "TIM8 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim8en::B0x1)
    }
}
#[doc = "USART1clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1en {
    #[doc = "0: USART1clock disabled"]
    B0x0 = 0,
    #[doc = "1: USART1clock enabled"]
    B0x1 = 1,
}
impl From<Usart1en> for bool {
    #[inline(always)]
    fn from(variant: Usart1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1EN` reader - USART1clock enable Set and cleared by software."]
pub type Usart1enR = crate::BitReader<Usart1en>;
impl Usart1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1en {
        match self.bits {
            false => Usart1en::B0x0,
            true => Usart1en::B0x1,
        }
    }
    #[doc = "USART1clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1en::B0x0
    }
    #[doc = "USART1clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1en::B0x1
    }
}
#[doc = "Field `USART1EN` writer - USART1clock enable Set and cleared by software."]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG, Usart1en>;
impl<'a, REG> Usart1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART1clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1en::B0x0)
    }
    #[doc = "USART1clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1en::B0x1)
    }
}
#[doc = "SPI4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi4en {
    #[doc = "0: SPI4 clock disabled"]
    B0x0 = 0,
    #[doc = "1: SPI4 clock enabled"]
    B0x1 = 1,
}
impl From<Spi4en> for bool {
    #[inline(always)]
    fn from(variant: Spi4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI4EN` reader - SPI4 clock enable Set and cleared by software."]
pub type Spi4enR = crate::BitReader<Spi4en>;
impl Spi4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi4en {
        match self.bits {
            false => Spi4en::B0x0,
            true => Spi4en::B0x1,
        }
    }
    #[doc = "SPI4 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi4en::B0x0
    }
    #[doc = "SPI4 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi4en::B0x1
    }
}
#[doc = "Field `SPI4EN` writer - SPI4 clock enable Set and cleared by software."]
pub type Spi4enW<'a, REG> = crate::BitWriter<'a, REG, Spi4en>;
impl<'a, REG> Spi4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi4en::B0x0)
    }
    #[doc = "SPI4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi4en::B0x1)
    }
}
#[doc = "TIM15 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim15en {
    #[doc = "0: TIM15 timer clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM15 timer clock enabled"]
    B0x1 = 1,
}
impl From<Tim15en> for bool {
    #[inline(always)]
    fn from(variant: Tim15en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable Set and cleared by software."]
pub type Tim15enR = crate::BitReader<Tim15en>;
impl Tim15enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim15en {
        match self.bits {
            false => Tim15en::B0x0,
            true => Tim15en::B0x1,
        }
    }
    #[doc = "TIM15 timer clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim15en::B0x0
    }
    #[doc = "TIM15 timer clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim15en::B0x1
    }
}
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable Set and cleared by software."]
pub type Tim15enW<'a, REG> = crate::BitWriter<'a, REG, Tim15en>;
impl<'a, REG> Tim15enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM15 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15en::B0x0)
    }
    #[doc = "TIM15 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim15en::B0x1)
    }
}
#[doc = "TIM16 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim16en {
    #[doc = "0: TIM16 timer clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM16 timer clock enabled"]
    B0x1 = 1,
}
impl From<Tim16en> for bool {
    #[inline(always)]
    fn from(variant: Tim16en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable Set and cleared by software."]
pub type Tim16enR = crate::BitReader<Tim16en>;
impl Tim16enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim16en {
        match self.bits {
            false => Tim16en::B0x0,
            true => Tim16en::B0x1,
        }
    }
    #[doc = "TIM16 timer clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim16en::B0x0
    }
    #[doc = "TIM16 timer clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim16en::B0x1
    }
}
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable Set and cleared by software."]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG, Tim16en>;
impl<'a, REG> Tim16enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM16 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16en::B0x0)
    }
    #[doc = "TIM16 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim16en::B0x1)
    }
}
#[doc = "TIM17 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim17en {
    #[doc = "0: TIM17 timer clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM17 timer clock enabled"]
    B0x1 = 1,
}
impl From<Tim17en> for bool {
    #[inline(always)]
    fn from(variant: Tim17en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable Set and cleared by software."]
pub type Tim17enR = crate::BitReader<Tim17en>;
impl Tim17enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim17en {
        match self.bits {
            false => Tim17en::B0x0,
            true => Tim17en::B0x1,
        }
    }
    #[doc = "TIM17 timer clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim17en::B0x0
    }
    #[doc = "TIM17 timer clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim17en::B0x1
    }
}
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable Set and cleared by software."]
pub type Tim17enW<'a, REG> = crate::BitWriter<'a, REG, Tim17en>;
impl<'a, REG> Tim17enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM17 timer clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim17en::B0x0)
    }
    #[doc = "TIM17 timer clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim17en::B0x1)
    }
}
#[doc = "TIM20 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim20en {
    #[doc = "0: TIM20 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM20 clock enabled"]
    B0x1 = 1,
}
impl From<Tim20en> for bool {
    #[inline(always)]
    fn from(variant: Tim20en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM20EN` reader - TIM20 timer clock enable Set and cleared by software."]
pub type Tim20enR = crate::BitReader<Tim20en>;
impl Tim20enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim20en {
        match self.bits {
            false => Tim20en::B0x0,
            true => Tim20en::B0x1,
        }
    }
    #[doc = "TIM20 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim20en::B0x0
    }
    #[doc = "TIM20 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim20en::B0x1
    }
}
#[doc = "Field `TIM20EN` writer - TIM20 timer clock enable Set and cleared by software."]
pub type Tim20enW<'a, REG> = crate::BitWriter<'a, REG, Tim20en>;
impl<'a, REG> Tim20enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM20 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim20en::B0x0)
    }
    #[doc = "TIM20 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim20en::B0x1)
    }
}
#[doc = "SAI1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sai1en {
    #[doc = "0: SAI1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: SAI1 clock enabled"]
    B0x1 = 1,
}
impl From<Sai1en> for bool {
    #[inline(always)]
    fn from(variant: Sai1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAI1EN` reader - SAI1 clock enable Set and cleared by software."]
pub type Sai1enR = crate::BitReader<Sai1en>;
impl Sai1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sai1en {
        match self.bits {
            false => Sai1en::B0x0,
            true => Sai1en::B0x1,
        }
    }
    #[doc = "SAI1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sai1en::B0x0
    }
    #[doc = "SAI1 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sai1en::B0x1
    }
}
#[doc = "Field `SAI1EN` writer - SAI1 clock enable Set and cleared by software."]
pub type Sai1enW<'a, REG> = crate::BitWriter<'a, REG, Sai1en>;
impl<'a, REG> Sai1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAI1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1en::B0x0)
    }
    #[doc = "SAI1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1en::B0x1)
    }
}
#[doc = "HRTIM1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrtim1en {
    #[doc = "0: HRTIM1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: HRTIM1 clock enable"]
    B0x1 = 1,
}
impl From<Hrtim1en> for bool {
    #[inline(always)]
    fn from(variant: Hrtim1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRTIM1EN` reader - HRTIM1 clock enable Set and cleared by software."]
pub type Hrtim1enR = crate::BitReader<Hrtim1en>;
impl Hrtim1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrtim1en {
        match self.bits {
            false => Hrtim1en::B0x0,
            true => Hrtim1en::B0x1,
        }
    }
    #[doc = "HRTIM1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hrtim1en::B0x0
    }
    #[doc = "HRTIM1 clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hrtim1en::B0x1
    }
}
#[doc = "Field `HRTIM1EN` writer - HRTIM1 clock enable Set and cleared by software."]
pub type Hrtim1enW<'a, REG> = crate::BitWriter<'a, REG, Hrtim1en>;
impl<'a, REG> Hrtim1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRTIM1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hrtim1en::B0x0)
    }
    #[doc = "HRTIM1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrtim1en::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi4en(&self) -> Spi4enR {
        Spi4enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim15en(&self) -> Tim15enR {
        Tim15enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim17en(&self) -> Tim17enR {
        Tim17enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim20en(&self) -> Tim20enR {
        Tim20enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn sai1en(&self) -> Sai1enR {
        Sai1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - HRTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1en(&self) -> Hrtim1enR {
        Hrtim1enR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB2ENR")
            .field("syscfgen", &self.syscfgen())
            .field("tim1en", &self.tim1en())
            .field("spi1en", &self.spi1en())
            .field("tim8en", &self.tim8en())
            .field("usart1en", &self.usart1en())
            .field("spi4en", &self.spi4en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("tim20en", &self.tim20en())
            .field("sai1en", &self.sai1en())
            .field("hrtim1en", &self.hrtim1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG + COMP + VREFBUF + OPAMP clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<RccApb2enrSpec> {
        SyscfgenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim1en(&mut self) -> Tim1enW<RccApb2enrSpec> {
        Tim1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<RccApb2enrSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim8en(&mut self) -> Tim8enW<RccApb2enrSpec> {
        Tim8enW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<RccApb2enrSpec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi4en(&mut self) -> Spi4enW<RccApb2enrSpec> {
        Spi4enW::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim15en(&mut self) -> Tim15enW<RccApb2enrSpec> {
        Tim15enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim16en(&mut self) -> Tim16enW<RccApb2enrSpec> {
        Tim16enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim17en(&mut self) -> Tim17enW<RccApb2enrSpec> {
        Tim17enW::new(self, 18)
    }
    #[doc = "Bit 20 - TIM20 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim20en(&mut self) -> Tim20enW<RccApb2enrSpec> {
        Tim20enW::new(self, 20)
    }
    #[doc = "Bit 21 - SAI1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn sai1en(&mut self) -> Sai1enW<RccApb2enrSpec> {
        Sai1enW::new(self, 21)
    }
    #[doc = "Bit 26 - HRTIM1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn hrtim1en(&mut self) -> Hrtim1enW<RccApb2enrSpec> {
        Hrtim1enW::new(self, 26)
    }
}
#[doc = "APB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb2enrSpec;
impl crate::RegisterSpec for RccApb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2enr::R`](R) reader structure"]
impl crate::Readable for RccApb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2enr::W`](W) writer structure"]
impl crate::Writable for RccApb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_APB2ENR to value 0"]
impl crate::Resettable for RccApb2enrSpec {}
