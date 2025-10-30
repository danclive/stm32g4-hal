#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WdataSpec>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<WdataSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WdataW<'_, WdataSpec> {
        WdataW::new(self, 0)
    }
}
#[doc = "FMAC Write Data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdataSpec;
impl crate::RegisterSpec for WdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WdataSpec {}
