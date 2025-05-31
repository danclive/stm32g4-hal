#[doc = "Register `KR` writer"]
pub type W = crate::W<KrSpec>;
#[doc = "Field `KEY` writer - Key value (write only, read 0x0000)"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<KrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Key value (write only, read 0x0000)"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<KrSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrSpec;
impl crate::RegisterSpec for KrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`kr::W`](W) writer structure"]
impl crate::Writable for KrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KrSpec {}
