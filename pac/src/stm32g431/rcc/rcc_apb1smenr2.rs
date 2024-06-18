#[doc = "Register `RCC_APB1SMENR2` reader"]
pub type R = crate::R<RccApb1smenr2Spec>;
#[doc = "Register `RCC_APB1SMENR2` writer"]
pub type W = crate::W<RccApb1smenr2Spec>;
#[doc = "Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1smen {
    #[doc = "0: LPUART1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: LPUART1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Lpuart1smen> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart1smenR = crate::BitReader<Lpuart1smen>;
impl Lpuart1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1smen {
        match self.bits {
            false => Lpuart1smen::B0x0,
            true => Lpuart1smen::B0x1,
        }
    }
    #[doc = "LPUART1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1smen::B0x0
    }
    #[doc = "LPUART1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1smen::B0x1
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Lpuart1smenW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1smen>;
impl<'a, REG> Lpuart1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1smen::B0x0)
    }
    #[doc = "LPUART1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1smen::B0x1)
    }
}
#[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c4smen {
    #[doc = "0: I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: I2C4 clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<I2c4smen> for bool {
    #[inline(always)]
    fn from(variant: I2c4smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
pub type I2c4smenR = crate::BitReader<I2c4smen>;
impl I2c4smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c4smen {
        match self.bits {
            false => I2c4smen::B0x0,
            true => I2c4smen::B0x1,
        }
    }
    #[doc = "I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4smen::B0x0
    }
    #[doc = "I2C4 clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4smen::B0x1
    }
}
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
pub type I2c4smenW<'a, REG> = crate::BitWriter<'a, REG, I2c4smen>;
impl<'a, REG> I2c4smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4smen::B0x0)
    }
    #[doc = "I2C4 clock enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4smen::B0x1)
    }
}
#[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpd1smen {
    #[doc = "0: UCPD1 clocks disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: UCPD1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Ucpd1smen> for bool {
    #[inline(always)]
    fn from(variant: Ucpd1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPD1SMEN` reader - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Ucpd1smenR = crate::BitReader<Ucpd1smen>;
impl Ucpd1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpd1smen {
        match self.bits {
            false => Ucpd1smen::B0x0,
            true => Ucpd1smen::B0x1,
        }
    }
    #[doc = "UCPD1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ucpd1smen::B0x0
    }
    #[doc = "UCPD1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ucpd1smen::B0x1
    }
}
#[doc = "Field `UCPD1SMEN` writer - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Ucpd1smenW<'a, REG> = crate::BitWriter<'a, REG, Ucpd1smen>;
impl<'a, REG> Ucpd1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UCPD1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1smen::B0x0)
    }
    #[doc = "UCPD1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1smen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> Lpuart1smenR {
        Lpuart1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2c4smenR {
        I2c4smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> Ucpd1smenR {
        Ucpd1smenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1SMENR2")
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c4smen", &self.i2c4smen())
            .field("ucpd1smen", &self.ucpd1smen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> Lpuart1smenW<RccApb1smenr2Spec> {
        Lpuart1smenW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2c4smenW<RccApb1smenr2Spec> {
        I2c4smenW::new(self, 1)
    }
    #[doc = "Bit 8 - UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> Ucpd1smenW<RccApb1smenr2Spec> {
        Ucpd1smenW::new(self, 8)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb1smenr2Spec;
impl crate::RegisterSpec for RccApb1smenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1smenr2::R`](R) reader structure"]
impl crate::Readable for RccApb1smenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1smenr2::W`](W) writer structure"]
impl crate::Writable for RccApb1smenr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB1SMENR2 to value 0x0103"]
impl crate::Resettable for RccApb1smenr2Spec {
    const RESET_VALUE: u32 = 0x0103;
}
