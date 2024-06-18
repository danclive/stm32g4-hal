#[doc = "Register `RCC_APB1ENR2` reader"]
pub type R = crate::R<RccApb1enr2Spec>;
#[doc = "Register `RCC_APB1ENR2` writer"]
pub type W = crate::W<RccApb1enr2Spec>;
#[doc = "Low power UART 1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1en {
    #[doc = "0: LPUART1 clock disable"]
    B0x0 = 0,
    #[doc = "1: LPUART1 clock enable"]
    B0x1 = 1,
}
impl From<Lpuart1en> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable Set and cleared by software."]
pub type Lpuart1enR = crate::BitReader<Lpuart1en>;
impl Lpuart1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1en {
        match self.bits {
            false => Lpuart1en::B0x0,
            true => Lpuart1en::B0x1,
        }
    }
    #[doc = "LPUART1 clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Lpuart1en::B0x0
    }
    #[doc = "LPUART1 clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Lpuart1en::B0x1
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable Set and cleared by software."]
pub type Lpuart1enW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1en>;
impl<'a, REG> Lpuart1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1en::B0x0)
    }
    #[doc = "LPUART1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1en::B0x1)
    }
}
#[doc = "I2C4 clock enable Set and cleared by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c4en {
    #[doc = "0: I2C4 clock disabled"]
    B0x0 = 0,
    #[doc = "1: I2C4 clock enabled"]
    B0x1 = 1,
}
impl From<I2c4en> for bool {
    #[inline(always)]
    fn from(variant: I2c4en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4EN` reader - I2C4 clock enable Set and cleared by software"]
pub type I2c4enR = crate::BitReader<I2c4en>;
impl I2c4enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c4en {
        match self.bits {
            false => I2c4en::B0x0,
            true => I2c4en::B0x1,
        }
    }
    #[doc = "I2C4 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2c4en::B0x0
    }
    #[doc = "I2C4 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2c4en::B0x1
    }
}
#[doc = "Field `I2C4EN` writer - I2C4 clock enable Set and cleared by software"]
pub type I2c4enW<'a, REG> = crate::BitWriter<'a, REG, I2c4en>;
impl<'a, REG> I2c4enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C4 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4en::B0x0)
    }
    #[doc = "I2C4 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c4en::B0x1)
    }
}
#[doc = "UCPD1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpd1en {
    #[doc = "0: UCPD1 clock disable"]
    B0x0 = 0,
    #[doc = "1: UCPD1 clock enable"]
    B0x1 = 1,
}
impl From<Ucpd1en> for bool {
    #[inline(always)]
    fn from(variant: Ucpd1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPD1EN` reader - UCPD1 clock enable Set and cleared by software."]
pub type Ucpd1enR = crate::BitReader<Ucpd1en>;
impl Ucpd1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpd1en {
        match self.bits {
            false => Ucpd1en::B0x0,
            true => Ucpd1en::B0x1,
        }
    }
    #[doc = "UCPD1 clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Ucpd1en::B0x0
    }
    #[doc = "UCPD1 clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Ucpd1en::B0x1
    }
}
#[doc = "Field `UCPD1EN` writer - UCPD1 clock enable Set and cleared by software."]
pub type Ucpd1enW<'a, REG> = crate::BitWriter<'a, REG, Ucpd1en>;
impl<'a, REG> Ucpd1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UCPD1 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1en::B0x0)
    }
    #[doc = "UCPD1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpd1en::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Low power UART 1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> Lpuart1enR {
        Lpuart1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable Set and cleared by software"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2c4enR {
        I2c4enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn ucpd1en(&self) -> Ucpd1enR {
        Ucpd1enR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1ENR2")
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c4en", &self.i2c4en())
            .field("ucpd1en", &self.ucpd1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> Lpuart1enW<RccApb1enr2Spec> {
        Lpuart1enW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2c4enW<RccApb1enr2Spec> {
        I2c4enW::new(self, 1)
    }
    #[doc = "Bit 8 - UCPD1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1en(&mut self) -> Ucpd1enW<RccApb1enr2Spec> {
        Ucpd1enW::new(self, 8)
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccApb1enr2Spec;
impl crate::RegisterSpec for RccApb1enr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb1enr2::R`](R) reader structure"]
impl crate::Readable for RccApb1enr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb1enr2::W`](W) writer structure"]
impl crate::Writable for RccApb1enr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB1ENR2 to value 0"]
impl crate::Resettable for RccApb1enr2Spec {
    const RESET_VALUE: u32 = 0;
}
