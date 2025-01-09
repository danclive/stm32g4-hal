#[doc = "Register `RCC_CCIPR2` reader"]
pub type R = crate::R<RccCcipr2Spec>;
#[doc = "Register `RCC_CCIPR2` writer"]
pub type W = crate::W<RccCcipr2Spec>;
#[doc = "I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c4sel {
    #[doc = "0: PCLK selected as I2C4 clock"]
    B0x0 = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C4 clock"]
    B0x1 = 1,
    #[doc = "2: HSI16 clock selected as I2C4 clock"]
    B0x2 = 2,
}
impl From<I2c4sel> for u8 {
    #[inline(always)]
    fn from(variant: I2c4sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2c4sel {
    type Ux = u8;
}
impl crate::IsEnum for I2c4sel {}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
pub type I2c4selR = crate::FieldReader<I2c4sel>;
impl I2c4selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2c4sel> {
        match self.bits {
            0 => Some(I2c4sel::B0x0),
            1 => Some(I2c4sel::B0x1),
            2 => Some(I2c4sel::B0x2),
            _ => None,
        }
    }
    #[doc = "PCLK selected as I2C4 clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4sel::B0x0
    }
    #[doc = "System clock (SYSCLK) selected as I2C4 clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4sel::B0x1
    }
    #[doc = "HSI16 clock selected as I2C4 clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2c4sel::B0x2
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
pub type I2c4selW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2c4sel>;
impl<'a, REG> I2c4selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK selected as I2C4 clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4sel::B0x0)
    }
    #[doc = "System clock (SYSCLK) selected as I2C4 clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4sel::B0x1)
    }
    #[doc = "HSI16 clock selected as I2C4 clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4sel::B0x2)
    }
}
#[doc = "QUADSPI clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qspisel {
    #[doc = "0: system clock selected as QUADSPI kernel clock"]
    B0x0 = 0,
    #[doc = "1: HSI16 clock selected as QUADSPI kernel clock"]
    B0x1 = 1,
    #[doc = "2: PLL Q clock selected as QUADSPI kernel clock"]
    B0x2 = 2,
}
impl From<Qspisel> for u8 {
    #[inline(always)]
    fn from(variant: Qspisel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qspisel {
    type Ux = u8;
}
impl crate::IsEnum for Qspisel {}
#[doc = "Field `QSPISEL` reader - QUADSPI clock source selection Set and reset by software."]
pub type QspiselR = crate::FieldReader<Qspisel>;
impl QspiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Qspisel> {
        match self.bits {
            0 => Some(Qspisel::B0x0),
            1 => Some(Qspisel::B0x1),
            2 => Some(Qspisel::B0x2),
            _ => None,
        }
    }
    #[doc = "system clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Qspisel::B0x0
    }
    #[doc = "HSI16 clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Qspisel::B0x1
    }
    #[doc = "PLL Q clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == Qspisel::B0x2
    }
}
#[doc = "Field `QSPISEL` writer - QUADSPI clock source selection Set and reset by software."]
pub type QspiselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Qspisel>;
impl<'a, REG> QspiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "system clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Qspisel::B0x0)
    }
    #[doc = "HSI16 clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Qspisel::B0x1)
    }
    #[doc = "PLL Q clock selected as QUADSPI kernel clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Qspisel::B0x2)
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2c4selR {
        I2c4selR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 20:21 - QUADSPI clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn qspisel(&self) -> QspiselR {
        QspiselR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCIPR2")
            .field("i2c4sel", &self.i2c4sel())
            .field("qspisel", &self.qspisel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection These bits are set and cleared by software to select the I2C4 clock source."]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2c4selW<RccCcipr2Spec> {
        I2c4selW::new(self, 0)
    }
    #[doc = "Bits 20:21 - QUADSPI clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn qspisel(&mut self) -> QspiselW<RccCcipr2Spec> {
        QspiselW::new(self, 20)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCcipr2Spec;
impl crate::RegisterSpec for RccCcipr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ccipr2::R`](R) reader structure"]
impl crate::Readable for RccCcipr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ccipr2::W`](W) writer structure"]
impl crate::Writable for RccCcipr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CCIPR2 to value 0"]
impl crate::Resettable for RccCcipr2Spec {
    const RESET_VALUE: u32 = 0;
}
