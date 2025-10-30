#[doc = "Register `RCC_AHB3SMENR` reader"]
pub type R = crate::R<RccAhb3smenrSpec>;
#[doc = "Register `RCC_AHB3SMENR` writer"]
pub type W = crate::W<RccAhb3smenrSpec>;
#[doc = "Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmcsmen {
    #[doc = "0: FSMC clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: FSMC clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Fmcsmen> for bool {
    #[inline(always)]
    fn from(variant: Fmcsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCSMEN` reader - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FmcsmenR = crate::BitReader<Fmcsmen>;
impl FmcsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmcsmen {
        match self.bits {
            false => Fmcsmen::B0x0,
            true => Fmcsmen::B0x1,
        }
    }
    #[doc = "FSMC clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmcsmen::B0x0
    }
    #[doc = "FSMC clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmcsmen::B0x1
    }
}
#[doc = "Field `FMCSMEN` writer - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FmcsmenW<'a, REG> = crate::BitWriter<'a, REG, Fmcsmen>;
impl<'a, REG> FmcsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FSMC clocks disabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcsmen::B0x0)
    }
    #[doc = "FSMC clocks enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcsmen::B0x1)
    }
}
#[doc = "QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qspismen {
    #[doc = "0: QUADSPI clock disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: QUADSPI clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Qspismen> for bool {
    #[inline(always)]
    fn from(variant: Qspismen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPISMEN` reader - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
pub type QspismenR = crate::BitReader<Qspismen>;
impl QspismenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qspismen {
        match self.bits {
            false => Qspismen::B0x0,
            true => Qspismen::B0x1,
        }
    }
    #[doc = "QUADSPI clock disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Qspismen::B0x0
    }
    #[doc = "QUADSPI clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Qspismen::B0x1
    }
}
#[doc = "Field `QSPISMEN` writer - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
pub type QspismenW<'a, REG> = crate::BitWriter<'a, REG, Qspismen>;
impl<'a, REG> QspismenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "QUADSPI clock disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Qspismen::B0x0)
    }
    #[doc = "QUADSPI clock enabled by the clock gating<sup>(1)</sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Qspismen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn fmcsmen(&self) -> FmcsmenR {
        FmcsmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn qspismen(&self) -> QspismenR {
        QspismenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB3SMENR")
            .field("fmcsmen", &self.fmcsmen())
            .field("qspismen", &self.qspismen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn fmcsmen(&mut self) -> FmcsmenW<'_, RccAhb3smenrSpec> {
        FmcsmenW::new(self, 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn qspismen(&mut self) -> QspismenW<'_, RccAhb3smenrSpec> {
        QspismenW::new(self, 8)
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb3smenrSpec;
impl crate::RegisterSpec for RccAhb3smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb3smenr::R`](R) reader structure"]
impl crate::Readable for RccAhb3smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb3smenr::W`](W) writer structure"]
impl crate::Writable for RccAhb3smenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_AHB3SMENR to value 0x0101"]
impl crate::Resettable for RccAhb3smenrSpec {
    const RESET_VALUE: u32 = 0x0101;
}
