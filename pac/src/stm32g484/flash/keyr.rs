#[doc = "Register `KEYR` writer"]
pub type W = crate::W<KeyrSpec>;
#[doc = "Field `KEYR` writer - KEYR"]
pub type KeyrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KeyrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - KEYR"]
    #[inline(always)]
    #[must_use]
    pub fn keyr(&mut self) -> KeyrW<KeyrSpec> {
        KeyrW::new(self, 0)
    }
}
#[doc = "Flash key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyrSpec;
impl crate::RegisterSpec for KeyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr::W`](W) writer structure"]
impl crate::Writable for KeyrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR to value 0"]
impl crate::Resettable for KeyrSpec {
    const RESET_VALUE: u32 = 0;
}
