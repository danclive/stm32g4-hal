#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CccrSpec>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CccrSpec>;
#[doc = "Field `INIT` reader - INIT"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - INIT"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - CCE"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - CCE"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - ASM"]
pub type AsmR = crate::BitReader;
#[doc = "Field `ASM` writer - ASM"]
pub type AsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - CSA"]
pub type CsaR = crate::BitReader;
#[doc = "Field `CSR` reader - CSR"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSR` writer - CSR"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - MON"]
pub type MonR = crate::BitReader;
#[doc = "Field `MON` writer - MON"]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - DAR"]
pub type DarR = crate::BitReader;
#[doc = "Field `DAR` writer - DAR"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - TEST"]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - TEST"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - FDOE"]
pub type FdoeR = crate::BitReader;
#[doc = "Field `FDOE` writer - FDOE"]
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSE` reader - BRSE"]
pub type BrseR = crate::BitReader;
#[doc = "Field `BRSE` writer - BRSE"]
pub type BrseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - PXHD"]
pub type PxhdR = crate::BitReader;
#[doc = "Field `PXHD` writer - PXHD"]
pub type PxhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - EFBI"]
pub type EfbiR = crate::BitReader;
#[doc = "Field `EFBI` writer - EFBI"]
pub type EfbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - TXP"]
pub type TxpR = crate::BitReader;
#[doc = "Field `TXP` writer - TXP"]
pub type TxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - NISO"]
pub type NisoR = crate::BitReader;
#[doc = "Field `NISO` writer - NISO"]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCE"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM"]
    #[inline(always)]
    pub fn asm(&self) -> AsmR {
        AsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CSA"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSR"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MON"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TEST"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FDOE"]
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRSE"]
    #[inline(always)]
    pub fn brse(&self) -> BrseR {
        BrseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - PXHD"]
    #[inline(always)]
    pub fn pxhd(&self) -> PxhdR {
        PxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EFBI"]
    #[inline(always)]
    pub fn efbi(&self) -> EfbiR {
        EfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NISO"]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<CccrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - CCE"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CceW<CccrSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - ASM"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> AsmW<CccrSpec> {
        AsmW::new(self, 2)
    }
    #[doc = "Bit 4 - CSR"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CsrW<CccrSpec> {
        CsrW::new(self, 4)
    }
    #[doc = "Bit 5 - MON"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MonW<CccrSpec> {
        MonW::new(self, 5)
    }
    #[doc = "Bit 6 - DAR"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DarW<CccrSpec> {
        DarW::new(self, 6)
    }
    #[doc = "Bit 7 - TEST"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TestW<CccrSpec> {
        TestW::new(self, 7)
    }
    #[doc = "Bit 8 - FDOE"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FdoeW<CccrSpec> {
        FdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - BRSE"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BrseW<CccrSpec> {
        BrseW::new(self, 9)
    }
    #[doc = "Bit 12 - PXHD"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PxhdW<CccrSpec> {
        PxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - EFBI"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EfbiW<CccrSpec> {
        EfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TxpW<CccrSpec> {
        TxpW::new(self, 14)
    }
    #[doc = "Bit 15 - NISO"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NisoW<CccrSpec> {
        NisoW::new(self, 15)
    }
}
#[doc = "For details about setting and resetting of single bits see Software initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccrSpec;
impl crate::RegisterSpec for CccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CccrSpec {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CccrSpec {
    const RESET_VALUE: u32 = 0x01;
}
