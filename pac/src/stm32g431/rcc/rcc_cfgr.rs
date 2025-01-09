#[doc = "Register `RCC_CFGR` reader"]
pub type R = crate::R<RccCfgrSpec>;
#[doc = "Register `RCC_CFGR` writer"]
pub type W = crate::W<RccCfgrSpec>;
#[doc = "System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sw {
    #[doc = "0: Reserved, must be kept at reset value"]
    B0x0 = 0,
    #[doc = "1: HSI16 selected as system clock"]
    B0x1 = 1,
    #[doc = "2: HSE selected as system clock"]
    B0x2 = 2,
    #[doc = "3: PLL selected as system clock"]
    B0x3 = 3,
}
impl From<Sw> for u8 {
    #[inline(always)]
    fn from(variant: Sw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sw {
    type Ux = u8;
}
impl crate::IsEnum for Sw {}
#[doc = "Field `SW` reader - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
pub type SwR = crate::FieldReader<Sw>;
impl SwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sw {
        match self.bits {
            0 => Sw::B0x0,
            1 => Sw::B0x1,
            2 => Sw::B0x2,
            3 => Sw::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sw::B0x0
    }
    #[doc = "HSI16 selected as system clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sw::B0x1
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Sw::B0x2
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Sw::B0x3
    }
}
#[doc = "Field `SW` writer - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sw, crate::Safe>;
impl<'a, REG> SwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x0)
    }
    #[doc = "HSI16 selected as system clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x1)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x2)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sw::B0x3)
    }
}
#[doc = "System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sws {
    #[doc = "0: Reserved, must be kept at reset value"]
    B0x0 = 0,
    #[doc = "1: HSI16 oscillator used as system clock"]
    B0x1 = 1,
    #[doc = "2: HSE used as system clock"]
    B0x2 = 2,
    #[doc = "3: PLL used as system clock"]
    B0x3 = 3,
}
impl From<Sws> for u8 {
    #[inline(always)]
    fn from(variant: Sws) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sws {
    type Ux = u8;
}
impl crate::IsEnum for Sws {}
#[doc = "Field `SWS` reader - System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
pub type SwsR = crate::FieldReader<Sws>;
impl SwsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sws {
        match self.bits {
            0 => Sws::B0x0,
            1 => Sws::B0x1,
            2 => Sws::B0x2,
            3 => Sws::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sws::B0x0
    }
    #[doc = "HSI16 oscillator used as system clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sws::B0x1
    }
    #[doc = "HSE used as system clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Sws::B0x2
    }
    #[doc = "PLL used as system clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Sws::B0x3
    }
}
#[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hpre {
    #[doc = "8: SYSCLK divided by 2"]
    B0x8 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    B0x9 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    B0xA = 10,
    #[doc = "11: SYSCLK divided by 16"]
    B0xB = 11,
    #[doc = "12: SYSCLK divided by 64"]
    B0xC = 12,
    #[doc = "13: SYSCLK divided by 128"]
    B0xD = 13,
    #[doc = "14: SYSCLK divided by 256"]
    B0xE = 14,
    #[doc = "15: SYSCLK divided by 512"]
    B0xF = 15,
}
impl From<Hpre> for u8 {
    #[inline(always)]
    fn from(variant: Hpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hpre {
    type Ux = u8;
}
impl crate::IsEnum for Hpre {}
#[doc = "Field `HPRE` reader - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
pub type HpreR = crate::FieldReader<Hpre>;
impl HpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hpre> {
        match self.bits {
            8 => Some(Hpre::B0x8),
            9 => Some(Hpre::B0x9),
            10 => Some(Hpre::B0xA),
            11 => Some(Hpre::B0xB),
            12 => Some(Hpre::B0xC),
            13 => Some(Hpre::B0xD),
            14 => Some(Hpre::B0xE),
            15 => Some(Hpre::B0xF),
            _ => None,
        }
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Hpre::B0x8
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == Hpre::B0x9
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == Hpre::B0xA
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == Hpre::B0xB
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == Hpre::B0xC
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == Hpre::B0xD
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == Hpre::B0xE
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == Hpre::B0xF
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hpre>;
impl<'a, REG> HpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0x8)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0x9)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xA)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xB)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xC)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xD)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xE)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(Hpre::B0xF)
    }
}
#[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ppre1 {
    #[doc = "4: HCLK divided by 2"]
    B0x4 = 4,
    #[doc = "5: HCLK divided by 4"]
    B0x5 = 5,
    #[doc = "6: HCLK divided by 8"]
    B0x6 = 6,
    #[doc = "7: HCLK divided by 16"]
    B0x7 = 7,
}
impl From<Ppre1> for u8 {
    #[inline(always)]
    fn from(variant: Ppre1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ppre1 {
    type Ux = u8;
}
impl crate::IsEnum for Ppre1 {}
#[doc = "Field `PPRE1` reader - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
pub type Ppre1R = crate::FieldReader<Ppre1>;
impl Ppre1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ppre1> {
        match self.bits {
            4 => Some(Ppre1::B0x4),
            5 => Some(Ppre1::B0x5),
            6 => Some(Ppre1::B0x6),
            7 => Some(Ppre1::B0x7),
            _ => None,
        }
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ppre1::B0x4
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ppre1::B0x5
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ppre1::B0x6
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ppre1::B0x7
    }
}
#[doc = "Field `PPRE1` writer - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
pub type Ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Ppre1>;
impl<'a, REG> Ppre1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre1::B0x4)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre1::B0x5)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre1::B0x6)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre1::B0x7)
    }
}
#[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ppre2 {
    #[doc = "4: HCLK divided by 2"]
    B0x4 = 4,
    #[doc = "5: HCLK divided by 4"]
    B0x5 = 5,
    #[doc = "6: HCLK divided by 8"]
    B0x6 = 6,
    #[doc = "7: HCLK divided by 16"]
    B0x7 = 7,
}
impl From<Ppre2> for u8 {
    #[inline(always)]
    fn from(variant: Ppre2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ppre2 {
    type Ux = u8;
}
impl crate::IsEnum for Ppre2 {}
#[doc = "Field `PPRE2` reader - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
pub type Ppre2R = crate::FieldReader<Ppre2>;
impl Ppre2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ppre2> {
        match self.bits {
            4 => Some(Ppre2::B0x4),
            5 => Some(Ppre2::B0x5),
            6 => Some(Ppre2::B0x6),
            7 => Some(Ppre2::B0x7),
            _ => None,
        }
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Ppre2::B0x4
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Ppre2::B0x5
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Ppre2::B0x6
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Ppre2::B0x7
    }
}
#[doc = "Field `PPRE2` writer - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
pub type Ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3, Ppre2>;
impl<'a, REG> Ppre2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre2::B0x4)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre2::B0x5)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre2::B0x6)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Ppre2::B0x7)
    }
}
#[doc = "Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcosel {
    #[doc = "0: MCO output disabled, no clock on MCO"]
    B0x0 = 0,
    #[doc = "1: SYSCLK system clock selected"]
    B0x1 = 1,
    #[doc = "2: Reserved, must be kept at reset value"]
    B0x2 = 2,
    #[doc = "3: HSI16 clock selected"]
    B0x3 = 3,
    #[doc = "4: HSE clock selected"]
    B0x4 = 4,
    #[doc = "5: Main PLL clock selected"]
    B0x5 = 5,
    #[doc = "6: LSI clock selected"]
    B0x6 = 6,
    #[doc = "7: LSE clock selected"]
    B0x7 = 7,
    #[doc = "8: Internal HSI48 clock selected"]
    B0x8 = 8,
}
impl From<Mcosel> for u8 {
    #[inline(always)]
    fn from(variant: Mcosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcosel {
    type Ux = u8;
}
impl crate::IsEnum for Mcosel {}
#[doc = "Field `MCOSEL` reader - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
pub type McoselR = crate::FieldReader<Mcosel>;
impl McoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcosel> {
        match self.bits {
            0 => Some(Mcosel::B0x0),
            1 => Some(Mcosel::B0x1),
            2 => Some(Mcosel::B0x2),
            3 => Some(Mcosel::B0x3),
            4 => Some(Mcosel::B0x4),
            5 => Some(Mcosel::B0x5),
            6 => Some(Mcosel::B0x6),
            7 => Some(Mcosel::B0x7),
            8 => Some(Mcosel::B0x8),
            _ => None,
        }
    }
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mcosel::B0x0
    }
    #[doc = "SYSCLK system clock selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mcosel::B0x1
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mcosel::B0x2
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mcosel::B0x3
    }
    #[doc = "HSE clock selected"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mcosel::B0x4
    }
    #[doc = "Main PLL clock selected"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == Mcosel::B0x5
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == Mcosel::B0x6
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == Mcosel::B0x7
    }
    #[doc = "Internal HSI48 clock selected"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == Mcosel::B0x8
    }
}
#[doc = "Field `MCOSEL` writer - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mcosel>;
impl<'a, REG> McoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x0)
    }
    #[doc = "SYSCLK system clock selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x1)
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x2)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x3)
    }
    #[doc = "HSE clock selected"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x4)
    }
    #[doc = "Main PLL clock selected"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x5)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x6)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x7)
    }
    #[doc = "Internal HSI48 clock selected"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Mcosel::B0x8)
    }
}
#[doc = "Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mcopre {
    #[doc = "0: MCO is divided by 1"]
    B0x0 = 0,
    #[doc = "1: MCO is divided by 2"]
    B0x1 = 1,
    #[doc = "2: MCO is divided by 4"]
    B0x2 = 2,
    #[doc = "3: MCO is divided by 8"]
    B0x3 = 3,
    #[doc = "4: MCO is divided by 16"]
    B0x4 = 4,
}
impl From<Mcopre> for u8 {
    #[inline(always)]
    fn from(variant: Mcopre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mcopre {
    type Ux = u8;
}
impl crate::IsEnum for Mcopre {}
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
pub type McopreR = crate::FieldReader<Mcopre>;
impl McopreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mcopre> {
        match self.bits {
            0 => Some(Mcopre::B0x0),
            1 => Some(Mcopre::B0x1),
            2 => Some(Mcopre::B0x2),
            3 => Some(Mcopre::B0x3),
            4 => Some(Mcopre::B0x4),
            _ => None,
        }
    }
    #[doc = "MCO is divided by 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Mcopre::B0x0
    }
    #[doc = "MCO is divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Mcopre::B0x1
    }
    #[doc = "MCO is divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Mcopre::B0x2
    }
    #[doc = "MCO is divided by 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Mcopre::B0x3
    }
    #[doc = "MCO is divided by 16"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == Mcopre::B0x4
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
pub type McopreW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mcopre>;
impl<'a, REG> McopreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCO is divided by 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x0)
    }
    #[doc = "MCO is divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x1)
    }
    #[doc = "MCO is divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x2)
    }
    #[doc = "MCO is divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x3)
    }
    #[doc = "MCO is divided by 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Mcopre::B0x4)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre1(&self) -> Ppre1R {
        Ppre1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre2(&self) -> Ppre2R {
        Ppre2R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
    #[inline(always)]
    pub fn mcopre(&self) -> McopreR {
        McopreR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CFGR")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("hpre", &self.hpre())
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force HSI16 oscillator selection when exiting stop and standby modes or in case of failure of the HSE oscillator."]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<RccCfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler Set and cleared by software to control the division factor of the AHB clock. Note: Depending on the device voltage range, the software has to set correctly these bits to ensure that the system frequency does not exceed the maximum allowed frequency (for more details please refer to Section 6.1.5: Dynamic voltage scaling management). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value has been taken into account. 0xxx: SYSCLK not divided"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<RccCfgrSpec> {
        HpreW::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> Ppre1W<RccCfgrSpec> {
        Ppre1W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> Ppre2W<RccCfgrSpec> {
        Ppre2W::new(self, 11)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output Set and cleared by software. Others: Reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
    #[inline(always)]
    pub fn mcosel(&mut self) -> McoselW<RccCfgrSpec> {
        McoselW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler These bits are set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> McopreW<RccCfgrSpec> {
        McopreW::new(self, 28)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCfgrSpec;
impl crate::RegisterSpec for RccCfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cfgr::R`](R) reader structure"]
impl crate::Readable for RccCfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_cfgr::W`](W) writer structure"]
impl crate::Writable for RccCfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CFGR to value 0x05"]
impl crate::Resettable for RccCfgrSpec {
    const RESET_VALUE: u32 = 0x05;
}
