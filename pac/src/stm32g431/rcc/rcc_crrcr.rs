#[doc = "Register `RCC_CRRCR` reader"]
pub type R = crate::R<RccCrrcrSpec>;
#[doc = "Register `RCC_CRRCR` writer"]
pub type W = crate::W<RccCrrcrSpec>;
#[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi48on {
    #[doc = "0: HSI48 oscillator OFF"]
    B0x0 = 0,
    #[doc = "1: HSI48 oscillator ON"]
    B0x1 = 1,
}
impl From<Hsi48on> for bool {
    #[inline(always)]
    fn from(variant: Hsi48on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48ON` reader - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
pub type Hsi48onR = crate::BitReader<Hsi48on>;
impl Hsi48onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsi48on {
        match self.bits {
            false => Hsi48on::B0x0,
            true => Hsi48on::B0x1,
        }
    }
    #[doc = "HSI48 oscillator OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsi48on::B0x0
    }
    #[doc = "HSI48 oscillator ON"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsi48on::B0x1
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
pub type Hsi48onW<'a, REG> = crate::BitWriter<'a, REG, Hsi48on>;
impl<'a, REG> Hsi48onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI48 oscillator OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48on::B0x0)
    }
    #[doc = "HSI48 oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi48on::B0x1)
    }
}
#[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi48rdy {
    #[doc = "0: HSI48 oscillator not ready"]
    B0x0 = 0,
    #[doc = "1: HSI48 oscillator ready"]
    B0x1 = 1,
}
impl From<Hsi48rdy> for bool {
    #[inline(always)]
    fn from(variant: Hsi48rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
pub type Hsi48rdyR = crate::BitReader<Hsi48rdy>;
impl Hsi48rdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsi48rdy {
        match self.bits {
            false => Hsi48rdy::B0x0,
            true => Hsi48rdy::B0x1,
        }
    }
    #[doc = "HSI48 oscillator not ready"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Hsi48rdy::B0x0
    }
    #[doc = "HSI48 oscillator ready"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Hsi48rdy::B0x1
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value. They are ready only."]
pub type Hsi48calR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
    #[inline(always)]
    pub fn hsi48on(&self) -> Hsi48onR {
        Hsi48onR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> Hsi48rdyR {
        Hsi48rdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:15 - HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value. They are ready only."]
    #[inline(always)]
    pub fn hsi48cal(&self) -> Hsi48calR {
        Hsi48calR::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CRRCR")
            .field("hsi48on", &self.hsi48on())
            .field("hsi48rdy", &self.hsi48rdy())
            .field("hsi48cal", &self.hsi48cal())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> Hsi48onW<RccCrrcrSpec> {
        Hsi48onW::new(self, 0)
    }
}
#[doc = "Clock recovery RC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_crrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_crrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccCrrcrSpec;
impl crate::RegisterSpec for RccCrrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_crrcr::R`](R) reader structure"]
impl crate::Readable for RccCrrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_crrcr::W`](W) writer structure"]
impl crate::Writable for RccCrrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_CRRCR to value 0"]
impl crate::Resettable for RccCrrcrSpec {}
