#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CMPMCF` writer - compare match Clear Flag"]
pub type CmpmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub type ArrmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub type ExttrigcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKCF` writer - Compare register update OK Clear Flag"]
pub type CmpokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub type ArrokcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub type UpcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub type DowncfW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - compare match Clear Flag"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CmpmcfW<IcrSpec> {
        CmpmcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ArrmcfW<IcrSpec> {
        ArrmcfW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> ExttrigcfW<IcrSpec> {
        ExttrigcfW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CmpokcfW<IcrSpec> {
        CmpokcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ArrokcfW<IcrSpec> {
        ArrokcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UpcfW<IcrSpec> {
        UpcfW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DowncfW<IcrSpec> {
        DowncfW::new(self, 6)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
