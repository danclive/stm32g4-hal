#[doc = "Register `RCC_AHB2ENR` reader"]
pub type R = crate::R<RccAhb2enrSpec>;
#[doc = "Register `RCC_AHB2ENR` writer"]
pub type W = crate::W<RccAhb2enrSpec>;
#[doc = "IO port A clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioaen {
    #[doc = "0: IO port A clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port A clock enabled"]
    B0x1 = 1,
}
impl From<Gpioaen> for bool {
    #[inline(always)]
    fn from(variant: Gpioaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable Set and cleared by software."]
pub type GpioaenR = crate::BitReader<Gpioaen>;
impl GpioaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioaen {
        match self.bits {
            false => Gpioaen::B0x0,
            true => Gpioaen::B0x1,
        }
    }
    #[doc = "IO port A clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioaen::B0x0
    }
    #[doc = "IO port A clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioaen::B0x1
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable Set and cleared by software."]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG, Gpioaen>;
impl<'a, REG> GpioaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port A clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioaen::B0x0)
    }
    #[doc = "IO port A clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioaen::B0x1)
    }
}
#[doc = "IO port B clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioben {
    #[doc = "0: IO port B clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port B clock enabled"]
    B0x1 = 1,
}
impl From<Gpioben> for bool {
    #[inline(always)]
    fn from(variant: Gpioben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable Set and cleared by software."]
pub type GpiobenR = crate::BitReader<Gpioben>;
impl GpiobenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioben {
        match self.bits {
            false => Gpioben::B0x0,
            true => Gpioben::B0x1,
        }
    }
    #[doc = "IO port B clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioben::B0x0
    }
    #[doc = "IO port B clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioben::B0x1
    }
}
#[doc = "Field `GPIOBEN` writer - IO port B clock enable Set and cleared by software."]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG, Gpioben>;
impl<'a, REG> GpiobenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port B clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioben::B0x0)
    }
    #[doc = "IO port B clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioben::B0x1)
    }
}
#[doc = "IO port C clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiocen {
    #[doc = "0: IO port C clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port C clock enabled"]
    B0x1 = 1,
}
impl From<Gpiocen> for bool {
    #[inline(always)]
    fn from(variant: Gpiocen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCEN` reader - IO port C clock enable Set and cleared by software."]
pub type GpiocenR = crate::BitReader<Gpiocen>;
impl GpiocenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiocen {
        match self.bits {
            false => Gpiocen::B0x0,
            true => Gpiocen::B0x1,
        }
    }
    #[doc = "IO port C clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiocen::B0x0
    }
    #[doc = "IO port C clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiocen::B0x1
    }
}
#[doc = "Field `GPIOCEN` writer - IO port C clock enable Set and cleared by software."]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG, Gpiocen>;
impl<'a, REG> GpiocenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port C clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocen::B0x0)
    }
    #[doc = "IO port C clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocen::B0x1)
    }
}
#[doc = "IO port D clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioden {
    #[doc = "0: IO port D clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port D clock enabled"]
    B0x1 = 1,
}
impl From<Gpioden> for bool {
    #[inline(always)]
    fn from(variant: Gpioden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODEN` reader - IO port D clock enable Set and cleared by software."]
pub type GpiodenR = crate::BitReader<Gpioden>;
impl GpiodenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioden {
        match self.bits {
            false => Gpioden::B0x0,
            true => Gpioden::B0x1,
        }
    }
    #[doc = "IO port D clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioden::B0x0
    }
    #[doc = "IO port D clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioden::B0x1
    }
}
#[doc = "Field `GPIODEN` writer - IO port D clock enable Set and cleared by software."]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG, Gpioden>;
impl<'a, REG> GpiodenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port D clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioden::B0x0)
    }
    #[doc = "IO port D clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioden::B0x1)
    }
}
#[doc = "IO port E clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioeen {
    #[doc = "0: IO port E clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port E clock enabled"]
    B0x1 = 1,
}
impl From<Gpioeen> for bool {
    #[inline(always)]
    fn from(variant: Gpioeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOEEN` reader - IO port E clock enable Set and cleared by software."]
pub type GpioeenR = crate::BitReader<Gpioeen>;
impl GpioeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioeen {
        match self.bits {
            false => Gpioeen::B0x0,
            true => Gpioeen::B0x1,
        }
    }
    #[doc = "IO port E clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioeen::B0x0
    }
    #[doc = "IO port E clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioeen::B0x1
    }
}
#[doc = "Field `GPIOEEN` writer - IO port E clock enable Set and cleared by software."]
pub type GpioeenW<'a, REG> = crate::BitWriter<'a, REG, Gpioeen>;
impl<'a, REG> GpioeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port E clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioeen::B0x0)
    }
    #[doc = "IO port E clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioeen::B0x1)
    }
}
#[doc = "IO port F clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiofen {
    #[doc = "0: IO port F clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port F clock enabled"]
    B0x1 = 1,
}
impl From<Gpiofen> for bool {
    #[inline(always)]
    fn from(variant: Gpiofen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOFEN` reader - IO port F clock enable Set and cleared by software."]
pub type GpiofenR = crate::BitReader<Gpiofen>;
impl GpiofenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiofen {
        match self.bits {
            false => Gpiofen::B0x0,
            true => Gpiofen::B0x1,
        }
    }
    #[doc = "IO port F clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiofen::B0x0
    }
    #[doc = "IO port F clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiofen::B0x1
    }
}
#[doc = "Field `GPIOFEN` writer - IO port F clock enable Set and cleared by software."]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG, Gpiofen>;
impl<'a, REG> GpiofenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port F clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofen::B0x0)
    }
    #[doc = "IO port F clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofen::B0x1)
    }
}
#[doc = "IO port G clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiogen {
    #[doc = "0: IO port G clock disabled"]
    B0x0 = 0,
    #[doc = "1: IO port G clock enabled"]
    B0x1 = 1,
}
impl From<Gpiogen> for bool {
    #[inline(always)]
    fn from(variant: Gpiogen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOGEN` reader - IO port G clock enable Set and cleared by software."]
pub type GpiogenR = crate::BitReader<Gpiogen>;
impl GpiogenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiogen {
        match self.bits {
            false => Gpiogen::B0x0,
            true => Gpiogen::B0x1,
        }
    }
    #[doc = "IO port G clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiogen::B0x0
    }
    #[doc = "IO port G clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiogen::B0x1
    }
}
#[doc = "Field `GPIOGEN` writer - IO port G clock enable Set and cleared by software."]
pub type GpiogenW<'a, REG> = crate::BitWriter<'a, REG, Gpiogen>;
impl<'a, REG> GpiogenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port G clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiogen::B0x0)
    }
    #[doc = "IO port G clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiogen::B0x1)
    }
}
#[doc = "ADC12 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12en {
    #[doc = "0: ADC12 clock disabled"]
    B0x0 = 0,
    #[doc = "1: ADC12 clock enabled"]
    B0x1 = 1,
}
impl From<Adc12en> for bool {
    #[inline(always)]
    fn from(variant: Adc12en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12EN` reader - ADC12 clock enable Set and cleared by software."]
pub type Adc12enR = crate::BitReader<Adc12en>;
impl Adc12enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12en {
        match self.bits {
            false => Adc12en::B0x0,
            true => Adc12en::B0x1,
        }
    }
    #[doc = "ADC12 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc12en::B0x0
    }
    #[doc = "ADC12 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc12en::B0x1
    }
}
#[doc = "Field `ADC12EN` writer - ADC12 clock enable Set and cleared by software."]
pub type Adc12enW<'a, REG> = crate::BitWriter<'a, REG, Adc12en>;
impl<'a, REG> Adc12enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC12 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12en::B0x0)
    }
    #[doc = "ADC12 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12en::B0x1)
    }
}
#[doc = "ADC345 clock enable Set and cleared by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc345en {
    #[doc = "0: ADC345 clock disabled"]
    B0x0 = 0,
    #[doc = "1: ADC345 clock enabled"]
    B0x1 = 1,
}
impl From<Adc345en> for bool {
    #[inline(always)]
    fn from(variant: Adc345en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC345EN` reader - ADC345 clock enable Set and cleared by software"]
pub type Adc345enR = crate::BitReader<Adc345en>;
impl Adc345enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc345en {
        match self.bits {
            false => Adc345en::B0x0,
            true => Adc345en::B0x1,
        }
    }
    #[doc = "ADC345 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc345en::B0x0
    }
    #[doc = "ADC345 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc345en::B0x1
    }
}
#[doc = "Field `ADC345EN` writer - ADC345 clock enable Set and cleared by software"]
pub type Adc345enW<'a, REG> = crate::BitWriter<'a, REG, Adc345en>;
impl<'a, REG> Adc345enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC345 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345en::B0x0)
    }
    #[doc = "ADC345 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345en::B0x1)
    }
}
#[doc = "DAC1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1en {
    #[doc = "0: DAC1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC1 clock enabled"]
    B0x1 = 1,
}
impl From<Dac1en> for bool {
    #[inline(always)]
    fn from(variant: Dac1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1EN` reader - DAC1 clock enable Set and cleared by software."]
pub type Dac1enR = crate::BitReader<Dac1en>;
impl Dac1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1en {
        match self.bits {
            false => Dac1en::B0x0,
            true => Dac1en::B0x1,
        }
    }
    #[doc = "DAC1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac1en::B0x0
    }
    #[doc = "DAC1 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac1en::B0x1
    }
}
#[doc = "Field `DAC1EN` writer - DAC1 clock enable Set and cleared by software."]
pub type Dac1enW<'a, REG> = crate::BitWriter<'a, REG, Dac1en>;
impl<'a, REG> Dac1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1en::B0x0)
    }
    #[doc = "DAC1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1en::B0x1)
    }
}
#[doc = "DAC2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac2en {
    #[doc = "0: DAC2 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC2 clock enabled"]
    B0x1 = 1,
}
impl From<Dac2en> for bool {
    #[inline(always)]
    fn from(variant: Dac2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2EN` reader - DAC2 clock enable Set and cleared by software."]
pub type Dac2enR = crate::BitReader<Dac2en>;
impl Dac2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac2en {
        match self.bits {
            false => Dac2en::B0x0,
            true => Dac2en::B0x1,
        }
    }
    #[doc = "DAC2 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac2en::B0x0
    }
    #[doc = "DAC2 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac2en::B0x1
    }
}
#[doc = "Field `DAC2EN` writer - DAC2 clock enable Set and cleared by software."]
pub type Dac2enW<'a, REG> = crate::BitWriter<'a, REG, Dac2en>;
impl<'a, REG> Dac2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac2en::B0x0)
    }
    #[doc = "DAC2 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac2en::B0x1)
    }
}
#[doc = "DAC3 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac3en {
    #[doc = "0: DAC3 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC3 clock enabled"]
    B0x1 = 1,
}
impl From<Dac3en> for bool {
    #[inline(always)]
    fn from(variant: Dac3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC3EN` reader - DAC3 clock enable Set and cleared by software."]
pub type Dac3enR = crate::BitReader<Dac3en>;
impl Dac3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac3en {
        match self.bits {
            false => Dac3en::B0x0,
            true => Dac3en::B0x1,
        }
    }
    #[doc = "DAC3 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac3en::B0x0
    }
    #[doc = "DAC3 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac3en::B0x1
    }
}
#[doc = "Field `DAC3EN` writer - DAC3 clock enable Set and cleared by software."]
pub type Dac3enW<'a, REG> = crate::BitWriter<'a, REG, Dac3en>;
impl<'a, REG> Dac3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac3en::B0x0)
    }
    #[doc = "DAC3 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac3en::B0x1)
    }
}
#[doc = "DAC4 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac4en {
    #[doc = "0: DAC4 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC4 clock enabled"]
    B0x1 = 1,
}
impl From<Dac4en> for bool {
    #[inline(always)]
    fn from(variant: Dac4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC4EN` reader - DAC4 clock enable Set and cleared by software."]
pub type Dac4enR = crate::BitReader<Dac4en>;
impl Dac4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac4en {
        match self.bits {
            false => Dac4en::B0x0,
            true => Dac4en::B0x1,
        }
    }
    #[doc = "DAC4 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac4en::B0x0
    }
    #[doc = "DAC4 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac4en::B0x1
    }
}
#[doc = "Field `DAC4EN` writer - DAC4 clock enable Set and cleared by software."]
pub type Dac4enW<'a, REG> = crate::BitWriter<'a, REG, Dac4en>;
impl<'a, REG> Dac4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac4en::B0x0)
    }
    #[doc = "DAC4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac4en::B0x1)
    }
}
#[doc = "AES clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesen {
    #[doc = "0: AES clock disabled"]
    B0x0 = 0,
    #[doc = "1: AES clock enabled"]
    B0x1 = 1,
}
impl From<Aesen> for bool {
    #[inline(always)]
    fn from(variant: Aesen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESEN` reader - AES clock enable Set and cleared by software."]
pub type AesenR = crate::BitReader<Aesen>;
impl AesenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesen {
        match self.bits {
            false => Aesen::B0x0,
            true => Aesen::B0x1,
        }
    }
    #[doc = "AES clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aesen::B0x0
    }
    #[doc = "AES clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aesen::B0x1
    }
}
#[doc = "Field `AESEN` writer - AES clock enable Set and cleared by software."]
pub type AesenW<'a, REG> = crate::BitWriter<'a, REG, Aesen>;
impl<'a, REG> AesenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesen::B0x0)
    }
    #[doc = "AES clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesen::B0x1)
    }
}
#[doc = "RNG enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rngen {
    #[doc = "0: RNG disabled"]
    B0x0 = 0,
    #[doc = "1: RNG enabled"]
    B0x1 = 1,
}
impl From<Rngen> for bool {
    #[inline(always)]
    fn from(variant: Rngen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - RNG enable Set and cleared by software."]
pub type RngenR = crate::BitReader<Rngen>;
impl RngenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rngen {
        match self.bits {
            false => Rngen::B0x0,
            true => Rngen::B0x1,
        }
    }
    #[doc = "RNG disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rngen::B0x0
    }
    #[doc = "RNG enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rngen::B0x1
    }
}
#[doc = "Field `RNGEN` writer - RNG enable Set and cleared by software."]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG, Rngen>;
impl<'a, REG> RngenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNG disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rngen::B0x0)
    }
    #[doc = "RNG enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rngen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - IO port A clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogen(&self) -> GpiogenR {
        GpiogenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adc12en(&self) -> Adc12enR {
        Adc12enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn adc345en(&self) -> Adc345enR {
        Adc345enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1en(&self) -> Dac1enR {
        Dac1enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac2en(&self) -> Dac2enR {
        Dac2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac3en(&self) -> Dac3enR {
        Dac3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac4en(&self) -> Dac4enR {
        Dac4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AES clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn aesen(&self) -> AesenR {
        AesenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB2ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("adc12en", &self.adc12en())
            .field("adc345en", &self.adc345en())
            .field("dac1en", &self.dac1en())
            .field("dac2en", &self.dac2en())
            .field("dac3en", &self.dac3en())
            .field("dac4en", &self.dac4en())
            .field("aesen", &self.aesen())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GpioaenW<RccAhb2enrSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GpiobenW<RccAhb2enrSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GpiocenW<RccAhb2enrSpec> {
        GpiocenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GpiodenW<RccAhb2enrSpec> {
        GpiodenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GpioeenW<RccAhb2enrSpec> {
        GpioeenW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GpiofenW<RccAhb2enrSpec> {
        GpiofenW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GpiogenW<RccAhb2enrSpec> {
        GpiogenW::new(self, 6)
    }
    #[doc = "Bit 13 - ADC12 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adc12en(&mut self) -> Adc12enW<RccAhb2enrSpec> {
        Adc12enW::new(self, 13)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn adc345en(&mut self) -> Adc345enW<RccAhb2enrSpec> {
        Adc345enW::new(self, 14)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1en(&mut self) -> Dac1enW<RccAhb2enrSpec> {
        Dac1enW::new(self, 16)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac2en(&mut self) -> Dac2enW<RccAhb2enrSpec> {
        Dac2enW::new(self, 17)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac3en(&mut self) -> Dac3enW<RccAhb2enrSpec> {
        Dac3enW::new(self, 18)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac4en(&mut self) -> Dac4enW<RccAhb2enrSpec> {
        Dac4enW::new(self, 19)
    }
    #[doc = "Bit 24 - AES clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn aesen(&mut self) -> AesenW<RccAhb2enrSpec> {
        AesenW::new(self, 24)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&mut self) -> RngenW<RccAhb2enrSpec> {
        RngenW::new(self, 26)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb2enrSpec;
impl crate::RegisterSpec for RccAhb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2enr::R`](R) reader structure"]
impl crate::Readable for RccAhb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2enr::W`](W) writer structure"]
impl crate::Writable for RccAhb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_AHB2ENR to value 0"]
impl crate::Resettable for RccAhb2enrSpec {}
