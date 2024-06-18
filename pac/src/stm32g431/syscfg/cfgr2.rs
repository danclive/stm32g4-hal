#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `CLL` reader - Core Lockup Lock"]
pub type CllR = crate::BitReader;
#[doc = "Field `CLL` writer - Core Lockup Lock"]
pub type CllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` reader - SRAM Parity Lock"]
pub type SplR = crate::BitReader;
#[doc = "Field `SPL` writer - SRAM Parity Lock"]
pub type SplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDL` reader - PVD Lock"]
pub type PvdlR = crate::BitReader;
#[doc = "Field `PVDL` writer - PVD Lock"]
pub type PvdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCL` reader - ECC Lock"]
pub type EcclR = crate::BitReader;
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type EcclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPF` reader - SRAM Parity Flag"]
pub type SpfR = crate::BitReader;
#[doc = "Field `SPF` writer - SRAM Parity Flag"]
pub type SpfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core Lockup Lock"]
    #[inline(always)]
    pub fn cll(&self) -> CllR {
        CllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Parity Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SplR {
        SplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD Lock"]
    #[inline(always)]
    pub fn pvdl(&self) -> PvdlR {
        PvdlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&self) -> EcclR {
        EcclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Flag"]
    #[inline(always)]
    pub fn spf(&self) -> SpfR {
        SpfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("cll", &self.cll())
            .field("spl", &self.spl())
            .field("pvdl", &self.pvdl())
            .field("eccl", &self.eccl())
            .field("spf", &self.spf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core Lockup Lock"]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CllW<Cfgr2Spec> {
        CllW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM Parity Lock"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SplW<Cfgr2Spec> {
        SplW::new(self, 1)
    }
    #[doc = "Bit 2 - PVD Lock"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PvdlW<Cfgr2Spec> {
        PvdlW::new(self, 2)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> EcclW<Cfgr2Spec> {
        EcclW::new(self, 3)
    }
    #[doc = "Bit 8 - SRAM Parity Flag"]
    #[inline(always)]
    #[must_use]
    pub fn spf(&mut self) -> SpfW<Cfgr2Spec> {
        SpfW::new(self, 8)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {
    const RESET_VALUE: u32 = 0;
}
