#[doc = "Register `RCC_AHB3RSTR` reader"]
pub type R = crate::R<RccAhb3rstrSpec>;
#[doc = "Register `RCC_AHB3RSTR` writer"]
pub type W = crate::W<RccAhb3rstrSpec>;
#[doc = "Flexible static memory controller reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fmcrst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset FSMC"]
    B0x1 = 1,
}
impl From<Fmcrst> for bool {
    #[inline(always)]
    fn from(variant: Fmcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCRST` reader - Flexible static memory controller reset Set and cleared by software."]
pub type FmcrstR = crate::BitReader<Fmcrst>;
impl FmcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmcrst {
        match self.bits {
            false => Fmcrst::B0x0,
            true => Fmcrst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Fmcrst::B0x0
    }
    #[doc = "Reset FSMC"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Fmcrst::B0x1
    }
}
#[doc = "Field `FMCRST` writer - Flexible static memory controller reset Set and cleared by software."]
pub type FmcrstW<'a, REG> = crate::BitWriter<'a, REG, Fmcrst>;
impl<'a, REG> FmcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcrst::B0x0)
    }
    #[doc = "Reset FSMC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcrst::B0x1)
    }
}
#[doc = "QUADSPI reset Set and cleared by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qspirst {
    #[doc = "0: No effect"]
    B0x0 = 0,
    #[doc = "1: Reset QUADSPI"]
    B0x1 = 1,
}
impl From<Qspirst> for bool {
    #[inline(always)]
    fn from(variant: Qspirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPIRST` reader - QUADSPI reset Set and cleared by software."]
pub type QspirstR = crate::BitReader<Qspirst>;
impl QspirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qspirst {
        match self.bits {
            false => Qspirst::B0x0,
            true => Qspirst::B0x1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Qspirst::B0x0
    }
    #[doc = "Reset QUADSPI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Qspirst::B0x1
    }
}
#[doc = "Field `QSPIRST` writer - QUADSPI reset Set and cleared by software."]
pub type QspirstW<'a, REG> = crate::BitWriter<'a, REG, Qspirst>;
impl<'a, REG> QspirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Qspirst::B0x0)
    }
    #[doc = "Reset QUADSPI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Qspirst::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Flexible static memory controller reset Set and cleared by software."]
    #[inline(always)]
    pub fn fmcrst(&self) -> FmcrstR {
        FmcrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - QUADSPI reset Set and cleared by software."]
    #[inline(always)]
    pub fn qspirst(&self) -> QspirstR {
        QspirstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB3RSTR")
            .field("fmcrst", &self.fmcrst())
            .field("qspirst", &self.qspirst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller reset Set and cleared by software."]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FmcrstW<RccAhb3rstrSpec> {
        FmcrstW::new(self, 0)
    }
    #[doc = "Bit 8 - QUADSPI reset Set and cleared by software."]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QspirstW<RccAhb3rstrSpec> {
        QspirstW::new(self, 8)
    }
}
#[doc = "AHB3 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcc_ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RccAhb3rstrSpec;
impl crate::RegisterSpec for RccAhb3rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb3rstr::R`](R) reader structure"]
impl crate::Readable for RccAhb3rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb3rstr::W`](W) writer structure"]
impl crate::Writable for RccAhb3rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCC_AHB3RSTR to value 0"]
impl crate::Resettable for RccAhb3rstrSpec {}
