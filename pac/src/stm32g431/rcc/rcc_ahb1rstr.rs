#[doc = "Register `RCC_AHB1RSTR` reader"]
pub type R = crate::R<RccAhb1rstrSpec>;
#[doc = "Register `RCC_AHB1RSTR` writer"]
pub type W = crate::W<RccAhb1rstrSpec>;
#[doc = "DMA1 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DMA1"]
    B0x1 = 1,
}
impl From<Dma1rst> for bool {
    #[inline(always)]
    fn from(variant: Dma1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 reset Set and cleared by software."]
pub type Dma1rstR = crate::BitReader<Dma1rst>;
impl Dma1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1rst {
        match self.bits {
            false => Dma1rst::B0x0,
            true => Dma1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma1rst::B0x0
    }
    #[doc = "Reset DMA1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma1rst::B0x1
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 reset Set and cleared by software."]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG, Dma1rst>;
impl<'a, REG> Dma1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rst::B0x0)
    }
    #[doc = "Reset DMA1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1rst::B0x1)
    }
}
#[doc = "DMA2 reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DMA2"]
    B0x1 = 1,
}
impl From<Dma2rst> for bool {
    #[inline(always)]
    fn from(variant: Dma2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 reset Set and cleared by software."]
pub type Dma2rstR = crate::BitReader<Dma2rst>;
impl Dma2rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2rst {
        match self.bits {
            false => Dma2rst::B0x0,
            true => Dma2rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma2rst::B0x0
    }
    #[doc = "Reset DMA2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma2rst::B0x1
    }
}
#[doc = "Field `DMA2RST` writer - DMA2 reset Set and cleared by software."]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG, Dma2rst>;
impl<'a, REG> Dma2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2rst::B0x0)
    }
    #[doc = "Reset DMA2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2rst::B0x1)
    }
}
#[doc = "Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamux1rst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset DMAMUX1"]
    B0x1 = 1,
}
impl From<Dmamux1rst> for bool {
    #[inline(always)]
    fn from(variant: Dmamux1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX1RST` reader - Set and cleared by software."]
pub type Dmamux1rstR = crate::BitReader<Dmamux1rst>;
impl Dmamux1rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamux1rst {
        match self.bits {
            false => Dmamux1rst::B0x0,
            true => Dmamux1rst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmamux1rst::B0x0
    }
    #[doc = "Reset DMAMUX1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmamux1rst::B0x1
    }
}
#[doc = "Field `DMAMUX1RST` writer - Set and cleared by software."]
pub type Dmamux1rstW<'a, REG> = crate::BitWriter<'a, REG, Dmamux1rst>;
impl<'a, REG> Dmamux1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux1rst::B0x0)
    }
    #[doc = "Reset DMAMUX1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux1rst::B0x1)
    }
}
#[doc = "Set and cleared by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cordicrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset CORDIC"]
    B0x1 = 1,
}
impl From<Cordicrst> for bool {
    #[inline(always)]
    fn from(variant: Cordicrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORDICRST` reader - Set and cleared by software"]
pub type CordicrstR = crate::BitReader<Cordicrst>;
impl CordicrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cordicrst {
        match self.bits {
            false => Cordicrst::B0x0,
            true => Cordicrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cordicrst::B0x0
    }
    #[doc = "Reset CORDIC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cordicrst::B0x1
    }
}
#[doc = "Field `CORDICRST` writer - Set and cleared by software"]
pub type CordicrstW<'a, REG> = crate::BitWriter<'a, REG, Cordicrst>;
impl<'a, REG> CordicrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cordicrst::B0x0)
    }
    #[doc = "Reset CORDIC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cordicrst::B0x1)
    }
}
#[doc = "Set and cleared by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmacrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset FMAC"]
    B0x1 = 1,
}
impl From<Fmacrst> for bool {
    #[inline(always)]
    fn from(variant: Fmacrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMACRST` reader - Set and cleared by software"]
pub type FmacrstR = crate::BitReader<Fmacrst>;
impl FmacrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmacrst {
        match self.bits {
            false => Fmacrst::B0x0,
            true => Fmacrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmacrst::B0x0
    }
    #[doc = "Reset FMAC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmacrst::B0x1
    }
}
#[doc = "Field `FMACRST` writer - Set and cleared by software"]
pub type FmacrstW<'a, REG> = crate::BitWriter<'a, REG, Fmacrst>;
impl<'a, REG> FmacrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmacrst::B0x0)
    }
    #[doc = "Reset FMAC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmacrst::B0x1)
    }
}
#[doc = "Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset Flash memory interface"]
    B0x1 = 1,
}
impl From<Flashrst> for bool {
    #[inline(always)]
    fn from(variant: Flashrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHRST` reader - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
pub type FlashrstR = crate::BitReader<Flashrst>;
impl FlashrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashrst {
        match self.bits {
            false => Flashrst::B0x0,
            true => Flashrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flashrst::B0x0
    }
    #[doc = "Reset Flash memory interface"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flashrst::B0x1
    }
}
#[doc = "Field `FLASHRST` writer - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
pub type FlashrstW<'a, REG> = crate::BitWriter<'a, REG, Flashrst>;
impl<'a, REG> FlashrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashrst::B0x0)
    }
    #[doc = "Reset Flash memory interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashrst::B0x1)
    }
}
#[doc = "CRC reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset CRC"]
    B0x1 = 1,
}
impl From<Crcrst> for bool {
    #[inline(always)]
    fn from(variant: Crcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCRST` reader - CRC reset Set and cleared by software."]
pub type CrcrstR = crate::BitReader<Crcrst>;
impl CrcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcrst {
        match self.bits {
            false => Crcrst::B0x0,
            true => Crcrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcrst::B0x0
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcrst::B0x1
    }
}
#[doc = "Field `CRCRST` writer - CRC reset Set and cleared by software."]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG, Crcrst>;
impl<'a, REG> CrcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrst::B0x0)
    }
    #[doc = "Reset CRC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcrst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset Set and cleared by software."]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set and cleared by software."]
    #[inline(always)]
    pub fn dmamux1rst(&self) -> Dmamux1rstR {
        Dmamux1rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set and cleared by software"]
    #[inline(always)]
    pub fn cordicrst(&self) -> CordicrstR {
        CordicrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set and cleared by software"]
    #[inline(always)]
    pub fn fmacrst(&self) -> FmacrstR {
        FmacrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
    #[inline(always)]
    pub fn flashrst(&self) -> FlashrstR {
        FlashrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB1RSTR")
            .field("dma1rst", &self.dma1rst())
            .field("dma2rst", &self.dma2rst())
            .field("dmamux1rst", &self.dmamux1rst())
            .field("cordicrst", &self.cordicrst())
            .field("fmacrst", &self.fmacrst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> Dma1rstW<RccAhb1rstrSpec> {
        Dma1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> Dma2rstW<RccAhb1rstrSpec> {
        Dma2rstW::new(self, 1)
    }
    #[doc = "Bit 2 - Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1rst(&mut self) -> Dmamux1rstW<RccAhb1rstrSpec> {
        Dmamux1rstW::new(self, 2)
    }
    #[doc = "Bit 3 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn cordicrst(&mut self) -> CordicrstW<RccAhb1rstrSpec> {
        CordicrstW::new(self, 3)
    }
    #[doc = "Bit 4 - Set and cleared by software"]
    #[inline(always)]
    #[must_use]
    pub fn fmacrst(&mut self) -> FmacrstW<RccAhb1rstrSpec> {
        FmacrstW::new(self, 4)
    }
    #[doc = "Bit 8 - Flash memory interface reset Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FlashrstW<RccAhb1rstrSpec> {
        FlashrstW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CrcrstW<RccAhb1rstrSpec> {
        CrcrstW::new(self, 12)
    }
}
#[doc = "AHB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb1rstrSpec;
impl crate::RegisterSpec for RccAhb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb1rstr::R`](R) reader structure"]
impl crate::Readable for RccAhb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb1rstr::W`](W) writer structure"]
impl crate::Writable for RccAhb1rstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB1RSTR to value 0"]
impl crate::Resettable for RccAhb1rstrSpec {
    const RESET_VALUE: u32 = 0;
}
