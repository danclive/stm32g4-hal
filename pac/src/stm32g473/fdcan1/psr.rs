#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Field `LEC` reader - LEC"]
pub type LecR = crate::FieldReader;
#[doc = "Field `LEC` writer - LEC"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACT` reader - ACT"]
pub type ActR = crate::FieldReader;
#[doc = "Field `EP` reader - EP"]
pub type EpR = crate::BitReader;
#[doc = "Field `EW` reader - EW"]
pub type EwR = crate::BitReader;
#[doc = "Field `BO` reader - BO"]
pub type BoR = crate::BitReader;
#[doc = "Field `DLEC` reader - DLEC"]
pub type DlecR = crate::FieldReader;
#[doc = "Field `DLEC` writer - DLEC"]
pub type DlecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESI` reader - RESI"]
pub type ResiR = crate::BitReader;
#[doc = "Field `RESI` writer - RESI"]
pub type ResiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRS` reader - RBRS"]
pub type RbrsR = crate::BitReader;
#[doc = "Field `RBRS` writer - RBRS"]
pub type RbrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDL` reader - REDL"]
pub type RedlR = crate::BitReader;
#[doc = "Field `REDL` writer - REDL"]
pub type RedlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXE` reader - PXE"]
pub type PxeR = crate::BitReader;
#[doc = "Field `PXE` writer - PXE"]
pub type PxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCV` reader - TDCV"]
pub type TdcvR = crate::FieldReader;
#[doc = "Field `TDCV` writer - TDCV"]
pub type TdcvW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - ACT"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - DLEC"]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&self) -> RedlR {
        RedlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("lec", &self.lec())
            .field("act", &self.act())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("dlec", &self.dlec())
            .field("resi", &self.resi())
            .field("rbrs", &self.rbrs())
            .field("redl", &self.redl())
            .field("pxe", &self.pxe())
            .field("tdcv", &self.tdcv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<PsrSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bits 8:10 - DLEC"]
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DlecW<PsrSpec> {
        DlecW::new(self, 8)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> ResiW<PsrSpec> {
        ResiW::new(self, 11)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RbrsW<PsrSpec> {
        RbrsW::new(self, 12)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> RedlW<PsrSpec> {
        RedlW::new(self, 13)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PxeW<PsrSpec> {
        PxeW::new(self, 14)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    #[must_use]
    pub fn tdcv(&mut self) -> TdcvW<PsrSpec> {
        TdcvW::new(self, 16)
    }
}
#[doc = "FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x0707;
}
