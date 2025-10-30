#[doc = "Register `SHIFTR` writer"]
pub type W = crate::W<ShiftrSpec>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second"]
pub type SubfsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADD1S` writer - Add one second"]
pub type Add1sW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ShiftrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    pub fn subfs(&mut self) -> SubfsW<'_, ShiftrSpec> {
        SubfsW::new(self, 0)
    }
    #[doc = "Bit 31 - Add one second"]
    #[inline(always)]
    pub fn add1s(&mut self) -> Add1sW<'_, ShiftrSpec> {
        Add1sW::new(self, 31)
    }
}
#[doc = "shift control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftrSpec;
impl crate::RegisterSpec for ShiftrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftr::W`](W) writer structure"]
impl crate::Writable for ShiftrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHIFTR to value 0"]
impl crate::Resettable for ShiftrSpec {}
