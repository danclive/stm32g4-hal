#[doc = "Register `RCC_AHB1SMENR` reader"]
pub type R = crate::R<RccAhb1smenrSpec>;
#[doc = "Register `RCC_AHB1SMENR` writer"]
pub type W = crate::W<RccAhb1smenrSpec>;
#[doc = "DMA1 clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1smen {
    #[doc = "0: DMA1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: DMA1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Dma1smen> for bool {
    #[inline(always)]
    fn from(variant: Dma1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1SMEN` reader - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Dma1smenR = crate::BitReader<Dma1smen>;
impl Dma1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1smen {
        match self.bits {
            false => Dma1smen::B0x0,
            true => Dma1smen::B0x1,
        }
    }
    #[doc = "DMA1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma1smen::B0x0
    }
    #[doc = "DMA1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma1smen::B0x1
    }
}
#[doc = "Field `DMA1SMEN` writer - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Dma1smenW<'a, REG> = crate::BitWriter<'a, REG, Dma1smen>;
impl<'a, REG> Dma1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1smen::B0x0)
    }
    #[doc = "DMA1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1smen::B0x1)
    }
}
#[doc = "DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2smen {
    #[doc = "0: DMA2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: DMA2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Dma2smen> for bool {
    #[inline(always)]
    fn from(variant: Dma2smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2SMEN` reader - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
pub type Dma2smenR = crate::BitReader<Dma2smen>;
impl Dma2smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2smen {
        match self.bits {
            false => Dma2smen::B0x0,
            true => Dma2smen::B0x1,
        }
    }
    #[doc = "DMA2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma2smen::B0x0
    }
    #[doc = "DMA2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma2smen::B0x1
    }
}
#[doc = "Field `DMA2SMEN` writer - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
pub type Dma2smenW<'a, REG> = crate::BitWriter<'a, REG, Dma2smen>;
impl<'a, REG> Dma2smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA2 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2smen::B0x0)
    }
    #[doc = "DMA2 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2smen::B0x1)
    }
}
#[doc = "DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamux1smen {
    #[doc = "0: DMAMUX1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: DMAMUX1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Dmamux1smen> for bool {
    #[inline(always)]
    fn from(variant: Dmamux1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX1SMEN` reader - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
pub type Dmamux1smenR = crate::BitReader<Dmamux1smen>;
impl Dmamux1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamux1smen {
        match self.bits {
            false => Dmamux1smen::B0x0,
            true => Dmamux1smen::B0x1,
        }
    }
    #[doc = "DMAMUX1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmamux1smen::B0x0
    }
    #[doc = "DMAMUX1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmamux1smen::B0x1
    }
}
#[doc = "Field `DMAMUX1SMEN` writer - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
pub type Dmamux1smenW<'a, REG> = crate::BitWriter<'a, REG, Dmamux1smen>;
impl<'a, REG> Dmamux1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAMUX1 clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux1smen::B0x0)
    }
    #[doc = "DMAMUX1 clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux1smen::B0x1)
    }
}
#[doc = "CORDICSM clock enable. Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cordicsmen {
    #[doc = "0: CORDICSM clocks disabled."]
    B0x0 = 0,
    #[doc = "1: CORDICSM clocks enabled."]
    B0x1 = 1,
}
impl From<Cordicsmen> for bool {
    #[inline(always)]
    fn from(variant: Cordicsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORDICSMEN` reader - CORDICSM clock enable. Set and cleared by software."]
pub type CordicsmenR = crate::BitReader<Cordicsmen>;
impl CordicsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cordicsmen {
        match self.bits {
            false => Cordicsmen::B0x0,
            true => Cordicsmen::B0x1,
        }
    }
    #[doc = "CORDICSM clocks disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cordicsmen::B0x0
    }
    #[doc = "CORDICSM clocks enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cordicsmen::B0x1
    }
}
#[doc = "Field `CORDICSMEN` writer - CORDICSM clock enable. Set and cleared by software."]
pub type CordicsmenW<'a, REG> = crate::BitWriter<'a, REG, Cordicsmen>;
impl<'a, REG> CordicsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CORDICSM clocks disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cordicsmen::B0x0)
    }
    #[doc = "CORDICSM clocks enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cordicsmen::B0x1)
    }
}
#[doc = "FMACSM clock enable. Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmacsmen {
    #[doc = "0: FMACSM clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: FMACSM clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Fmacsmen> for bool {
    #[inline(always)]
    fn from(variant: Fmacsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMACSMEN` reader - FMACSM clock enable. Set and cleared by software."]
pub type FmacsmenR = crate::BitReader<Fmacsmen>;
impl FmacsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmacsmen {
        match self.bits {
            false => Fmacsmen::B0x0,
            true => Fmacsmen::B0x1,
        }
    }
    #[doc = "FMACSM clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmacsmen::B0x0
    }
    #[doc = "FMACSM clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmacsmen::B0x1
    }
}
#[doc = "Field `FMACSMEN` writer - FMACSM clock enable. Set and cleared by software."]
pub type FmacsmenW<'a, REG> = crate::BitWriter<'a, REG, Fmacsmen>;
impl<'a, REG> FmacsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FMACSM clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmacsmen::B0x0)
    }
    #[doc = "FMACSM clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmacsmen::B0x1)
    }
}
#[doc = "Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashsmen {
    #[doc = "0: Flash memory interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: Flash memory interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Flashsmen> for bool {
    #[inline(always)]
    fn from(variant: Flashsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHSMEN` reader - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FlashsmenR = crate::BitReader<Flashsmen>;
impl FlashsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashsmen {
        match self.bits {
            false => Flashsmen::B0x0,
            true => Flashsmen::B0x1,
        }
    }
    #[doc = "Flash memory interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flashsmen::B0x0
    }
    #[doc = "Flash memory interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flashsmen::B0x1
    }
}
#[doc = "Field `FLASHSMEN` writer - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type FlashsmenW<'a, REG> = crate::BitWriter<'a, REG, Flashsmen>;
impl<'a, REG> FlashsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsmen::B0x0)
    }
    #[doc = "Flash memory interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashsmen::B0x1)
    }
}
#[doc = "SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sram1smen {
    #[doc = "0: SRAM1 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: SRAM1 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Sram1smen> for bool {
    #[inline(always)]
    fn from(variant: Sram1smen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Sram1smenR = crate::BitReader<Sram1smen>;
impl Sram1smenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sram1smen {
        match self.bits {
            false => Sram1smen::B0x0,
            true => Sram1smen::B0x1,
        }
    }
    #[doc = "SRAM1 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Sram1smen::B0x0
    }
    #[doc = "SRAM1 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Sram1smen::B0x1
    }
}
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type Sram1smenW<'a, REG> = crate::BitWriter<'a, REG, Sram1smen>;
impl<'a, REG> Sram1smenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 interface clocks disabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Sram1smen::B0x0)
    }
    #[doc = "SRAM1 interface clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Sram1smen::B0x1)
    }
}
#[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcsmen {
    #[doc = "0: CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    B0x0 = 0,
    #[doc = "1: CRC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    B0x1 = 1,
}
impl From<Crcsmen> for bool {
    #[inline(always)]
    fn from(variant: Crcsmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSMEN` reader - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CrcsmenR = crate::BitReader<Crcsmen>;
impl CrcsmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcsmen {
        match self.bits {
            false => Crcsmen::B0x0,
            true => Crcsmen::B0x1,
        }
    }
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcsmen::B0x0
    }
    #[doc = "CRC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcsmen::B0x1
    }
}
#[doc = "Field `CRCSMEN` writer - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
pub type CrcsmenW<'a, REG> = crate::BitWriter<'a, REG, Crcsmen>;
impl<'a, REG> CrcsmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsmen::B0x0)
    }
    #[doc = "CRC clocks enabled by the clock gating&lt;sup>(1)&lt;/sup> during Sleep and Stop modes"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsmen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn dma1smen(&self) -> Dma1smenR {
        Dma1smenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
    #[inline(always)]
    pub fn dma2smen(&self) -> Dma2smenR {
        Dma2smenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> Dmamux1smenR {
        Dmamux1smenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CORDICSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub fn cordicsmen(&self) -> CordicsmenR {
        CordicsmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMACSM clock enable. Set and cleared by software."]
    #[inline(always)]
    pub fn fmacsmen(&self) -> FmacsmenR {
        FmacsmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FlashsmenR {
        FlashsmenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn sram1smen(&self) -> Sram1smenR {
        Sram1smenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CrcsmenR {
        CrcsmenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB1SMENR")
            .field("dma1smen", &self.dma1smen())
            .field("dma2smen", &self.dma2smen())
            .field("dmamux1smen", &self.dmamux1smen())
            .field("cordicsmen", &self.cordicsmen())
            .field("fmacsmen", &self.fmacsmen())
            .field("flashsmen", &self.flashsmen())
            .field("sram1smen", &self.sram1smen())
            .field("crcsmen", &self.crcsmen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1smen(&mut self) -> Dma1smenW<RccAhb1smenrSpec> {
        Dma1smenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma2smen(&mut self) -> Dma2smenW<RccAhb1smenrSpec> {
        Dma2smenW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during Sleep and Stop modes. Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1smen(&mut self) -> Dmamux1smenW<RccAhb1smenrSpec> {
        Dmamux1smenW::new(self, 2)
    }
    #[doc = "Bit 3 - CORDICSM clock enable. Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicsmen(&mut self) -> CordicsmenW<RccAhb1smenrSpec> {
        CordicsmenW::new(self, 3)
    }
    #[doc = "Bit 4 - FMACSM clock enable. Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacsmen(&mut self) -> FmacsmenW<RccAhb1smenrSpec> {
        FmacsmenW::new(self, 4)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FlashsmenW<RccAhb1smenrSpec> {
        FlashsmenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram1smen(&mut self) -> Sram1smenW<RccAhb1smenrSpec> {
        Sram1smenW::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcsmen(&mut self) -> CrcsmenW<RccAhb1smenrSpec> {
        CrcsmenW::new(self, 12)
    }
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb1smenrSpec;
impl crate::RegisterSpec for RccAhb1smenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb1smenr::R`](R) reader structure"]
impl crate::Readable for RccAhb1smenrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb1smenr::W`](W) writer structure"]
impl crate::Writable for RccAhb1smenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB1SMENR to value 0x130f"]
impl crate::Resettable for RccAhb1smenrSpec {
    const RESET_VALUE: u32 = 0x130f;
}
