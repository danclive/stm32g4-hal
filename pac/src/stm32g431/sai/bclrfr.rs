#[doc = "Register `BCLRFR` writer"]
pub type W = crate::W<BclrfrSpec>;
#[doc = "Field `OVRUDR` writer - Clear overrun / underrun"]
pub type OvrudrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTEDET` writer - Mute detection flag"]
pub type MutedetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCKCFG` writer - Clear wrong clock configuration flag"]
pub type WckcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNRDY` writer - Clear codec not ready flag"]
pub type CnrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CafsdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSDET` writer - Clear late frame synchronization detection flag"]
pub type LfsdetW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BclrfrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn ovrudr(&mut self) -> OvrudrW<BclrfrSpec> {
        OvrudrW::new(self, 0)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn mutedet(&mut self) -> MutedetW<BclrfrSpec> {
        MutedetW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn wckcfg(&mut self) -> WckcfgW<BclrfrSpec> {
        WckcfgW::new(self, 2)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn cnrdy(&mut self) -> CnrdyW<BclrfrSpec> {
        CnrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CafsdetW<BclrfrSpec> {
        CafsdetW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn lfsdet(&mut self) -> LfsdetW<BclrfrSpec> {
        LfsdetW::new(self, 6)
    }
}
#[doc = "BClear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BclrfrSpec;
impl crate::RegisterSpec for BclrfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bclrfr::W`](W) writer structure"]
impl crate::Writable for BclrfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCLRFR to value 0"]
impl crate::Resettable for BclrfrSpec {
    const RESET_VALUE: u32 = 0;
}
