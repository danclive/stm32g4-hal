#[doc = "Register `RCC_AHB1ENR` reader"]
pub type R = crate::R<RccAhb1enrSpec>;
#[doc = "Register `RCC_AHB1ENR` writer"]
pub type W = crate::W<RccAhb1enrSpec>;
#[doc = "DMA1 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1en {
    #[doc = "0: DMA1 clock disable"]
    B0x0 = 0,
    #[doc = "1: DMA1 clock enable"]
    B0x1 = 1,
}
impl From<Dma1en> for bool {
    #[inline(always)]
    fn from(variant: Dma1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable Set and cleared by software."]
pub type Dma1enR = crate::BitReader<Dma1en>;
impl Dma1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1en {
        match self.bits {
            false => Dma1en::B0x0,
            true => Dma1en::B0x1,
        }
    }
    #[doc = "DMA1 clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma1en::B0x0
    }
    #[doc = "DMA1 clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma1en::B0x1
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable Set and cleared by software."]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG, Dma1en>;
impl<'a, REG> Dma1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA1 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::B0x0)
    }
    #[doc = "DMA1 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma1en::B0x1)
    }
}
#[doc = "DMA2 clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma2en {
    #[doc = "0: DMA2 clock disable"]
    B0x0 = 0,
    #[doc = "1: DMA2 clock enable"]
    B0x1 = 1,
}
impl From<Dma2en> for bool {
    #[inline(always)]
    fn from(variant: Dma2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2EN` reader - DMA2 clock enable Set and cleared by software."]
pub type Dma2enR = crate::BitReader<Dma2en>;
impl Dma2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma2en {
        match self.bits {
            false => Dma2en::B0x0,
            true => Dma2en::B0x1,
        }
    }
    #[doc = "DMA2 clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dma2en::B0x0
    }
    #[doc = "DMA2 clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dma2en::B0x1
    }
}
#[doc = "Field `DMA2EN` writer - DMA2 clock enable Set and cleared by software."]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG, Dma2en>;
impl<'a, REG> Dma2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA2 clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2en::B0x0)
    }
    #[doc = "DMA2 clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma2en::B0x1)
    }
}
#[doc = "DMAMUX1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamux1en {
    #[doc = "0: DMAMUX1 clock disabled"]
    B0x0 = 0,
    #[doc = "1: DMAMUX1 clock enabled"]
    B0x1 = 1,
}
impl From<Dmamux1en> for bool {
    #[inline(always)]
    fn from(variant: Dmamux1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX1EN` reader - DMAMUX1 clock enable Set and reset by software."]
pub type Dmamux1enR = crate::BitReader<Dmamux1en>;
impl Dmamux1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamux1en {
        match self.bits {
            false => Dmamux1en::B0x0,
            true => Dmamux1en::B0x1,
        }
    }
    #[doc = "DMAMUX1 clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Dmamux1en::B0x0
    }
    #[doc = "DMAMUX1 clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Dmamux1en::B0x1
    }
}
#[doc = "Field `DMAMUX1EN` writer - DMAMUX1 clock enable Set and reset by software."]
pub type Dmamux1enW<'a, REG> = crate::BitWriter<'a, REG, Dmamux1en>;
impl<'a, REG> Dmamux1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMAMUX1 clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux1en::B0x0)
    }
    #[doc = "DMAMUX1 clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux1en::B0x1)
    }
}
#[doc = "CORDIC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cordicen {
    #[doc = "0: CORDIC clock disabled"]
    B0x0 = 0,
    #[doc = "1: CORDIC clock enabled"]
    B0x1 = 1,
}
impl From<Cordicen> for bool {
    #[inline(always)]
    fn from(variant: Cordicen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORDICEN` reader - CORDIC clock enable Set and reset by software."]
pub type CordicenR = crate::BitReader<Cordicen>;
impl CordicenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cordicen {
        match self.bits {
            false => Cordicen::B0x0,
            true => Cordicen::B0x1,
        }
    }
    #[doc = "CORDIC clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Cordicen::B0x0
    }
    #[doc = "CORDIC clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Cordicen::B0x1
    }
}
#[doc = "Field `CORDICEN` writer - CORDIC clock enable Set and reset by software."]
pub type CordicenW<'a, REG> = crate::BitWriter<'a, REG, Cordicen>;
impl<'a, REG> CordicenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CORDIC clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Cordicen::B0x0)
    }
    #[doc = "CORDIC clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Cordicen::B0x1)
    }
}
#[doc = "FMAC enable Set and reset by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmacen {
    #[doc = "0: FMAC clock disabled"]
    B0x0 = 0,
    #[doc = "1: FMAC clock enabled"]
    B0x1 = 1,
}
impl From<Fmacen> for bool {
    #[inline(always)]
    fn from(variant: Fmacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMACEN` reader - FMAC enable Set and reset by software."]
pub type FmacenR = crate::BitReader<Fmacen>;
impl FmacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmacen {
        match self.bits {
            false => Fmacen::B0x0,
            true => Fmacen::B0x1,
        }
    }
    #[doc = "FMAC clock disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmacen::B0x0
    }
    #[doc = "FMAC clock enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmacen::B0x1
    }
}
#[doc = "Field `FMACEN` writer - FMAC enable Set and reset by software."]
pub type FmacenW<'a, REG> = crate::BitWriter<'a, REG, Fmacen>;
impl<'a, REG> FmacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FMAC clock disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmacen::B0x0)
    }
    #[doc = "FMAC clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmacen::B0x1)
    }
}
#[doc = "Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashen {
    #[doc = "0: Flash memory interface clock disable"]
    B0x0 = 0,
    #[doc = "1: Flash memory interface clock enable"]
    B0x1 = 1,
}
impl From<Flashen> for bool {
    #[inline(always)]
    fn from(variant: Flashen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
pub type FlashenR = crate::BitReader<Flashen>;
impl FlashenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashen {
        match self.bits {
            false => Flashen::B0x0,
            true => Flashen::B0x1,
        }
    }
    #[doc = "Flash memory interface clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Flashen::B0x0
    }
    #[doc = "Flash memory interface clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Flashen::B0x1
    }
}
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
pub type FlashenW<'a, REG> = crate::BitWriter<'a, REG, Flashen>;
impl<'a, REG> FlashenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory interface clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashen::B0x0)
    }
    #[doc = "Flash memory interface clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashen::B0x1)
    }
}
#[doc = "CRC clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcen {
    #[doc = "0: CRC clock disable"]
    B0x0 = 0,
    #[doc = "1: CRC clock enable"]
    B0x1 = 1,
}
impl From<Crcen> for bool {
    #[inline(always)]
    fn from(variant: Crcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCEN` reader - CRC clock enable Set and cleared by software."]
pub type CrcenR = crate::BitReader<Crcen>;
impl CrcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcen {
        match self.bits {
            false => Crcen::B0x0,
            true => Crcen::B0x1,
        }
    }
    #[doc = "CRC clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Crcen::B0x0
    }
    #[doc = "CRC clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Crcen::B0x1
    }
}
#[doc = "Field `CRCEN` writer - CRC clock enable Set and cleared by software."]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG, Crcen>;
impl<'a, REG> CrcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcen::B0x0)
    }
    #[doc = "CRC clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcen::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dmamux1en(&self) -> Dmamux1enR {
        Dmamux1enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn cordicen(&self) -> CordicenR {
        CordicenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMAC enable Set and reset by software."]
    #[inline(always)]
    pub fn fmacen(&self) -> FmacenR {
        FmacenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
    #[inline(always)]
    pub fn flashen(&self) -> FlashenR {
        FlashenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB1ENR")
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("dmamux1en", &self.dmamux1en())
            .field("cordicen", &self.cordicen())
            .field("fmacen", &self.fmacen())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> Dma1enW<RccAhb1enrSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> Dma2enW<RccAhb1enrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1en(&mut self) -> Dmamux1enW<RccAhb1enrSpec> {
        Dmamux1enW::new(self, 2)
    }
    #[doc = "Bit 3 - CORDIC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn cordicen(&mut self) -> CordicenW<RccAhb1enrSpec> {
        CordicenW::new(self, 3)
    }
    #[doc = "Bit 4 - FMAC enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fmacen(&mut self) -> FmacenW<RccAhb1enrSpec> {
        FmacenW::new(self, 4)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable Set and cleared by software. This bit can be disabled only when the Flash is in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FlashenW<RccAhb1enrSpec> {
        FlashenW::new(self, 8)
    }
    #[doc = "Bit 12 - CRC clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CrcenW<RccAhb1enrSpec> {
        CrcenW::new(self, 12)
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb1enrSpec;
impl crate::RegisterSpec for RccAhb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb1enr::R`](R) reader structure"]
impl crate::Readable for RccAhb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb1enr::W`](W) writer structure"]
impl crate::Writable for RccAhb1enrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB1ENR to value 0x0100"]
impl crate::Resettable for RccAhb1enrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
