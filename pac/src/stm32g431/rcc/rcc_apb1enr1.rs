#[doc = "Register `RCC_APB1ENR1` reader"]
pub type R = crate::R<RccApb1enr1Spec>;
#[doc = "Register `RCC_APB1ENR1` writer"]
pub type W = crate::W<RccApb1enr1Spec>;
#[doc = "TIM2 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim2en {
    #[doc = "0: TIM2 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM2 clock enabled"]
    B0x1 = 1,
}
impl From<Tim2en> for bool {
    #[inline(always)]
    fn from(variant: Tim2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable Set and cleared by software."]
pub type Tim2enR = crate::BitReader<Tim2en>;
impl Tim2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim2en {
        match self.bits {
            false => Tim2en::B0x0,
            true => Tim2en::B0x1,
        }
    }
    #[doc = "TIM2 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim2en::B0x0
    }
    #[doc = "TIM2 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim2en::B0x1
    }
}
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable Set and cleared by software."]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG, Tim2en>;
impl<'a, REG> Tim2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2en::B0x0)
    }
    #[doc = "TIM2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim2en::B0x1)
    }
}
#[doc = "TIM3 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim3en {
    #[doc = "0: TIM3 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM3 clock enabled"]
    B0x1 = 1,
}
impl From<Tim3en> for bool {
    #[inline(always)]
    fn from(variant: Tim3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software."]
pub type Tim3enR = crate::BitReader<Tim3en>;
impl Tim3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim3en {
        match self.bits {
            false => Tim3en::B0x0,
            true => Tim3en::B0x1,
        }
    }
    #[doc = "TIM3 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim3en::B0x0
    }
    #[doc = "TIM3 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim3en::B0x1
    }
}
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software."]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG, Tim3en>;
impl<'a, REG> Tim3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3en::B0x0)
    }
    #[doc = "TIM3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim3en::B0x1)
    }
}
#[doc = "TIM4 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim4en {
    #[doc = "0: TIM4 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM4 clock enabled"]
    B0x1 = 1,
}
impl From<Tim4en> for bool {
    #[inline(always)]
    fn from(variant: Tim4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM4EN` reader - TIM4 timer clock enable Set and cleared by software."]
pub type Tim4enR = crate::BitReader<Tim4en>;
impl Tim4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim4en {
        match self.bits {
            false => Tim4en::B0x0,
            true => Tim4en::B0x1,
        }
    }
    #[doc = "TIM4 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim4en::B0x0
    }
    #[doc = "TIM4 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim4en::B0x1
    }
}
#[doc = "Field `TIM4EN` writer - TIM4 timer clock enable Set and cleared by software."]
pub type Tim4enW<'a, REG> = crate::BitWriter<'a, REG, Tim4en>;
impl<'a, REG> Tim4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim4en::B0x0)
    }
    #[doc = "TIM4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim4en::B0x1)
    }
}
#[doc = "TIM5 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim5en {
    #[doc = "0: TIM5 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM5 clock enabled"]
    B0x1 = 1,
}
impl From<Tim5en> for bool {
    #[inline(always)]
    fn from(variant: Tim5en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5EN` reader - TIM5 timer clock enable Set and cleared by software."]
pub type Tim5enR = crate::BitReader<Tim5en>;
impl Tim5enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim5en {
        match self.bits {
            false => Tim5en::B0x0,
            true => Tim5en::B0x1,
        }
    }
    #[doc = "TIM5 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim5en::B0x0
    }
    #[doc = "TIM5 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim5en::B0x1
    }
}
#[doc = "Field `TIM5EN` writer - TIM5 timer clock enable Set and cleared by software."]
pub type Tim5enW<'a, REG> = crate::BitWriter<'a, REG, Tim5en>;
impl<'a, REG> Tim5enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM5 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim5en::B0x0)
    }
    #[doc = "TIM5 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim5en::B0x1)
    }
}
#[doc = "TIM6 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim6en {
    #[doc = "0: TIM6 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM6 clock enabled"]
    B0x1 = 1,
}
impl From<Tim6en> for bool {
    #[inline(always)]
    fn from(variant: Tim6en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable Set and cleared by software."]
pub type Tim6enR = crate::BitReader<Tim6en>;
impl Tim6enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim6en {
        match self.bits {
            false => Tim6en::B0x0,
            true => Tim6en::B0x1,
        }
    }
    #[doc = "TIM6 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim6en::B0x0
    }
    #[doc = "TIM6 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim6en::B0x1
    }
}
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable Set and cleared by software."]
pub type Tim6enW<'a, REG> = crate::BitWriter<'a, REG, Tim6en>;
impl<'a, REG> Tim6enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM6 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6en::B0x0)
    }
    #[doc = "TIM6 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim6en::B0x1)
    }
}
#[doc = "TIM7 timer clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tim7en {
    #[doc = "0: TIM7 clock disabled"]
    B0x0 = 0,
    #[doc = "1: TIM7 clock enabled"]
    B0x1 = 1,
}
impl From<Tim7en> for bool {
    #[inline(always)]
    fn from(variant: Tim7en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable Set and cleared by software."]
pub type Tim7enR = crate::BitReader<Tim7en>;
impl Tim7enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tim7en {
        match self.bits {
            false => Tim7en::B0x0,
            true => Tim7en::B0x1,
        }
    }
    #[doc = "TIM7 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Tim7en::B0x0
    }
    #[doc = "TIM7 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Tim7en::B0x1
    }
}
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable Set and cleared by software."]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG, Tim7en>;
impl<'a, REG> Tim7enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM7 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7en::B0x0)
    }
    #[doc = "TIM7 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Tim7en::B0x1)
    }
}
#[doc = "CRS Recovery System clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crsen {
    #[doc = "0: CRS clock disabled"]
    B0x0 = 0,
    #[doc = "1: CRS clock enabled"]
    B0x1 = 1,
}
impl From<Crsen> for bool {
    #[inline(always)]
    fn from(variant: Crsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSEN` reader - CRS Recovery System clock enable Set and cleared by software."]
pub type CrsenR = crate::BitReader<Crsen>;
impl CrsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crsen {
        match self.bits {
            false => Crsen::B0x0,
            true => Crsen::B0x1,
        }
    }
    #[doc = "CRS clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crsen::B0x0
    }
    #[doc = "CRS clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crsen::B0x1
    }
}
#[doc = "Field `CRSEN` writer - CRS Recovery System clock enable Set and cleared by software."]
pub type CrsenW<'a, REG> = crate::BitWriter<'a, REG, Crsen>;
impl<'a, REG> CrsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRS clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crsen::B0x0)
    }
    #[doc = "CRS clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crsen::B0x1)
    }
}
#[doc = "RTC APB clock enable Set and cleared by software\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcapben {
    #[doc = "0: RTC APB clock disabled"]
    B0x0 = 0,
    #[doc = "1: RTC APB clock enabled"]
    B0x1 = 1,
}
impl From<Rtcapben> for bool {
    #[inline(always)]
    fn from(variant: Rtcapben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software"]
pub type RtcapbenR = crate::BitReader<Rtcapben>;
impl RtcapbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcapben {
        match self.bits {
            false => Rtcapben::B0x0,
            true => Rtcapben::B0x1,
        }
    }
    #[doc = "RTC APB clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rtcapben::B0x0
    }
    #[doc = "RTC APB clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rtcapben::B0x1
    }
}
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software"]
pub type RtcapbenW<'a, REG> = crate::BitWriter<'a, REG, Rtcapben>;
impl<'a, REG> RtcapbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC APB clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapben::B0x0)
    }
    #[doc = "RTC APB clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcapben::B0x1)
    }
}
#[doc = "Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdgen {
    #[doc = "0: Window watchdog clock disabled"]
    B0x0 = 0,
    #[doc = "1: Window watchdog clock enabled"]
    B0x1 = 1,
}
impl From<Wwdgen> for bool {
    #[inline(always)]
    fn from(variant: Wwdgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
pub type WwdgenR = crate::BitReader<Wwdgen>;
impl WwdgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdgen {
        match self.bits {
            false => Wwdgen::B0x0,
            true => Wwdgen::B0x1,
        }
    }
    #[doc = "Window watchdog clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Wwdgen::B0x0
    }
    #[doc = "Window watchdog clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Wwdgen::B0x1
    }
}
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
pub type WwdgenW<'a, REG> = crate::BitWriter<'a, REG, Wwdgen>;
impl<'a, REG> WwdgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Window watchdog clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgen::B0x0)
    }
    #[doc = "Window watchdog clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdgen::B0x1)
    }
}
#[doc = "SPI2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi2en {
    #[doc = "0: SPI2 clock disabled"]
    B0x0 = 0,
    #[doc = "1: SPI2 clock enabled"]
    B0x1 = 1,
}
impl From<Spi2en> for bool {
    #[inline(always)]
    fn from(variant: Spi2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI2EN` reader - SPI2 clock enable Set and cleared by software."]
pub type Spi2enR = crate::BitReader<Spi2en>;
impl Spi2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi2en {
        match self.bits {
            false => Spi2en::B0x0,
            true => Spi2en::B0x1,
        }
    }
    #[doc = "SPI2 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi2en::B0x0
    }
    #[doc = "SPI2 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi2en::B0x1
    }
}
#[doc = "Field `SPI2EN` writer - SPI2 clock enable Set and cleared by software."]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG, Spi2en>;
impl<'a, REG> Spi2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2en::B0x0)
    }
    #[doc = "SPI2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi2en::B0x1)
    }
}
#[doc = "SPI3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi3en {
    #[doc = "0: SPI3 clock disabled"]
    B0x0 = 0,
    #[doc = "1: SPI3 clock enabled"]
    B0x1 = 1,
}
impl From<Spi3en> for bool {
    #[inline(always)]
    fn from(variant: Spi3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI3EN` reader - SPI3 clock enable Set and cleared by software."]
pub type Spi3enR = crate::BitReader<Spi3en>;
impl Spi3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi3en {
        match self.bits {
            false => Spi3en::B0x0,
            true => Spi3en::B0x1,
        }
    }
    #[doc = "SPI3 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Spi3en::B0x0
    }
    #[doc = "SPI3 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Spi3en::B0x1
    }
}
#[doc = "Field `SPI3EN` writer - SPI3 clock enable Set and cleared by software."]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG, Spi3en>;
impl<'a, REG> Spi3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3en::B0x0)
    }
    #[doc = "SPI3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi3en::B0x1)
    }
}
#[doc = "USART2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart2en {
    #[doc = "0: USART2 clock disabled"]
    B0x0 = 0,
    #[doc = "1: USART2 clock enabled"]
    B0x1 = 1,
}
impl From<Usart2en> for bool {
    #[inline(always)]
    fn from(variant: Usart2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and cleared by software."]
pub type Usart2enR = crate::BitReader<Usart2en>;
impl Usart2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2en {
        match self.bits {
            false => Usart2en::B0x0,
            true => Usart2en::B0x1,
        }
    }
    #[doc = "USART2 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2en::B0x0
    }
    #[doc = "USART2 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2en::B0x1
    }
}
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and cleared by software."]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG, Usart2en>;
impl<'a, REG> Usart2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2en::B0x0)
    }
    #[doc = "USART2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2en::B0x1)
    }
}
#[doc = "USART3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart3en {
    #[doc = "0: USART3 clock disabled"]
    B0x0 = 0,
    #[doc = "1: USART3 clock enabled"]
    B0x1 = 1,
}
impl From<Usart3en> for bool {
    #[inline(always)]
    fn from(variant: Usart3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART3EN` reader - USART3 clock enable Set and cleared by software."]
pub type Usart3enR = crate::BitReader<Usart3en>;
impl Usart3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3en {
        match self.bits {
            false => Usart3en::B0x0,
            true => Usart3en::B0x1,
        }
    }
    #[doc = "USART3 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3en::B0x0
    }
    #[doc = "USART3 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3en::B0x1
    }
}
#[doc = "Field `USART3EN` writer - USART3 clock enable Set and cleared by software."]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG, Usart3en>;
impl<'a, REG> Usart3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3en::B0x0)
    }
    #[doc = "USART3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3en::B0x1)
    }
}
#[doc = "UART4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart4en {
    #[doc = "0: UART4 clock disabled"]
    B0x0 = 0,
    #[doc = "1: UART4 clock enabled"]
    B0x1 = 1,
}
impl From<Uart4en> for bool {
    #[inline(always)]
    fn from(variant: Uart4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4EN` reader - UART4 clock enable Set and cleared by software."]
pub type Uart4enR = crate::BitReader<Uart4en>;
impl Uart4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart4en {
        match self.bits {
            false => Uart4en::B0x0,
            true => Uart4en::B0x1,
        }
    }
    #[doc = "UART4 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart4en::B0x0
    }
    #[doc = "UART4 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart4en::B0x1
    }
}
#[doc = "Field `UART4EN` writer - UART4 clock enable Set and cleared by software."]
pub type Uart4enW<'a, REG> = crate::BitWriter<'a, REG, Uart4en>;
impl<'a, REG> Uart4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4en::B0x0)
    }
    #[doc = "UART4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4en::B0x1)
    }
}
#[doc = "UART5 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart5en {
    #[doc = "0: UART5 clock disabled"]
    B0x0 = 0,
    #[doc = "1: UART5 clock enabled"]
    B0x1 = 1,
}
impl From<Uart5en> for bool {
    #[inline(always)]
    fn from(variant: Uart5en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART5EN` reader - UART5 clock enable Set and cleared by software."]
pub type Uart5enR = crate::BitReader<Uart5en>;
impl Uart5enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart5en {
        match self.bits {
            false => Uart5en::B0x0,
            true => Uart5en::B0x1,
        }
    }
    #[doc = "UART5 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart5en::B0x0
    }
    #[doc = "UART5 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart5en::B0x1
    }
}
#[doc = "Field `UART5EN` writer - UART5 clock enable Set and cleared by software."]
pub type Uart5enW<'a, REG> = crate::BitWriter<'a, REG, Uart5en>;
impl<'a, REG> Uart5enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART5 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5en::B0x0)
    }
    #[doc = "UART5 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5en::B0x1)
    }
}
#[doc = "I2C1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1en {
    #[doc = "0: I2C1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: I2C1 clock enabled"]
    B0x1 = 1,
}
impl From<I2c1en> for bool {
    #[inline(always)]
    fn from(variant: I2c1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software."]
pub type I2c1enR = crate::BitReader<I2c1en>;
impl I2c1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1en {
        match self.bits {
            false => I2c1en::B0x0,
            true => I2c1en::B0x1,
        }
    }
    #[doc = "I2C1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1en::B0x0
    }
    #[doc = "I2C1 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1en::B0x1
    }
}
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software."]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG, I2c1en>;
impl<'a, REG> I2c1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1en::B0x0)
    }
    #[doc = "I2C1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1en::B0x1)
    }
}
#[doc = "I2C2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2en {
    #[doc = "0: I2C2 clock disabled"]
    B0x0 = 0,
    #[doc = "1: I2C2 clock enabled"]
    B0x1 = 1,
}
impl From<I2c2en> for bool {
    #[inline(always)]
    fn from(variant: I2c2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2EN` reader - I2C2 clock enable Set and cleared by software."]
pub type I2c2enR = crate::BitReader<I2c2en>;
impl I2c2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2en {
        match self.bits {
            false => I2c2en::B0x0,
            true => I2c2en::B0x1,
        }
    }
    #[doc = "I2C2 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2en::B0x0
    }
    #[doc = "I2C2 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2en::B0x1
    }
}
#[doc = "Field `I2C2EN` writer - I2C2 clock enable Set and cleared by software."]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG, I2c2en>;
impl<'a, REG> I2c2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2en::B0x0)
    }
    #[doc = "I2C2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2en::B0x1)
    }
}
#[doc = "USB device clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usben {
    #[doc = "0: USB device clock disabled"]
    B0x0 = 0,
    #[doc = "1: USB device clock enabled"]
    B0x1 = 1,
}
impl From<Usben> for bool {
    #[inline(always)]
    fn from(variant: Usben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEN` reader - USB device clock enable Set and cleared by software."]
pub type UsbenR = crate::BitReader<Usben>;
impl UsbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usben {
        match self.bits {
            false => Usben::B0x0,
            true => Usben::B0x1,
        }
    }
    #[doc = "USB device clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usben::B0x0
    }
    #[doc = "USB device clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usben::B0x1
    }
}
#[doc = "Field `USBEN` writer - USB device clock enable Set and cleared by software."]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG, Usben>;
impl<'a, REG> UsbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB device clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usben::B0x0)
    }
    #[doc = "USB device clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usben::B0x1)
    }
}
#[doc = "FDCAN clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdcanen {
    #[doc = "0: FDCAN clock disabled"]
    B0x0 = 0,
    #[doc = "1: FDCAN clock enabled"]
    B0x1 = 1,
}
impl From<Fdcanen> for bool {
    #[inline(always)]
    fn from(variant: Fdcanen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCANEN` reader - FDCAN clock enable Set and cleared by software."]
pub type FdcanenR = crate::BitReader<Fdcanen>;
impl FdcanenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdcanen {
        match self.bits {
            false => Fdcanen::B0x0,
            true => Fdcanen::B0x1,
        }
    }
    #[doc = "FDCAN clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fdcanen::B0x0
    }
    #[doc = "FDCAN clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fdcanen::B0x1
    }
}
#[doc = "Field `FDCANEN` writer - FDCAN clock enable Set and cleared by software."]
pub type FdcanenW<'a, REG> = crate::BitWriter<'a, REG, Fdcanen>;
impl<'a, REG> FdcanenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FDCAN clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcanen::B0x0)
    }
    #[doc = "FDCAN clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcanen::B0x1)
    }
}
#[doc = "Power interface clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwren {
    #[doc = "0: Power interface clock disabled"]
    B0x0 = 0,
    #[doc = "1: Power interface clock enabled"]
    B0x1 = 1,
}
impl From<Pwren> for bool {
    #[inline(always)]
    fn from(variant: Pwren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWREN` reader - Power interface clock enable Set and cleared by software."]
pub type PwrenR = crate::BitReader<Pwren>;
impl PwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwren {
        match self.bits {
            false => Pwren::B0x0,
            true => Pwren::B0x1,
        }
    }
    #[doc = "Power interface clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Pwren::B0x0
    }
    #[doc = "Power interface clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Pwren::B0x1
    }
}
#[doc = "Field `PWREN` writer - Power interface clock enable Set and cleared by software."]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG, Pwren>;
impl<'a, REG> PwrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power interface clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwren::B0x0)
    }
    #[doc = "Power interface clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwren::B0x1)
    }
}
#[doc = "I2C3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c3en {
    #[doc = "0: I2C3 clock disabled"]
    B0x0 = 0,
    #[doc = "1: I2C3 clock enabled"]
    B0x1 = 1,
}
impl From<I2c3en> for bool {
    #[inline(always)]
    fn from(variant: I2c3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3EN` reader - I2C3 clock enable Set and cleared by software."]
pub type I2c3enR = crate::BitReader<I2c3en>;
impl I2c3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c3en {
        match self.bits {
            false => I2c3en::B0x0,
            true => I2c3en::B0x1,
        }
    }
    #[doc = "I2C3 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3en::B0x0
    }
    #[doc = "I2C3 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3en::B0x1
    }
}
#[doc = "Field `I2C3EN` writer - I2C3 clock enable Set and cleared by software."]
pub type I2c3enW<'a, REG> = crate::BitWriter<'a, REG, I2c3en>;
impl<'a, REG> I2c3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3en::B0x0)
    }
    #[doc = "I2C3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3en::B0x1)
    }
}
#[doc = "Low power timer 1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptim1en {
    #[doc = "0: LPTIM1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: LPTIM1 clock enabled"]
    B0x1 = 1,
}
impl From<Lptim1en> for bool {
    #[inline(always)]
    fn from(variant: Lptim1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM1EN` reader - Low power timer 1 clock enable Set and cleared by software."]
pub type Lptim1enR = crate::BitReader<Lptim1en>;
impl Lptim1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1en {
        match self.bits {
            false => Lptim1en::B0x0,
            true => Lptim1en::B0x1,
        }
    }
    #[doc = "LPTIM1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1en::B0x0
    }
    #[doc = "LPTIM1 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1en::B0x1
    }
}
#[doc = "Field `LPTIM1EN` writer - Low power timer 1 clock enable Set and cleared by software."]
pub type Lptim1enW<'a, REG> = crate::BitWriter<'a, REG, Lptim1en>;
impl<'a, REG> Lptim1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1en::B0x0)
    }
    #[doc = "LPTIM1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1en::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim4en(&self) -> Tim4enR {
        Tim4enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim5en(&self) -> Tim5enR {
        Tim5enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim6en(&self) -> Tim6enR {
        Tim6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRS Recovery System clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn crsen(&self) -> CrsenR {
        CrsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RtcapbenR {
        RtcapbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WwdgenR {
        WwdgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn uart4en(&self) -> Uart4enR {
        Uart4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn uart5en(&self) -> Uart5enR {
        Uart5enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB device clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn fdcanen(&self) -> FdcanenR {
        FdcanenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2c3enR {
        I2c3enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> Lptim1enR {
        Lptim1enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1ENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim5en", &self.tim5en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("crsen", &self.crsen())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("uart4en", &self.uart4en())
            .field("uart5en", &self.uart5en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("usben", &self.usben())
            .field("fdcanen", &self.fdcanen())
            .field("pwren", &self.pwren())
            .field("i2c3en", &self.i2c3en())
            .field("lptim1en", &self.lptim1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim2en(&mut self) -> Tim2enW<RccApb1enr1Spec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim3en(&mut self) -> Tim3enW<RccApb1enr1Spec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim4en(&mut self) -> Tim4enW<RccApb1enr1Spec> {
        Tim4enW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim5en(&mut self) -> Tim5enW<RccApb1enr1Spec> {
        Tim5enW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim6en(&mut self) -> Tim6enW<RccApb1enr1Spec> {
        Tim6enW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim7en(&mut self) -> Tim7enW<RccApb1enr1Spec> {
        Tim7enW::new(self, 5)
    }
    #[doc = "Bit 8 - CRS Recovery System clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn crsen(&mut self) -> CrsenW<RccApb1enr1Spec> {
        CrsenW::new(self, 8)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RtcapbenW<RccApb1enr1Spec> {
        RtcapbenW::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WwdgenW<RccApb1enr1Spec> {
        WwdgenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<RccApb1enr1Spec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn spi3en(&mut self) -> Spi3enW<RccApb1enr1Spec> {
        Spi3enW::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<RccApb1enr1Spec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart3en(&mut self) -> Usart3enW<RccApb1enr1Spec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn uart4en(&mut self) -> Uart4enW<RccApb1enr1Spec> {
        Uart4enW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn uart5en(&mut self) -> Uart5enW<RccApb1enr1Spec> {
        Uart5enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<RccApb1enr1Spec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<RccApb1enr1Spec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23 - USB device clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usben(&mut self) -> UsbenW<RccApb1enr1Spec> {
        UsbenW::new(self, 23)
    }
    #[doc = "Bit 25 - FDCAN clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FdcanenW<RccApb1enr1Spec> {
        FdcanenW::new(self, 25)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn pwren(&mut self) -> PwrenW<RccApb1enr1Spec> {
        PwrenW::new(self, 28)
    }
    #[doc = "Bit 30 - I2C3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2c3enW<RccApb1enr1Spec> {
        I2c3enW::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> Lptim1enW<RccApb1enr1Spec> {
        Lptim1enW::new(self, 31)
    }
}
#[doc = "APB1 peripheral clock enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb1enr1Spec;
impl crate::RegisterSpec for RccApb1enr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1enr1::R`](R) reader structure"]
impl crate::Readable for RccApb1enr1Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1enr1::W`](W) writer structure"]
impl crate::Writable for RccApb1enr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB1ENR1 to value 0x0400"]
impl crate::Resettable for RccApb1enr1Spec {
    const RESET_VALUE: u32 = 0x0400;
}
