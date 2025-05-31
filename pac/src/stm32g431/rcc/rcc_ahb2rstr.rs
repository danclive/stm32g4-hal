#[doc = "Register `RCC_AHB2RSTR` reader"]
pub type R = crate::R<RccAhb2rstrSpec>;
#[doc = "Register `RCC_AHB2RSTR` writer"]
pub type W = crate::W<RccAhb2rstrSpec>;
#[doc = "IO port A reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioarst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port A"]
    B0x1 = 1,
}
impl From<Gpioarst> for bool {
    #[inline(always)]
    fn from(variant: Gpioarst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOARST` reader - IO port A reset Set and cleared by software."]
pub type GpioarstR = crate::BitReader<Gpioarst>;
impl GpioarstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioarst {
        match self.bits {
            false => Gpioarst::B0x0,
            true => Gpioarst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioarst::B0x0
    }
    #[doc = "Reset IO port A"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioarst::B0x1
    }
}
#[doc = "Field `GPIOARST` writer - IO port A reset Set and cleared by software."]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG, Gpioarst>;
impl<'a, REG> GpioarstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioarst::B0x0)
    }
    #[doc = "Reset IO port A"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioarst::B0x1)
    }
}
#[doc = "IO port B reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiobrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port B"]
    B0x1 = 1,
}
impl From<Gpiobrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiobrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOBRST` reader - IO port B reset Set and cleared by software."]
pub type GpiobrstR = crate::BitReader<Gpiobrst>;
impl GpiobrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiobrst {
        match self.bits {
            false => Gpiobrst::B0x0,
            true => Gpiobrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiobrst::B0x0
    }
    #[doc = "Reset IO port B"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiobrst::B0x1
    }
}
#[doc = "Field `GPIOBRST` writer - IO port B reset Set and cleared by software."]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiobrst>;
impl<'a, REG> GpiobrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobrst::B0x0)
    }
    #[doc = "Reset IO port B"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiobrst::B0x1)
    }
}
#[doc = "IO port C reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiocrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port C"]
    B0x1 = 1,
}
impl From<Gpiocrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiocrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOCRST` reader - IO port C reset Set and cleared by software."]
pub type GpiocrstR = crate::BitReader<Gpiocrst>;
impl GpiocrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiocrst {
        match self.bits {
            false => Gpiocrst::B0x0,
            true => Gpiocrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiocrst::B0x0
    }
    #[doc = "Reset IO port C"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiocrst::B0x1
    }
}
#[doc = "Field `GPIOCRST` writer - IO port C reset Set and cleared by software."]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiocrst>;
impl<'a, REG> GpiocrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocrst::B0x0)
    }
    #[doc = "Reset IO port C"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiocrst::B0x1)
    }
}
#[doc = "IO port D reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiodrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port D"]
    B0x1 = 1,
}
impl From<Gpiodrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiodrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIODRST` reader - IO port D reset Set and cleared by software."]
pub type GpiodrstR = crate::BitReader<Gpiodrst>;
impl GpiodrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiodrst {
        match self.bits {
            false => Gpiodrst::B0x0,
            true => Gpiodrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiodrst::B0x0
    }
    #[doc = "Reset IO port D"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiodrst::B0x1
    }
}
#[doc = "Field `GPIODRST` writer - IO port D reset Set and cleared by software."]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiodrst>;
impl<'a, REG> GpiodrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodrst::B0x0)
    }
    #[doc = "Reset IO port D"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiodrst::B0x1)
    }
}
#[doc = "IO port E reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioerst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port E"]
    B0x1 = 1,
}
impl From<Gpioerst> for bool {
    #[inline(always)]
    fn from(variant: Gpioerst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOERST` reader - IO port E reset Set and cleared by software."]
pub type GpioerstR = crate::BitReader<Gpioerst>;
impl GpioerstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioerst {
        match self.bits {
            false => Gpioerst::B0x0,
            true => Gpioerst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpioerst::B0x0
    }
    #[doc = "Reset IO port E"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpioerst::B0x1
    }
}
#[doc = "Field `GPIOERST` writer - IO port E reset Set and cleared by software."]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG, Gpioerst>;
impl<'a, REG> GpioerstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioerst::B0x0)
    }
    #[doc = "Reset IO port E"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioerst::B0x1)
    }
}
#[doc = "IO port F reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiofrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port F"]
    B0x1 = 1,
}
impl From<Gpiofrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiofrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOFRST` reader - IO port F reset Set and cleared by software."]
pub type GpiofrstR = crate::BitReader<Gpiofrst>;
impl GpiofrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiofrst {
        match self.bits {
            false => Gpiofrst::B0x0,
            true => Gpiofrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiofrst::B0x0
    }
    #[doc = "Reset IO port F"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiofrst::B0x1
    }
}
#[doc = "Field `GPIOFRST` writer - IO port F reset Set and cleared by software."]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiofrst>;
impl<'a, REG> GpiofrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofrst::B0x0)
    }
    #[doc = "Reset IO port F"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiofrst::B0x1)
    }
}
#[doc = "IO port G reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpiogrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset IO port G"]
    B0x1 = 1,
}
impl From<Gpiogrst> for bool {
    #[inline(always)]
    fn from(variant: Gpiogrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOGRST` reader - IO port G reset Set and cleared by software."]
pub type GpiogrstR = crate::BitReader<Gpiogrst>;
impl GpiogrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpiogrst {
        match self.bits {
            false => Gpiogrst::B0x0,
            true => Gpiogrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Gpiogrst::B0x0
    }
    #[doc = "Reset IO port G"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Gpiogrst::B0x1
    }
}
#[doc = "Field `GPIOGRST` writer - IO port G reset Set and cleared by software."]
pub type GpiogrstW<'a, REG> = crate::BitWriter<'a, REG, Gpiogrst>;
impl<'a, REG> GpiogrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiogrst::B0x0)
    }
    #[doc = "Reset IO port G"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpiogrst::B0x1)
    }
}
#[doc = "ADC12 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc12rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset ADC12 interface"]
    B0x1 = 1,
}
impl From<Adc12rst> for bool {
    #[inline(always)]
    fn from(variant: Adc12rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC12RST` reader - ADC12 reset Set and cleared by software."]
pub type Adc12rstR = crate::BitReader<Adc12rst>;
impl Adc12rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc12rst {
        match self.bits {
            false => Adc12rst::B0x0,
            true => Adc12rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc12rst::B0x0
    }
    #[doc = "Reset ADC12 interface"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc12rst::B0x1
    }
}
#[doc = "Field `ADC12RST` writer - ADC12 reset Set and cleared by software."]
pub type Adc12rstW<'a, REG> = crate::BitWriter<'a, REG, Adc12rst>;
impl<'a, REG> Adc12rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12rst::B0x0)
    }
    #[doc = "Reset ADC12 interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12rst::B0x1)
    }
}
#[doc = "ADC345 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc345rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset ADC345"]
    B0x1 = 1,
}
impl From<Adc345rst> for bool {
    #[inline(always)]
    fn from(variant: Adc345rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC345RST` reader - ADC345 reset Set and cleared by software."]
pub type Adc345rstR = crate::BitReader<Adc345rst>;
impl Adc345rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc345rst {
        match self.bits {
            false => Adc345rst::B0x0,
            true => Adc345rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc345rst::B0x0
    }
    #[doc = "Reset ADC345"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc345rst::B0x1
    }
}
#[doc = "Field `ADC345RST` writer - ADC345 reset Set and cleared by software."]
pub type Adc345rstW<'a, REG> = crate::BitWriter<'a, REG, Adc345rst>;
impl<'a, REG> Adc345rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345rst::B0x0)
    }
    #[doc = "Reset ADC345"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345rst::B0x1)
    }
}
#[doc = "DAC1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DAC1"]
    B0x1 = 1,
}
impl From<Dac1rst> for bool {
    #[inline(always)]
    fn from(variant: Dac1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1RST` reader - DAC1 reset Set and cleared by software."]
pub type Dac1rstR = crate::BitReader<Dac1rst>;
impl Dac1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1rst {
        match self.bits {
            false => Dac1rst::B0x0,
            true => Dac1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac1rst::B0x0
    }
    #[doc = "Reset DAC1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac1rst::B0x1
    }
}
#[doc = "Field `DAC1RST` writer - DAC1 reset Set and cleared by software."]
pub type Dac1rstW<'a, REG> = crate::BitWriter<'a, REG, Dac1rst>;
impl<'a, REG> Dac1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1rst::B0x0)
    }
    #[doc = "Reset DAC1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1rst::B0x1)
    }
}
#[doc = "DAC2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DAC2"]
    B0x1 = 1,
}
impl From<Dac2rst> for bool {
    #[inline(always)]
    fn from(variant: Dac2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2RST` reader - DAC2 reset Set and cleared by software."]
pub type Dac2rstR = crate::BitReader<Dac2rst>;
impl Dac2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac2rst {
        match self.bits {
            false => Dac2rst::B0x0,
            true => Dac2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac2rst::B0x0
    }
    #[doc = "Reset DAC2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac2rst::B0x1
    }
}
#[doc = "Field `DAC2RST` writer - DAC2 reset Set and cleared by software."]
pub type Dac2rstW<'a, REG> = crate::BitWriter<'a, REG, Dac2rst>;
impl<'a, REG> Dac2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac2rst::B0x0)
    }
    #[doc = "Reset DAC2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac2rst::B0x1)
    }
}
#[doc = "DAC3 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac3rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DAC3"]
    B0x1 = 1,
}
impl From<Dac3rst> for bool {
    #[inline(always)]
    fn from(variant: Dac3rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC3RST` reader - DAC3 reset Set and cleared by software."]
pub type Dac3rstR = crate::BitReader<Dac3rst>;
impl Dac3rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac3rst {
        match self.bits {
            false => Dac3rst::B0x0,
            true => Dac3rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac3rst::B0x0
    }
    #[doc = "Reset DAC3"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac3rst::B0x1
    }
}
#[doc = "Field `DAC3RST` writer - DAC3 reset Set and cleared by software."]
pub type Dac3rstW<'a, REG> = crate::BitWriter<'a, REG, Dac3rst>;
impl<'a, REG> Dac3rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac3rst::B0x0)
    }
    #[doc = "Reset DAC3"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac3rst::B0x1)
    }
}
#[doc = "DAC4 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DAC4"]
    B0x1 = 1,
}
impl From<Dac4rst> for bool {
    #[inline(always)]
    fn from(variant: Dac4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC4RST` reader - DAC4 reset Set and cleared by software."]
pub type Dac4rstR = crate::BitReader<Dac4rst>;
impl Dac4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac4rst {
        match self.bits {
            false => Dac4rst::B0x0,
            true => Dac4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dac4rst::B0x0
    }
    #[doc = "Reset DAC4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dac4rst::B0x1
    }
}
#[doc = "Field `DAC4RST` writer - DAC4 reset Set and cleared by software."]
pub type Dac4rstW<'a, REG> = crate::BitWriter<'a, REG, Dac4rst>;
impl<'a, REG> Dac4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac4rst::B0x0)
    }
    #[doc = "Reset DAC4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac4rst::B0x1)
    }
}
#[doc = "AESRST reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset AES"]
    B0x1 = 1,
}
impl From<Aesrst> for bool {
    #[inline(always)]
    fn from(variant: Aesrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRST` reader - AESRST reset Set and cleared by software."]
pub type AesrstR = crate::BitReader<Aesrst>;
impl AesrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesrst {
        match self.bits {
            false => Aesrst::B0x0,
            true => Aesrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aesrst::B0x0
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aesrst::B0x1
    }
}
#[doc = "Field `AESRST` writer - AESRST reset Set and cleared by software."]
pub type AesrstW<'a, REG> = crate::BitWriter<'a, REG, Aesrst>;
impl<'a, REG> AesrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrst::B0x0)
    }
    #[doc = "Reset AES"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrst::B0x1)
    }
}
#[doc = "RNG reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rngrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset RNG"]
    B0x1 = 1,
}
impl From<Rngrst> for bool {
    #[inline(always)]
    fn from(variant: Rngrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGRST` reader - RNG reset Set and cleared by software."]
pub type RngrstR = crate::BitReader<Rngrst>;
impl RngrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rngrst {
        match self.bits {
            false => Rngrst::B0x0,
            true => Rngrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Rngrst::B0x0
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Rngrst::B0x1
    }
}
#[doc = "Field `RNGRST` writer - RNG reset Set and cleared by software."]
pub type RngrstW<'a, REG> = crate::BitWriter<'a, REG, Rngrst>;
impl<'a, REG> RngrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Rngrst::B0x0)
    }
    #[doc = "Reset RNG"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Rngrst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - IO port A reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GpiogrstR {
        GpiogrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC12 reset Set and cleared by software."]
    #[inline(always)]
    pub fn adc12rst(&self) -> Adc12rstR {
        Adc12rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC345 reset Set and cleared by software."]
    #[inline(always)]
    pub fn adc345rst(&self) -> Adc345rstR {
        Adc345rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac1rst(&self) -> Dac1rstR {
        Dac1rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac2rst(&self) -> Dac2rstR {
        Dac2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac3rst(&self) -> Dac3rstR {
        Dac3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac4rst(&self) -> Dac4rstR {
        Dac4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AESRST reset Set and cleared by software."]
    #[inline(always)]
    pub fn aesrst(&self) -> AesrstR {
        AesrstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RNG reset Set and cleared by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RngrstR {
        RngrstR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB2RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("adc12rst", &self.adc12rst())
            .field("adc345rst", &self.adc345rst())
            .field("dac1rst", &self.dac1rst())
            .field("dac2rst", &self.dac2rst())
            .field("dac3rst", &self.dac3rst())
            .field("dac4rst", &self.dac4rst())
            .field("aesrst", &self.aesrst())
            .field("rngrst", &self.rngrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GpioarstW<RccAhb2rstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GpiobrstW<RccAhb2rstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GpiocrstW<RccAhb2rstrSpec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GpiodrstW<RccAhb2rstrSpec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GpioerstW<RccAhb2rstrSpec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GpiofrstW<RccAhb2rstrSpec> {
        GpiofrstW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G reset Set and cleared by software."]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GpiogrstW<RccAhb2rstrSpec> {
        GpiogrstW::new(self, 6)
    }
    #[doc = "Bit 13 - ADC12 reset Set and cleared by software."]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> Adc12rstW<RccAhb2rstrSpec> {
        Adc12rstW::new(self, 13)
    }
    #[doc = "Bit 14 - ADC345 reset Set and cleared by software."]
    #[inline(always)]
    pub fn adc345rst(&mut self) -> Adc345rstW<RccAhb2rstrSpec> {
        Adc345rstW::new(self, 14)
    }
    #[doc = "Bit 16 - DAC1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac1rst(&mut self) -> Dac1rstW<RccAhb2rstrSpec> {
        Dac1rstW::new(self, 16)
    }
    #[doc = "Bit 17 - DAC2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac2rst(&mut self) -> Dac2rstW<RccAhb2rstrSpec> {
        Dac2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - DAC3 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac3rst(&mut self) -> Dac3rstW<RccAhb2rstrSpec> {
        Dac3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - DAC4 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dac4rst(&mut self) -> Dac4rstW<RccAhb2rstrSpec> {
        Dac4rstW::new(self, 19)
    }
    #[doc = "Bit 24 - AESRST reset Set and cleared by software."]
    #[inline(always)]
    pub fn aesrst(&mut self) -> AesrstW<RccAhb2rstrSpec> {
        AesrstW::new(self, 24)
    }
    #[doc = "Bit 26 - RNG reset Set and cleared by software."]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RngrstW<RccAhb2rstrSpec> {
        RngrstW::new(self, 26)
    }
}
#[doc = "AHB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb2rstrSpec;
impl crate::RegisterSpec for RccAhb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb2rstr::R`](R) reader structure"]
impl crate::Readable for RccAhb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb2rstr::W`](W) writer structure"]
impl crate::Writable for RccAhb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_AHB2RSTR to value 0"]
impl crate::Resettable for RccAhb2rstrSpec {}
