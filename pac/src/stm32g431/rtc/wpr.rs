#[doc = "Register `WPR` writer"]
pub type W = crate::W<WprSpec>;
#[doc = "Field `KEY` writer - Write protection key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<WprSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, WprSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WprSpec;
impl crate::RegisterSpec for WprSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WprSpec {}
