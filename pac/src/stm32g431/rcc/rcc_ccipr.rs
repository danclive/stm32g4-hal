#[doc = "Register `RCC_CCIPR` reader"]
pub type R = crate::R<RccCciprSpec>;
#[doc = "Register `RCC_CCIPR` writer"]
pub type W = crate::W<RccCciprSpec>;
#[doc = "USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart1sel {
    #[doc = "0: PCLK selected as USART1 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as USART1 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as USART1 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as USART1 clock"]
    B0x3 = 3,
}
impl From<Usart1sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart1sel {
    type Ux = u8;
}
impl crate::IsEnum for Usart1sel {}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
pub type Usart1selR = crate::FieldReader<Usart1sel>;
impl Usart1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1sel {
        match self.bits {
            0 => Usart1sel::B0x0,
            1 => Usart1sel::B0x1,
            2 => Usart1sel::B0x2,
            3 => Usart1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as USART1 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart1sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as USART1 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart1sel::B0x1
    }
    #[doc = "HSI16 clock selected as USART1 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Usart1sel::B0x2
    }
    #[doc = "LSE clock selected as USART1 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Usart1sel::B0x3
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
pub type Usart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usart1sel, crate::Safe>;
impl<'a, REG> Usart1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x1)
    }
    #[doc = "HSI16 clock selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x2)
    }
    #[doc = "LSE clock selected as USART1 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1sel::B0x3)
    }
}
#[doc = "USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart2sel {
    #[doc = "0: PCLK selected as USART2 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as USART2 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as USART2 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as USART2 clock"]
    B0x3 = 3,
}
impl From<Usart2sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart2sel {
    type Ux = u8;
}
impl crate::IsEnum for Usart2sel {}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
pub type Usart2selR = crate::FieldReader<Usart2sel>;
impl Usart2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart2sel {
        match self.bits {
            0 => Usart2sel::B0x0,
            1 => Usart2sel::B0x1,
            2 => Usart2sel::B0x2,
            3 => Usart2sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as USART2 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart2sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as USART2 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart2sel::B0x1
    }
    #[doc = "HSI16 clock selected as USART2 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Usart2sel::B0x2
    }
    #[doc = "LSE clock selected as USART2 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Usart2sel::B0x3
    }
}
#[doc = "Field `USART2SEL` writer - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
pub type Usart2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usart2sel, crate::Safe>;
impl<'a, REG> Usart2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x1)
    }
    #[doc = "HSI16 clock selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x2)
    }
    #[doc = "LSE clock selected as USART2 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Usart2sel::B0x3)
    }
}
#[doc = "USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart3sel {
    #[doc = "0: PCLK selected as USART3 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as USART3 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as USART3 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as USART3 clock"]
    B0x3 = 3,
}
impl From<Usart3sel> for u8 {
    #[inline(always)]
    fn from(variant: Usart3sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usart3sel {
    type Ux = u8;
}
impl crate::IsEnum for Usart3sel {}
#[doc = "Field `USART3SEL` reader - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
pub type Usart3selR = crate::FieldReader<Usart3sel>;
impl Usart3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart3sel {
        match self.bits {
            0 => Usart3sel::B0x0,
            1 => Usart3sel::B0x1,
            2 => Usart3sel::B0x2,
            3 => Usart3sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as USART3 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Usart3sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as USART3 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Usart3sel::B0x1
    }
    #[doc = "HSI16 clock selected as USART3 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Usart3sel::B0x2
    }
    #[doc = "LSE clock selected as USART3 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Usart3sel::B0x3
    }
}
#[doc = "Field `USART3SEL` writer - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
pub type Usart3selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usart3sel, crate::Safe>;
impl<'a, REG> Usart3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3sel::B0x1)
    }
    #[doc = "HSI16 clock selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3sel::B0x2)
    }
    #[doc = "LSE clock selected as USART3 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Usart3sel::B0x3)
    }
}
#[doc = "UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart4sel {
    #[doc = "0: PCLK selected as UART4 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as UART4 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as UART4 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as UART4 clock"]
    B0x3 = 3,
}
impl From<Uart4sel> for u8 {
    #[inline(always)]
    fn from(variant: Uart4sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart4sel {
    type Ux = u8;
}
impl crate::IsEnum for Uart4sel {}
#[doc = "Field `UART4SEL` reader - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
pub type Uart4selR = crate::FieldReader<Uart4sel>;
impl Uart4selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart4sel {
        match self.bits {
            0 => Uart4sel::B0x0,
            1 => Uart4sel::B0x1,
            2 => Uart4sel::B0x2,
            3 => Uart4sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as UART4 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart4sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as UART4 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart4sel::B0x1
    }
    #[doc = "HSI16 clock selected as UART4 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Uart4sel::B0x2
    }
    #[doc = "LSE clock selected as UART4 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Uart4sel::B0x3
    }
}
#[doc = "Field `UART4SEL` writer - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
pub type Uart4selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart4sel, crate::Safe>;
impl<'a, REG> Uart4selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4sel::B0x1)
    }
    #[doc = "HSI16 clock selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4sel::B0x2)
    }
    #[doc = "LSE clock selected as UART4 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4sel::B0x3)
    }
}
#[doc = "UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart5sel {
    #[doc = "0: PCLK selected as UART5 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as UART5 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as UART5 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as UART5 clock"]
    B0x3 = 3,
}
impl From<Uart5sel> for u8 {
    #[inline(always)]
    fn from(variant: Uart5sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart5sel {
    type Ux = u8;
}
impl crate::IsEnum for Uart5sel {}
#[doc = "Field `UART5SEL` reader - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
pub type Uart5selR = crate::FieldReader<Uart5sel>;
impl Uart5selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart5sel {
        match self.bits {
            0 => Uart5sel::B0x0,
            1 => Uart5sel::B0x1,
            2 => Uart5sel::B0x2,
            3 => Uart5sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as UART5 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Uart5sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as UART5 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Uart5sel::B0x1
    }
    #[doc = "HSI16 clock selected as UART5 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Uart5sel::B0x2
    }
    #[doc = "LSE clock selected as UART5 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Uart5sel::B0x3
    }
}
#[doc = "Field `UART5SEL` writer - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
pub type Uart5selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart5sel, crate::Safe>;
impl<'a, REG> Uart5selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5sel::B0x1)
    }
    #[doc = "HSI16 clock selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5sel::B0x2)
    }
    #[doc = "LSE clock selected as UART5 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Uart5sel::B0x3)
    }
}
#[doc = "LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart1sel {
    #[doc = "0: PCLK selected as LPUART1 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as LPUART1 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as LPUART1 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as LPUART1 clock"]
    B0x3 = 3,
}
impl From<Lpuart1sel> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart1sel {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart1sel {}
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
pub type Lpuart1selR = crate::FieldReader<Lpuart1sel>;
impl Lpuart1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1sel {
        match self.bits {
            0 => Lpuart1sel::B0x0,
            1 => Lpuart1sel::B0x1,
            2 => Lpuart1sel::B0x2,
            3 => Lpuart1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as LPUART1 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as LPUART1 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1sel::B0x1
    }
    #[doc = "HSI16 clock selected as LPUART1 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lpuart1sel::B0x2
    }
    #[doc = "LSE clock selected as LPUART1 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lpuart1sel::B0x3
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
pub type Lpuart1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart1sel, crate::Safe>;
impl<'a, REG> Lpuart1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x1)
    }
    #[doc = "HSI16 clock selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x2)
    }
    #[doc = "LSE clock selected as LPUART1 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1sel::B0x3)
    }
}
#[doc = "I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c1sel {
    #[doc = "0: PCLK selected as I2C1 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C1 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as I2C1 clock"]
    B0x2 = 2,
}
impl From<I2c1sel> for u8 {
    #[inline(always)]
    fn from(variant: I2c1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2c1sel {
    type Ux = u8;
}
impl crate::IsEnum for I2c1sel {}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
pub type I2c1selR = crate::FieldReader<I2c1sel>;
impl I2c1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c1sel> {
        match self.bits {
            0 => Some(I2c1sel::B0x0),
            1 => Some(I2c1sel::B0x1),
            2 => Some(I2c1sel::B0x2),
            _ => None,
        }
    }
    #[doc = "PCLK selected as I2C1 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c1sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as I2C1 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c1sel::B0x1
    }
    #[doc = "HSI16 clock selected as I2C1 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2c1sel::B0x2
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
pub type I2c1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2c1sel>;
impl<'a, REG> I2c1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as I2C1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1sel::B0x1)
    }
    #[doc = "HSI16 clock selected as I2C1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1sel::B0x2)
    }
}
#[doc = "I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c2sel {
    #[doc = "0: PCLK selected as I2C2 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C2 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as I2C2 clock"]
    B0x2 = 2,
}
impl From<I2c2sel> for u8 {
    #[inline(always)]
    fn from(variant: I2c2sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2c2sel {
    type Ux = u8;
}
impl crate::IsEnum for I2c2sel {}
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
pub type I2c2selR = crate::FieldReader<I2c2sel>;
impl I2c2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c2sel> {
        match self.bits {
            0 => Some(I2c2sel::B0x0),
            1 => Some(I2c2sel::B0x1),
            2 => Some(I2c2sel::B0x2),
            _ => None,
        }
    }
    #[doc = "PCLK selected as I2C2 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c2sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as I2C2 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c2sel::B0x1
    }
    #[doc = "HSI16 clock selected as I2C2 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2c2sel::B0x2
    }
}
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
pub type I2c2selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2c2sel>;
impl<'a, REG> I2c2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as I2C2 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C2 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2sel::B0x1)
    }
    #[doc = "HSI16 clock selected as I2C2 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2sel::B0x2)
    }
}
#[doc = "I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c3sel {
    #[doc = "0: PCLK selected as I2C3 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C3 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as I2C3 clock"]
    B0x2 = 2,
}
impl From<I2c3sel> for u8 {
    #[inline(always)]
    fn from(variant: I2c3sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2c3sel {
    type Ux = u8;
}
impl crate::IsEnum for I2c3sel {}
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
pub type I2c3selR = crate::FieldReader<I2c3sel>;
impl I2c3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c3sel> {
        match self.bits {
            0 => Some(I2c3sel::B0x0),
            1 => Some(I2c3sel::B0x1),
            2 => Some(I2c3sel::B0x2),
            _ => None,
        }
    }
    #[doc = "PCLK selected as I2C3 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c3sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as I2C3 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c3sel::B0x1
    }
    #[doc = "HSI16 clock selected as I2C3 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2c3sel::B0x2
    }
}
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
pub type I2c3selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2c3sel>;
impl<'a, REG> I2c3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as I2C3 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C3 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3sel::B0x1)
    }
    #[doc = "HSI16 clock selected as I2C3 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2c3sel::B0x2)
    }
}
#[doc = "Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lptim1sel {
    #[doc = "0: PCLK selected as LPTIM1 clock"]
    B0x0 = 0,
    #[doc = "1: LSI clock selected as LPTIM1 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as LPTIM1 clock"]
    B0x2 = 2,
    #[doc = "3: LSE clock selected as LPTIM1 clock"]
    B0x3 = 3,
}
impl From<Lptim1sel> for u8 {
    #[inline(always)]
    fn from(variant: Lptim1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lptim1sel {
    type Ux = u8;
}
impl crate::IsEnum for Lptim1sel {}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
pub type Lptim1selR = crate::FieldReader<Lptim1sel>;
impl Lptim1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptim1sel {
        match self.bits {
            0 => Lptim1sel::B0x0,
            1 => Lptim1sel::B0x1,
            2 => Lptim1sel::B0x2,
            3 => Lptim1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lptim1sel::B0x0
    }
    #[doc = "LSI clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lptim1sel::B0x1
    }
    #[doc = "HSI16 clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Lptim1sel::B0x2
    }
    #[doc = "LSE clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Lptim1sel::B0x3
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
pub type Lptim1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lptim1sel, crate::Safe>;
impl<'a, REG> Lptim1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x0)
    }
    #[doc = "LSI clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x1)
    }
    #[doc = "HSI16 clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x2)
    }
    #[doc = "LSE clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Lptim1sel::B0x3)
    }
}
#[doc = "clock source selection These bits are set and cleared by software to select the SAI clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sai1sel {
    #[doc = "0: System clock selected as SAI clock"]
    B0x0 = 0,
    #[doc = "1: PLL Q clock selected as SAI clock"]
    B0x1 = 1,
    #[doc = "2: Clock provided on I2S_CKIN pin selected as SAI clock"]
    B0x2 = 2,
    #[doc = "3: HSI16 clock selected as SAI clock"]
    B0x3 = 3,
}
impl From<Sai1sel> for u8 {
    #[inline(always)]
    fn from(variant: Sai1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sai1sel {
    type Ux = u8;
}
impl crate::IsEnum for Sai1sel {}
#[doc = "Field `SAI1SEL` reader - clock source selection These bits are set and cleared by software to select the SAI clock source."]
pub type Sai1selR = crate::FieldReader<Sai1sel>;
impl Sai1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sai1sel {
        match self.bits {
            0 => Sai1sel::B0x0,
            1 => Sai1sel::B0x1,
            2 => Sai1sel::B0x2,
            3 => Sai1sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "System clock selected as SAI clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sai1sel::B0x0
    }
    #[doc = "PLL Q clock selected as SAI clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sai1sel::B0x1
    }
    #[doc = "Clock provided on I2S_CKIN pin selected as SAI clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Sai1sel::B0x2
    }
    #[doc = "HSI16 clock selected as SAI clock"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Sai1sel::B0x3
    }
}
#[doc = "Field `SAI1SEL` writer - clock source selection These bits are set and cleared by software to select the SAI clock source."]
pub type Sai1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sai1sel, crate::Safe>;
impl<'a, REG> Sai1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1sel::B0x0)
    }
    #[doc = "PLL Q clock selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1sel::B0x1)
    }
    #[doc = "Clock provided on I2S_CKIN pin selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1sel::B0x2)
    }
    #[doc = "HSI16 clock selected as SAI clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Sai1sel::B0x3)
    }
}
#[doc = "clock source selection These bits are set and cleared by software to select the I2S23 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2s23sel {
    #[doc = "0: System clock selected as I2S23 clock"]
    B0x0 = 0,
    #[doc = "1: PLL Q clock selected as I2S23 clock"]
    B0x1 = 1,
    #[doc = "2: Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    B0x2 = 2,
    #[doc = "3: HSI16 clock selected as I2S23 clock."]
    B0x3 = 3,
}
impl From<I2s23sel> for u8 {
    #[inline(always)]
    fn from(variant: I2s23sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2s23sel {
    type Ux = u8;
}
impl crate::IsEnum for I2s23sel {}
#[doc = "Field `I2S23SEL` reader - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
pub type I2s23selR = crate::FieldReader<I2s23sel>;
impl I2s23selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2s23sel {
        match self.bits {
            0 => I2s23sel::B0x0,
            1 => I2s23sel::B0x1,
            2 => I2s23sel::B0x2,
            3 => I2s23sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "System clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2s23sel::B0x0
    }
    #[doc = "PLL Q clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2s23sel::B0x1
    }
    #[doc = "Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2s23sel::B0x2
    }
    #[doc = "HSI16 clock selected as I2S23 clock."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2s23sel::B0x3
    }
}
#[doc = "Field `I2S23SEL` writer - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
pub type I2s23selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2s23sel, crate::Safe>;
impl<'a, REG> I2s23selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2s23sel::B0x0)
    }
    #[doc = "PLL Q clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2s23sel::B0x1)
    }
    #[doc = "Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2s23sel::B0x2)
    }
    #[doc = "HSI16 clock selected as I2S23 clock."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2s23sel::B0x3)
    }
}
#[doc = "Field `FDCANSEL` reader - None"]
pub type FdcanselR = crate::FieldReader;
#[doc = "Field `FDCANSEL` writer - None"]
pub type FdcanselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clk48sel {
    #[doc = "0: HSI48 clock selected as 48 MHz clock"]
    B0x0 = 0,
    #[doc = "2: PLL Q clock (PLL48M1CLK) selected as 48 MHz clock"]
    B0x2 = 2,
    #[doc = "3: Reserved, must be kept at reset value"]
    B0x3 = 3,
}
impl From<Clk48sel> for u8 {
    #[inline(always)]
    fn from(variant: Clk48sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clk48sel {
    type Ux = u8;
}
impl crate::IsEnum for Clk48sel {}
#[doc = "Field `CLK48SEL` reader - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
pub type Clk48selR = crate::FieldReader<Clk48sel>;
impl Clk48selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clk48sel> {
        match self.bits {
            0 => Some(Clk48sel::B0x0),
            2 => Some(Clk48sel::B0x2),
            3 => Some(Clk48sel::B0x3),
            _ => None,
        }
    }
    #[doc = "HSI48 clock selected as 48 MHz clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Clk48sel::B0x0
    }
    #[doc = "PLL Q clock (PLL48M1CLK) selected as 48 MHz clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Clk48sel::B0x2
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Clk48sel::B0x3
    }
}
#[doc = "Field `CLK48SEL` writer - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
pub type Clk48selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clk48sel>;
impl<'a, REG> Clk48selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI48 clock selected as 48 MHz clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x0)
    }
    #[doc = "PLL Q clock (PLL48M1CLK) selected as 48 MHz clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x2)
    }
    #[doc = "Reserved, must be kept at reset value"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Clk48sel::B0x3)
    }
}
#[doc = "ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc12sel {
    #[doc = "0: No clock selected"]
    B0x0 = 0,
    #[doc = "1: PLL P clock selected as ADC1/2 clock"]
    B0x1 = 1,
    #[doc = "2: System clock selected as ADC1/2 clock"]
    B0x2 = 2,
}
impl From<Adc12sel> for u8 {
    #[inline(always)]
    fn from(variant: Adc12sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc12sel {
    type Ux = u8;
}
impl crate::IsEnum for Adc12sel {}
#[doc = "Field `ADC12SEL` reader - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
pub type Adc12selR = crate::FieldReader<Adc12sel>;
impl Adc12selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc12sel> {
        match self.bits {
            0 => Some(Adc12sel::B0x0),
            1 => Some(Adc12sel::B0x1),
            2 => Some(Adc12sel::B0x2),
            _ => None,
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc12sel::B0x0
    }
    #[doc = "PLL P clock selected as ADC1/2 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc12sel::B0x1
    }
    #[doc = "System clock selected as ADC1/2 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Adc12sel::B0x2
    }
}
#[doc = "Field `ADC12SEL` writer - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
pub type Adc12selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc12sel>;
impl<'a, REG> Adc12selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sel::B0x0)
    }
    #[doc = "PLL P clock selected as ADC1/2 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sel::B0x1)
    }
    #[doc = "System clock selected as ADC1/2 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc12sel::B0x2)
    }
}
#[doc = "ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc345sel {
    #[doc = "0: No clock selected"]
    B0x0 = 0,
    #[doc = "1: PLL P clock selected as ADC345 clock"]
    B0x1 = 1,
    #[doc = "2: System clock selected as ADC3/4/5 clock"]
    B0x2 = 2,
    #[doc = "3: Reserved."]
    B0x3 = 3,
}
impl From<Adc345sel> for u8 {
    #[inline(always)]
    fn from(variant: Adc345sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc345sel {
    type Ux = u8;
}
impl crate::IsEnum for Adc345sel {}
#[doc = "Field `ADC345SEL` reader - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
pub type Adc345selR = crate::FieldReader<Adc345sel>;
impl Adc345selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc345sel {
        match self.bits {
            0 => Adc345sel::B0x0,
            1 => Adc345sel::B0x1,
            2 => Adc345sel::B0x2,
            3 => Adc345sel::B0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Adc345sel::B0x0
    }
    #[doc = "PLL P clock selected as ADC345 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Adc345sel::B0x1
    }
    #[doc = "System clock selected as ADC3/4/5 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Adc345sel::B0x2
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == Adc345sel::B0x3
    }
}
#[doc = "Field `ADC345SEL` writer - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
pub type Adc345selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc345sel, crate::Safe>;
impl<'a, REG> Adc345selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345sel::B0x0)
    }
    #[doc = "PLL P clock selected as ADC345 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345sel::B0x1)
    }
    #[doc = "System clock selected as ADC3/4/5 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345sel::B0x2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc345sel::B0x3)
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
    #[inline(always)]
    pub fn usart1sel(&self) -> Usart1selR {
        Usart1selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
    #[inline(always)]
    pub fn usart2sel(&self) -> Usart2selR {
        Usart2selR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
    #[inline(always)]
    pub fn usart3sel(&self) -> Usart3selR {
        Usart3selR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
    #[inline(always)]
    pub fn uart4sel(&self) -> Uart4selR {
        Uart4selR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
    #[inline(always)]
    pub fn uart5sel(&self) -> Uart5selR {
        Uart5selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> Lpuart1selR {
        Lpuart1selR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2c1selR {
        I2c1selR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2c2selR {
        I2c2selR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2c3selR {
        I2c3selR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
    #[inline(always)]
    pub fn lptim1sel(&self) -> Lptim1selR {
        Lptim1selR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - clock source selection These bits are set and cleared by software to select the SAI clock source."]
    #[inline(always)]
    pub fn sai1sel(&self) -> Sai1selR {
        Sai1selR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
    #[inline(always)]
    pub fn i2s23sel(&self) -> I2s23selR {
        I2s23selR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - None"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FdcanselR {
        FdcanselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
    #[inline(always)]
    pub fn clk48sel(&self) -> Clk48selR {
        Clk48selR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
    #[inline(always)]
    pub fn adc12sel(&self) -> Adc12selR {
        Adc12selR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
    #[inline(always)]
    pub fn adc345sel(&self) -> Adc345selR {
        Adc345selR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCIPR")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("uart4sel", &self.uart4sel())
            .field("uart5sel", &self.uart5sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("sai1sel", &self.sai1sel())
            .field("i2s23sel", &self.i2s23sel())
            .field("fdcansel", &self.fdcansel())
            .field("clk48sel", &self.clk48sel())
            .field("adc12sel", &self.adc12sel())
            .field("adc345sel", &self.adc345sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection This bit is set and cleared by software to select the USART1 clock source."]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> Usart1selW<RccCciprSpec> {
        Usart1selW::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection This bit is set and cleared by software to select the USART2 clock source."]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> Usart2selW<RccCciprSpec> {
        Usart2selW::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection This bit is set and cleared by software to select the USART3 clock source."]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> Usart3selW<RccCciprSpec> {
        Usart3selW::new(self, 4)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection This bit is set and cleared by software to select the UART4 clock source."]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> Uart4selW<RccCciprSpec> {
        Uart4selW::new(self, 6)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection These bits are set and cleared by software to select the UART5 clock source."]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> Uart5selW<RccCciprSpec> {
        Uart5selW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection These bits are set and cleared by software to select the LPUART1 clock source."]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> Lpuart1selW<RccCciprSpec> {
        Lpuart1selW::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection These bits are set and cleared by software to select the I2C1 clock source."]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2c1selW<RccCciprSpec> {
        I2c1selW::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection These bits are set and cleared by software to select the I2C2 clock source."]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2c2selW<RccCciprSpec> {
        I2c2selW::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection These bits are set and cleared by software to select the I2C3 clock source."]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2c3selW<RccCciprSpec> {
        I2c3selW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection These bits are set and cleared by software to select the LPTIM1 clock source."]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> Lptim1selW<RccCciprSpec> {
        Lptim1selW::new(self, 18)
    }
    #[doc = "Bits 20:21 - clock source selection These bits are set and cleared by software to select the SAI clock source."]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> Sai1selW<RccCciprSpec> {
        Sai1selW::new(self, 20)
    }
    #[doc = "Bits 22:23 - clock source selection These bits are set and cleared by software to select the I2S23 clock source."]
    #[inline(always)]
    pub fn i2s23sel(&mut self) -> I2s23selW<RccCciprSpec> {
        I2s23selW::new(self, 22)
    }
    #[doc = "Bits 24:25 - None"]
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FdcanselW<RccCciprSpec> {
        FdcanselW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection These bits are set and cleared by software to select the 48 MHz clock source used by USB device FS and RNG."]
    #[inline(always)]
    pub fn clk48sel(&mut self) -> Clk48selW<RccCciprSpec> {
        Clk48selW::new(self, 26)
    }
    #[doc = "Bits 28:29 - ADC1/2 clock source selection These bits are set and cleared by software to select the clock source used by the ADC interface."]
    #[inline(always)]
    pub fn adc12sel(&mut self) -> Adc12selW<RccCciprSpec> {
        Adc12selW::new(self, 28)
    }
    #[doc = "Bits 30:31 - ADC3/4/5 clock source selection These bits are set and cleared by software to select the clock source used by the ADC345 interface."]
    #[inline(always)]
    pub fn adc345sel(&mut self) -> Adc345selW<RccCciprSpec> {
        Adc345selW::new(self, 30)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCciprSpec;
impl crate::RegisterSpec for RccCciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ccipr::R`](R) reader structure"]
impl crate::Readable for RccCciprSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ccipr::W`](W) writer structure"]
impl crate::Writable for RccCciprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CCIPR to value 0"]
impl crate::Resettable for RccCciprSpec {
    const RESET_VALUE: u32 = 0;
}
