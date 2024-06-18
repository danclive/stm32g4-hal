#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `CMPMIE` reader - Compare match Interrupt Enable"]
pub type CmpmieR = crate::BitReader;
#[doc = "Field `CMPMIE` writer - Compare match Interrupt Enable"]
pub type CmpmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ArrmieR = crate::BitReader;
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ArrmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type ExttrigieR = crate::BitReader;
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type ExttrigieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKIE` reader - Compare register update OK Interrupt Enable"]
pub type CmpokieR = crate::BitReader;
#[doc = "Field `CMPOKIE` writer - Compare register update OK Interrupt Enable"]
pub type CmpokieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ArrokieR = crate::BitReader;
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ArrokieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable"]
pub type UpieR = crate::BitReader;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable"]
pub type DownieR = crate::BitReader;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable"]
pub type DownieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CmpmieR {
        CmpmieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ArrmieR {
        ArrmieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> ExttrigieR {
        ExttrigieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CmpokieR {
        CmpokieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ArrokieR {
        ArrokieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&self) -> DownieR {
        DownieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("downie", &self.downie())
            .field("upie", &self.upie())
            .field("arrokie", &self.arrokie())
            .field("cmpokie", &self.cmpokie())
            .field("exttrigie", &self.exttrigie())
            .field("arrmie", &self.arrmie())
            .field("cmpmie", &self.cmpmie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmie(&mut self) -> CmpmieW<IerSpec> {
        CmpmieW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ArrmieW<IerSpec> {
        ArrmieW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> ExttrigieW<IerSpec> {
        ExttrigieW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokie(&mut self) -> CmpokieW<IerSpec> {
        CmpokieW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ArrokieW<IerSpec> {
        ArrokieW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UpieW<IerSpec> {
        UpieW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DownieW<IerSpec> {
        DownieW::new(self, 6)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
