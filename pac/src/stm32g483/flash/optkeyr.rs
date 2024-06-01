#[doc = "Register `OPTKEYR` writer"]
pub type W = crate::W<OptkeyrSpec>;
#[doc = "Field `OPTKEYR` writer - Option byte key"]
pub type OptkeyrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OptkeyrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    #[must_use]
    pub fn optkeyr(&mut self) -> OptkeyrW<OptkeyrSpec> {
        OptkeyrW::new(self, 0)
    }
}
#[doc = "Option byte key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptkeyrSpec;
impl crate::RegisterSpec for OptkeyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure"]
impl crate::Writable for OptkeyrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTKEYR to value 0"]
impl crate::Resettable for OptkeyrSpec {
    const RESET_VALUE: u32 = 0;
}
