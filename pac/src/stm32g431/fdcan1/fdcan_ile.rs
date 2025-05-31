#[doc = "Register `FDCAN_ILE` reader"]
pub type R = crate::R<FdcanIleSpec>;
#[doc = "Register `FDCAN_ILE` writer"]
pub type W = crate::W<FdcanIleSpec>;
#[doc = "Enable interrupt line 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eint0 {
    #[doc = "0: Interrupt line fdcan_intr1_it disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt line fdcan_intr1_it enabled"]
    B0x1 = 1,
}
impl From<Eint0> for bool {
    #[inline(always)]
    fn from(variant: Eint0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT0` reader - Enable interrupt line 0"]
pub type Eint0R = crate::BitReader<Eint0>;
impl Eint0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eint0 {
        match self.bits {
            false => Eint0::B0x0,
            true => Eint0::B0x1,
        }
    }
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eint0::B0x0
    }
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eint0::B0x1
    }
}
#[doc = "Field `EINT0` writer - Enable interrupt line 0"]
pub type Eint0W<'a, REG> = crate::BitWriter<'a, REG, Eint0>;
impl<'a, REG> Eint0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eint0::B0x0)
    }
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eint0::B0x1)
    }
}
#[doc = "Enable interrupt line 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eint1 {
    #[doc = "0: Interrupt line fdcan_intr0_it disabled"]
    B0x0 = 0,
    #[doc = "1: Interrupt line fdcan_intr0_it enabled"]
    B0x1 = 1,
}
impl From<Eint1> for bool {
    #[inline(always)]
    fn from(variant: Eint1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT1` reader - Enable interrupt line 1"]
pub type Eint1R = crate::BitReader<Eint1>;
impl Eint1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eint1 {
        match self.bits {
            false => Eint1::B0x0,
            true => Eint1::B0x1,
        }
    }
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eint1::B0x0
    }
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eint1::B0x1
    }
}
#[doc = "Field `EINT1` writer - Enable interrupt line 1"]
pub type Eint1W<'a, REG> = crate::BitWriter<'a, REG, Eint1>;
impl<'a, REG> Eint1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eint1::B0x0)
    }
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eint1::B0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> Eint0R {
        Eint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> Eint1R {
        Eint1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILE")
            .field("eint0", &self.eint0())
            .field("eint1", &self.eint1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    pub fn eint0(&mut self) -> Eint0W<FdcanIleSpec> {
        Eint0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    pub fn eint1(&mut self) -> Eint1W<FdcanIleSpec> {
        Eint1W::new(self, 1)
    }
}
#[doc = "FDCAN interrupt line enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIleSpec;
impl crate::RegisterSpec for FdcanIleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ile::R`](R) reader structure"]
impl crate::Readable for FdcanIleSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ile::W`](W) writer structure"]
impl crate::Writable for FdcanIleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_ILE to value 0"]
impl crate::Resettable for FdcanIleSpec {}
