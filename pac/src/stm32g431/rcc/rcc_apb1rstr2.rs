#[doc = "Register `RCC_APB1RSTR2` reader"]
pub type R = crate::R<RccApb1rstr2Spec>;
#[doc = "Register `RCC_APB1RSTR2` writer"]
pub type W = crate::W<RccApb1rstr2Spec>;
#[doc = "Low-power UART 1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset LPUART1"]
    B0x1 = 1,
}
impl From<Lpuart1rst> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset Set and cleared by software."]
pub type Lpuart1rstR = crate::BitReader<Lpuart1rst>;
impl Lpuart1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1rst {
        match self.bits {
            false => Lpuart1rst::B0x0,
            true => Lpuart1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1rst::B0x0
    }
    #[doc = "Reset LPUART1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1rst::B0x1
    }
}
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset Set and cleared by software."]
pub type Lpuart1rstW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1rst>;
impl<'a, REG> Lpuart1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1rst::B0x0)
    }
    #[doc = "Reset LPUART1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1rst::B0x1)
    }
}
#[doc = "I2C4 reset Set and cleared by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c4rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset I2C4"]
    B0x1 = 1,
}
impl From<I2c4rst> for bool {
    #[inline(always)]
    fn from(variant: I2c4rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4RST` reader - I2C4 reset Set and cleared by software"]
pub type I2c4rstR = crate::BitReader<I2c4rst>;
impl I2c4rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c4rst {
        match self.bits {
            false => I2c4rst::B0x0,
            true => I2c4rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4rst::B0x0
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4rst::B0x1
    }
}
#[doc = "Field `I2C4RST` writer - I2C4 reset Set and cleared by software"]
pub type I2c4rstW<'a, REG> = crate::BitWriter<'a, REG, I2c4rst>;
impl<'a, REG> I2c4rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4rst::B0x0)
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4rst::B0x1)
    }
}
#[doc = "UCPD1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpd1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset UCPD1"]
    B0x1 = 1,
}
impl From<Ucpd1rst> for bool {
    #[inline(always)]
    fn from(variant: Ucpd1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPD1RST` reader - UCPD1 reset Set and cleared by software."]
pub type Ucpd1rstR = crate::BitReader<Ucpd1rst>;
impl Ucpd1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpd1rst {
        match self.bits {
            false => Ucpd1rst::B0x0,
            true => Ucpd1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ucpd1rst::B0x0
    }
    #[doc = "Reset UCPD1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ucpd1rst::B0x1
    }
}
#[doc = "Field `UCPD1RST` writer - UCPD1 reset Set and cleared by software."]
pub type Ucpd1rstW<'a, REG> = crate::BitWriter<'a, REG, Ucpd1rst>;
impl<'a, REG> Ucpd1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1rst::B0x0)
    }
    #[doc = "Reset UCPD1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1rst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> Lpuart1rstR {
        Lpuart1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 reset Set and cleared by software"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2c4rstR {
        I2c4rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn ucpd1rst(&self) -> Ucpd1rstR {
        Ucpd1rstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1RSTR2")
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c4rst", &self.i2c4rst())
            .field("ucpd1rst", &self.ucpd1rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> Lpuart1rstW<RccApb1rstr2Spec> {
        Lpuart1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 reset Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2c4rstW<RccApb1rstr2Spec> {
        I2c4rstW::new(self, 1)
    }
    #[doc = "Bit 8 - UCPD1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1rst(&mut self) -> Ucpd1rstW<RccApb1rstr2Spec> {
        Ucpd1rstW::new(self, 8)
    }
}
#[doc = "APB1 peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb1rstr2Spec;
impl crate::RegisterSpec for RccApb1rstr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1rstr2::R`](R) reader structure"]
impl crate::Readable for RccApb1rstr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1rstr2::W`](W) writer structure"]
impl crate::Writable for RccApb1rstr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB1RSTR2 to value 0"]
impl crate::Resettable for RccApb1rstr2Spec {
    const RESET_VALUE: u32 = 0;
}
