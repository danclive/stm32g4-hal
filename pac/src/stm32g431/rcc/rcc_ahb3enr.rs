#[doc = "Register `RCC_AHB3ENR` reader"]
pub type R = crate::R<RccAhb3enrSpec>;
#[doc = "Register `RCC_AHB3ENR` writer"]
pub type W = crate::W<RccAhb3enrSpec>;
#[doc = "Flexible static memory controller clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmcen {
    #[doc = "0: FSMC clock disable"]
    B0x0 = 0,
    #[doc = "1: FSMC clock enable"]
    B0x1 = 1,
}
impl From<Fmcen> for bool {
    #[inline(always)]
    fn from(variant: Fmcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCEN` reader - Flexible static memory controller clock enable Set and cleared by software."]
pub type FmcenR = crate::BitReader<Fmcen>;
impl FmcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmcen {
        match self.bits {
            false => Fmcen::B0x0,
            true => Fmcen::B0x1,
        }
    }
    #[doc = "FSMC clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmcen::B0x0
    }
    #[doc = "FSMC clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmcen::B0x1
    }
}
#[doc = "Field `FMCEN` writer - Flexible static memory controller clock enable Set and cleared by software."]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG, Fmcen>;
impl<'a, REG> FmcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FSMC clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcen::B0x0)
    }
    #[doc = "FSMC clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcen::B0x1)
    }
}
#[doc = "QUADSPI memory interface clock enable Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qspien {
    #[doc = "0: QUADSPI clock disable"]
    B0x0 = 0,
    #[doc = "1: QUADSPI clock enable"]
    B0x1 = 1,
}
impl From<Qspien> for bool {
    #[inline(always)]
    fn from(variant: Qspien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPIEN` reader - QUADSPI memory interface clock enable Set and cleared by software."]
pub type QspienR = crate::BitReader<Qspien>;
impl QspienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qspien {
        match self.bits {
            false => Qspien::B0x0,
            true => Qspien::B0x1,
        }
    }
    #[doc = "QUADSPI clock disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Qspien::B0x0
    }
    #[doc = "QUADSPI clock enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Qspien::B0x1
    }
}
#[doc = "Field `QSPIEN` writer - QUADSPI memory interface clock enable Set and cleared by software."]
pub type QspienW<'a, REG> = crate::BitWriter<'a, REG, Qspien>;
impl<'a, REG> QspienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "QUADSPI clock disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Qspien::B0x0)
    }
    #[doc = "QUADSPI clock enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Qspien::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn qspien(&self) -> QspienR {
        QspienR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB3ENR")
            .field("fmcen", &self.fmcen())
            .field("qspien", &self.qspien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FmcenW<'_, RccAhb3enrSpec> {
        FmcenW::new(self, 0)
    }
    #[doc = "Bit 8 - QUADSPI memory interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn qspien(&mut self) -> QspienW<'_, RccAhb3enrSpec> {
        QspienW::new(self, 8)
    }
}
#[doc = "AHB3 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb3enrSpec;
impl crate::RegisterSpec for RccAhb3enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb3enr::R`](R) reader structure"]
impl crate::Readable for RccAhb3enrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb3enr::W`](W) writer structure"]
impl crate::Writable for RccAhb3enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_AHB3ENR to value 0"]
impl crate::Resettable for RccAhb3enrSpec {}
