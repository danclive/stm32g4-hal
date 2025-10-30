#[doc = "Register `RCC_AHB2SMENR` reader"]
pub type R = crate::R<RccAhb2smenrSpec>;
#[doc = "Register `RCC_AHB2SMENR` writer"]
pub type W = crate::W<RccAhb2smenrSpec>;
#[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioasmen {
    #[doc = "0: IO port A clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port A clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpioasmen> for bool {
    #[inline(always)]
    fn from(variant: Gpioasmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOASMEN` reader - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpioasmenR = crate::BitReader<Gpioasmen>;
impl GpioasmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioasmen {
        match self.bits {
            false => Gpioasmen::B0x0,
            true => Gpioasmen::B0x1,
        }
    }
    #[doc = "IO port A clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioasmen::B0x0
    }
    #[doc = "IO port A clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioasmen::B0x1
    }
}
#[doc = "Field `GPIOASMEN` writer - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpioasmenW<'a, REG> = crate::BitWriter<'a, REG, Gpioasmen>;
impl<'a, REG> GpioasmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port A clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioasmen::B0x0)
    }
    #[doc = "IO port A clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioasmen::B0x1)
    }
}
#[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiobsmen {
    #[doc = "0: IO port B clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port B clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpiobsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiobsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBSMEN` reader - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiobsmenR = crate::BitReader<Gpiobsmen>;
impl GpiobsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiobsmen {
        match self.bits {
            false => Gpiobsmen::B0x0,
            true => Gpiobsmen::B0x1,
        }
    }
    #[doc = "IO port B clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiobsmen::B0x0
    }
    #[doc = "IO port B clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiobsmen::B0x1
    }
}
#[doc = "Field `GPIOBSMEN` writer - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiobsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiobsmen>;
impl<'a, REG> GpiobsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port B clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobsmen::B0x0)
    }
    #[doc = "IO port B clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobsmen::B0x1)
    }
}
#[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiocsmen {
    #[doc = "0: IO port C clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port C clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpiocsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiocsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCSMEN` reader - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiocsmenR = crate::BitReader<Gpiocsmen>;
impl GpiocsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiocsmen {
        match self.bits {
            false => Gpiocsmen::B0x0,
            true => Gpiocsmen::B0x1,
        }
    }
    #[doc = "IO port C clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiocsmen::B0x0
    }
    #[doc = "IO port C clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiocsmen::B0x1
    }
}
#[doc = "Field `GPIOCSMEN` writer - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiocsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiocsmen>;
impl<'a, REG> GpiocsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port C clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocsmen::B0x0)
    }
    #[doc = "IO port C clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocsmen::B0x1)
    }
}
#[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiodsmen {
    #[doc = "0: IO port D clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port D clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpiodsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiodsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODSMEN` reader - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiodsmenR = crate::BitReader<Gpiodsmen>;
impl GpiodsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiodsmen {
        match self.bits {
            false => Gpiodsmen::B0x0,
            true => Gpiodsmen::B0x1,
        }
    }
    #[doc = "IO port D clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiodsmen::B0x0
    }
    #[doc = "IO port D clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiodsmen::B0x1
    }
}
#[doc = "Field `GPIODSMEN` writer - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiodsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiodsmen>;
impl<'a, REG> GpiodsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port D clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodsmen::B0x0)
    }
    #[doc = "IO port D clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodsmen::B0x1)
    }
}
#[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioesmen {
    #[doc = "0: IO port E clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port E clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpioesmen> for bool {
    #[inline(always)]
    fn from(variant: Gpioesmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOESMEN` reader - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpioesmenR = crate::BitReader<Gpioesmen>;
impl GpioesmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioesmen {
        match self.bits {
            false => Gpioesmen::B0x0,
            true => Gpioesmen::B0x1,
        }
    }
    #[doc = "IO port E clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioesmen::B0x0
    }
    #[doc = "IO port E clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioesmen::B0x1
    }
}
#[doc = "Field `GPIOESMEN` writer - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpioesmenW<'a, REG> = crate::BitWriter<'a, REG, Gpioesmen>;
impl<'a, REG> GpioesmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port E clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioesmen::B0x0)
    }
    #[doc = "IO port E clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioesmen::B0x1)
    }
}
#[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiofsmen {
    #[doc = "0: IO port F clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port F clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpiofsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiofsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOFSMEN` reader - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiofsmenR = crate::BitReader<Gpiofsmen>;
impl GpiofsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiofsmen {
        match self.bits {
            false => Gpiofsmen::B0x0,
            true => Gpiofsmen::B0x1,
        }
    }
    #[doc = "IO port F clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiofsmen::B0x0
    }
    #[doc = "IO port F clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiofsmen::B0x1
    }
}
#[doc = "Field `GPIOFSMEN` writer - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiofsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiofsmen>;
impl<'a, REG> GpiofsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port F clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofsmen::B0x0)
    }
    #[doc = "IO port F clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofsmen::B0x1)
    }
}
#[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiogsmen {
    #[doc = "0: IO port G clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: IO port G clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Gpiogsmen> for bool {
    #[inline(always)]
    fn from(variant: Gpiogsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOGSMEN` reader - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiogsmenR = crate::BitReader<Gpiogsmen>;
impl GpiogsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiogsmen {
        match self.bits {
            false => Gpiogsmen::B0x0,
            true => Gpiogsmen::B0x1,
        }
    }
    #[doc = "IO port G clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiogsmen::B0x0
    }
    #[doc = "IO port G clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiogsmen::B0x1
    }
}
#[doc = "Field `GPIOGSMEN` writer - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type GpiogsmenW<'a, REG> = crate::BitWriter<'a, REG, Gpiogsmen>;
impl<'a, REG> GpiogsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO port G clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiogsmen::B0x0)
    }
    #[doc = "IO port G clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiogsmen::B0x1)
    }
}
#[doc = "CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccmsramsmen {
    #[doc = "0: CCM SRAM interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: CCM SRAM interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Ccmsramsmen> for bool {
    #[inline(always)]
    fn from(variant: Ccmsramsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCMSRAMSMEN` reader - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CcmsramsmenR = crate::BitReader<Ccmsramsmen>;
impl CcmsramsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccmsramsmen {
        match self.bits {
            false => Ccmsramsmen::B0x0,
            true => Ccmsramsmen::B0x1,
        }
    }
    #[doc = "CCM SRAM interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ccmsramsmen::B0x0
    }
    #[doc = "CCM SRAM interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ccmsramsmen::B0x1
    }
}
#[doc = "Field `CCMSRAMSMEN` writer - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CcmsramsmenW<'a, REG> = crate::BitWriter<'a, REG, Ccmsramsmen>;
impl<'a, REG> CcmsramsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCM SRAM interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccmsramsmen::B0x0)
    }
    #[doc = "CCM SRAM interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccmsramsmen::B0x1)
    }
}
#[doc = "SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram2smen {
    #[doc = "0: SRAM2 interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SRAM2 interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Sram2smen> for bool {
    #[inline(always)]
    fn from(variant: Sram2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2SMEN` reader - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Sram2smenR = crate::BitReader<Sram2smen>;
impl Sram2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram2smen {
        match self.bits {
            false => Sram2smen::B0x0,
            true => Sram2smen::B0x1,
        }
    }
    #[doc = "SRAM2 interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sram2smen::B0x0
    }
    #[doc = "SRAM2 interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sram2smen::B0x1
    }
}
#[doc = "Field `SRAM2SMEN` writer - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Sram2smenW<'a, REG> = crate::BitWriter<'a, REG, Sram2smen>;
impl<'a, REG> Sram2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 interface clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sram2smen::B0x0)
    }
    #[doc = "SRAM2 interface clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sram2smen::B0x1)
    }
}
#[doc = "ADC12 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12smen {
    #[doc = "0: ADC12 clocks disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: ADC12 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Adc12smen> for bool {
    #[inline(always)]
    fn from(variant: Adc12smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12SMEN` reader - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Adc12smenR = crate::BitReader<Adc12smen>;
impl Adc12smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12smen {
        match self.bits {
            false => Adc12smen::B0x0,
            true => Adc12smen::B0x1,
        }
    }
    #[doc = "ADC12 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc12smen::B0x0
    }
    #[doc = "ADC12 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc12smen::B0x1
    }
}
#[doc = "Field `ADC12SMEN` writer - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Adc12smenW<'a, REG> = crate::BitWriter<'a, REG, Adc12smen>;
impl<'a, REG> Adc12smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC12 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12smen::B0x0)
    }
    #[doc = "ADC12 clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12smen::B0x1)
    }
}
#[doc = "ADC345 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc345smen {
    #[doc = "0: ADC345 clock disabled"]
    B0x0 = 0,
    #[doc = "1: ADC345 clock enabled"]
    B0x1 = 1,
}
impl From<Adc345smen> for bool {
    #[inline(always)]
    fn from(variant: Adc345smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC345SMEN` reader - ADC345 clock enable Set and cleared by software."]
pub type Adc345smenR = crate::BitReader<Adc345smen>;
impl Adc345smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc345smen {
        match self.bits {
            false => Adc345smen::B0x0,
            true => Adc345smen::B0x1,
        }
    }
    #[doc = "ADC345 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc345smen::B0x0
    }
    #[doc = "ADC345 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc345smen::B0x1
    }
}
#[doc = "Field `ADC345SMEN` writer - ADC345 clock enable Set and cleared by software."]
pub type Adc345smenW<'a, REG> = crate::BitWriter<'a, REG, Adc345smen>;
impl<'a, REG> Adc345smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC345 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345smen::B0x0)
    }
    #[doc = "ADC345 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345smen::B0x1)
    }
}
#[doc = "DAC1 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1smen {
    #[doc = "0: DAC1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC1 clock enabled during sleep and stop modes"]
    B0x1 = 1,
}
impl From<Dac1smen> for bool {
    #[inline(always)]
    fn from(variant: Dac1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1SMEN` reader - DAC1 clock enable Set and cleared by software."]
pub type Dac1smenR = crate::BitReader<Dac1smen>;
impl Dac1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1smen {
        match self.bits {
            false => Dac1smen::B0x0,
            true => Dac1smen::B0x1,
        }
    }
    #[doc = "DAC1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac1smen::B0x0
    }
    #[doc = "DAC1 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac1smen::B0x1
    }
}
#[doc = "Field `DAC1SMEN` writer - DAC1 clock enable Set and cleared by software."]
pub type Dac1smenW<'a, REG> = crate::BitWriter<'a, REG, Dac1smen>;
impl<'a, REG> Dac1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1smen::B0x0)
    }
    #[doc = "DAC1 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1smen::B0x1)
    }
}
#[doc = "DAC2 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac2smen {
    #[doc = "0: DAC2 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC2 clock enabled during sleep and stop modes"]
    B0x1 = 1,
}
impl From<Dac2smen> for bool {
    #[inline(always)]
    fn from(variant: Dac2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2SMEN` reader - DAC2 clock enable Set and cleared by software."]
pub type Dac2smenR = crate::BitReader<Dac2smen>;
impl Dac2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac2smen {
        match self.bits {
            false => Dac2smen::B0x0,
            true => Dac2smen::B0x1,
        }
    }
    #[doc = "DAC2 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac2smen::B0x0
    }
    #[doc = "DAC2 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac2smen::B0x1
    }
}
#[doc = "Field `DAC2SMEN` writer - DAC2 clock enable Set and cleared by software."]
pub type Dac2smenW<'a, REG> = crate::BitWriter<'a, REG, Dac2smen>;
impl<'a, REG> Dac2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC2 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac2smen::B0x0)
    }
    #[doc = "DAC2 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac2smen::B0x1)
    }
}
#[doc = "DAC3 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac3smen {
    #[doc = "0: DAC3 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC3 clock enabled during sleep and stop modes"]
    B0x1 = 1,
}
impl From<Dac3smen> for bool {
    #[inline(always)]
    fn from(variant: Dac3smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC3SMEN` reader - DAC3 clock enable Set and cleared by software."]
pub type Dac3smenR = crate::BitReader<Dac3smen>;
impl Dac3smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac3smen {
        match self.bits {
            false => Dac3smen::B0x0,
            true => Dac3smen::B0x1,
        }
    }
    #[doc = "DAC3 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac3smen::B0x0
    }
    #[doc = "DAC3 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac3smen::B0x1
    }
}
#[doc = "Field `DAC3SMEN` writer - DAC3 clock enable Set and cleared by software."]
pub type Dac3smenW<'a, REG> = crate::BitWriter<'a, REG, Dac3smen>;
impl<'a, REG> Dac3smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC3 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac3smen::B0x0)
    }
    #[doc = "DAC3 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac3smen::B0x1)
    }
}
#[doc = "DAC4 clock enable Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac4smen {
    #[doc = "0: DAC4 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DAC4 clock enabled during sleep and stop modes"]
    B0x1 = 1,
}
impl From<Dac4smen> for bool {
    #[inline(always)]
    fn from(variant: Dac4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC4SMEN` reader - DAC4 clock enable Set and cleared by software."]
pub type Dac4smenR = crate::BitReader<Dac4smen>;
impl Dac4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac4smen {
        match self.bits {
            false => Dac4smen::B0x0,
            true => Dac4smen::B0x1,
        }
    }
    #[doc = "DAC4 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac4smen::B0x0
    }
    #[doc = "DAC4 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac4smen::B0x1
    }
}
#[doc = "Field `DAC4SMEN` writer - DAC4 clock enable Set and cleared by software."]
pub type Dac4smenW<'a, REG> = crate::BitWriter<'a, REG, Dac4smen>;
impl<'a, REG> Dac4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac4smen::B0x0)
    }
    #[doc = "DAC4 clock enabled during sleep and stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac4smen::B0x1)
    }
}
#[doc = "AESM clocks enable Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aessmen {
    #[doc = "0: AESM clocks disabled"]
    B0x0 = 0,
    #[doc = "1: AESM clocks enabled"]
    B0x1 = 1,
}
impl From<Aessmen> for bool {
    #[inline(always)]
    fn from(variant: Aessmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESSMEN` reader - AESM clocks enable Set and cleared by software."]
pub type AessmenR = crate::BitReader<Aessmen>;
impl AessmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aessmen {
        match self.bits {
            false => Aessmen::B0x0,
            true => Aessmen::B0x1,
        }
    }
    #[doc = "AESM clocks disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aessmen::B0x0
    }
    #[doc = "AESM clocks enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aessmen::B0x1
    }
}
#[doc = "Field `AESSMEN` writer - AESM clocks enable Set and cleared by software."]
pub type AessmenW<'a, REG> = crate::BitWriter<'a, REG, Aessmen>;
impl<'a, REG> AessmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AESM clocks disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aessmen::B0x0)
    }
    #[doc = "AESM clocks enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aessmen::B0x1)
    }
}
#[doc = "RNG enable Set and cleared by software.\n\nValue on reset: 1"]
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
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GpioasmenR {
        GpioasmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GpiobsmenR {
        GpiobsmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GpiocsmenR {
        GpiocsmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GpiodsmenR {
        GpiodsmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GpioesmenR {
        GpioesmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GpiofsmenR {
        GpiofsmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GpiogsmenR {
        GpiogsmenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn ccmsramsmen(&self) -> CcmsramsmenR {
        CcmsramsmenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sram2smen(&self) -> Sram2smenR {
        Sram2smenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn adc12smen(&self) -> Adc12smenR {
        Adc12smenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adc345smen(&self) -> Adc345smenR {
        Adc345smenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1smen(&self) -> Dac1smenR {
        Dac1smenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac2smen(&self) -> Dac2smenR {
        Dac2smenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac3smen(&self) -> Dac3smenR {
        Dac3smenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac4smen(&self) -> Dac4smenR {
        Dac4smenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AESM clocks enable Set and cleared by software."]
    #[inline(always)]
    pub fn aessmen(&self) -> AessmenR {
        AessmenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB2SMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .field("gpiogsmen", &self.gpiogsmen())
            .field("ccmsramsmen", &self.ccmsramsmen())
            .field("sram2smen", &self.sram2smen())
            .field("adc12smen", &self.adc12smen())
            .field("adc345smen", &self.adc345smen())
            .field("dac1smen", &self.dac1smen())
            .field("dac2smen", &self.dac2smen())
            .field("dac3smen", &self.dac3smen())
            .field("dac4smen", &self.dac4smen())
            .field("aessmen", &self.aessmen())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GpioasmenW<'_, RccAhb2smenrSpec> {
        GpioasmenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GpiobsmenW<'_, RccAhb2smenrSpec> {
        GpiobsmenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GpiocsmenW<'_, RccAhb2smenrSpec> {
        GpiocsmenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GpiodsmenW<'_, RccAhb2smenrSpec> {
        GpiodsmenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GpioesmenW<'_, RccAhb2smenrSpec> {
        GpioesmenW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GpiofsmenW<'_, RccAhb2smenrSpec> {
        GpiofsmenW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GpiogsmenW<'_, RccAhb2smenrSpec> {
        GpiogsmenW::new(self, 6)
    }
    #[doc = "Bit 9 - CCM SRAM interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn ccmsramsmen(&mut self) -> CcmsramsmenW<'_, RccAhb2smenrSpec> {
        CcmsramsmenW::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sram2smen(&mut self) -> Sram2smenW<'_, RccAhb2smenrSpec> {
        Sram2smenW::new(self, 10)
    }
    #[doc = "Bit 13 - ADC12 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn adc12smen(&mut self) -> Adc12smenW<'_, RccAhb2smenrSpec> {
        Adc12smenW::new(self, 13)
    }
    #[doc = "Bit 14 - ADC345 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn adc345smen(&mut self) -> Adc345smenW<'_, RccAhb2smenrSpec> {
        Adc345smenW::new(self, 14)
    }
    #[doc = "Bit 16 - DAC1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac1smen(&mut self) -> Dac1smenW<'_, RccAhb2smenrSpec> {
        Dac1smenW::new(self, 16)
    }
    #[doc = "Bit 17 - DAC2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac2smen(&mut self) -> Dac2smenW<'_, RccAhb2smenrSpec> {
        Dac2smenW::new(self, 17)
    }
    #[doc = "Bit 18 - DAC3 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac3smen(&mut self) -> Dac3smenW<'_, RccAhb2smenrSpec> {
        Dac3smenW::new(self, 18)
    }
    #[doc = "Bit 19 - DAC4 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dac4smen(&mut self) -> Dac4smenW<'_, RccAhb2smenrSpec> {
        Dac4smenW::new(self, 19)
    }
    #[doc = "Bit 24 - AESM clocks enable Set and cleared by software."]
    #[inline(always)]
    pub fn aessmen(&mut self) -> AessmenW<'_, RccAhb2smenrSpec> {
        AessmenW::new(self, 24)
    }
    #[doc = "Bit 26 - RNG enable Set and cleared by software."]
    #[inline(always)]
    pub fn rngen(&mut self) -> RngenW<'_, RccAhb2smenrSpec> {
        RngenW::new(self, 26)
    }
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb2smenrSpec;
impl crate::RegisterSpec for RccAhb2smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2smenr::R`](R) reader structure"]
impl crate::Readable for RccAhb2smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2smenr::W`](W) writer structure"]
impl crate::Writable for RccAhb2smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_AHB2SMENR to value 0x050f_667f"]
impl crate::Resettable for RccAhb2smenrSpec {
    const RESET_VALUE: u32 = 0x050f_667f;
}
