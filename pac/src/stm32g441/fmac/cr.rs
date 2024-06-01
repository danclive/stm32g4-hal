#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RIEN` reader - RIEN"]
pub type RienR = crate::BitReader;
#[doc = "Field `RIEN` writer - RIEN"]
pub type RienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIEN` reader - WIEN"]
pub type WienR = crate::BitReader;
#[doc = "Field `WIEN` writer - WIEN"]
pub type WienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFLIEN` reader - OVFLIEN"]
pub type OvflienR = crate::BitReader;
#[doc = "Field `OVFLIEN` writer - OVFLIEN"]
pub type OvflienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNFLIEN` reader - UNFLIEN"]
pub type UnflienR = crate::BitReader;
#[doc = "Field `UNFLIEN` writer - UNFLIEN"]
pub type UnflienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATIEN` reader - SATIEN"]
pub type SatienR = crate::BitReader;
#[doc = "Field `SATIEN` writer - SATIEN"]
pub type SatienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - DMAREN"]
pub type DmarenR = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMAREN"]
pub type DmarenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub type DmawenR = crate::BitReader;
#[doc = "Field `DMAWEN` writer - DMAWEN"]
pub type DmawenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLIPEN` reader - CLIPEN"]
pub type ClipenR = crate::BitReader;
#[doc = "Field `CLIPEN` writer - CLIPEN"]
pub type ClipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - RESET"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - RESET"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RIEN"]
    #[inline(always)]
    pub fn rien(&self) -> RienR {
        RienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WIEN"]
    #[inline(always)]
    pub fn wien(&self) -> WienR {
        WienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OVFLIEN"]
    #[inline(always)]
    pub fn ovflien(&self) -> OvflienR {
        OvflienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UNFLIEN"]
    #[inline(always)]
    pub fn unflien(&self) -> UnflienR {
        UnflienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SATIEN"]
    #[inline(always)]
    pub fn satien(&self) -> SatienR {
        SatienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DmarenR {
        DmarenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DmawenR {
        DmawenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - CLIPEN"]
    #[inline(always)]
    pub fn clipen(&self) -> ClipenR {
        ClipenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("reset", &self.reset())
            .field("clipen", &self.clipen())
            .field("dmawen", &self.dmawen())
            .field("dmaren", &self.dmaren())
            .field("satien", &self.satien())
            .field("unflien", &self.unflien())
            .field("ovflien", &self.ovflien())
            .field("wien", &self.wien())
            .field("rien", &self.rien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RIEN"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RienW<CrSpec> {
        RienW::new(self, 0)
    }
    #[doc = "Bit 1 - WIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wien(&mut self) -> WienW<CrSpec> {
        WienW::new(self, 1)
    }
    #[doc = "Bit 2 - OVFLIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ovflien(&mut self) -> OvflienW<CrSpec> {
        OvflienW::new(self, 2)
    }
    #[doc = "Bit 3 - UNFLIEN"]
    #[inline(always)]
    #[must_use]
    pub fn unflien(&mut self) -> UnflienW<CrSpec> {
        UnflienW::new(self, 3)
    }
    #[doc = "Bit 4 - SATIEN"]
    #[inline(always)]
    #[must_use]
    pub fn satien(&mut self) -> SatienW<CrSpec> {
        SatienW::new(self, 4)
    }
    #[doc = "Bit 8 - DMAREN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DmarenW<CrSpec> {
        DmarenW::new(self, 8)
    }
    #[doc = "Bit 9 - DMAWEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DmawenW<CrSpec> {
        DmawenW::new(self, 9)
    }
    #[doc = "Bit 15 - CLIPEN"]
    #[inline(always)]
    #[must_use]
    pub fn clipen(&mut self) -> ClipenW<CrSpec> {
        ClipenW::new(self, 15)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CrSpec> {
        ResetW::new(self, 16)
    }
}
#[doc = "FMAC Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
