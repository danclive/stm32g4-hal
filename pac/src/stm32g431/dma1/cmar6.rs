#[doc = "Register `CMAR6` reader"]
pub type R = crate::R<Cmar6Spec>;
#[doc = "Register `CMAR6` writer"]
pub type W = crate::W<Cmar6Spec>;
#[doc = "Field `MA` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMAR6").field("ma", &self.ma()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MaW<Cmar6Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA channel x memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmar6Spec;
impl crate::RegisterSpec for Cmar6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar6::R`](R) reader structure"]
impl crate::Readable for Cmar6Spec {}
#[doc = "`write(|w| ..)` method takes [`cmar6::W`](W) writer structure"]
impl crate::Writable for Cmar6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMAR6 to value 0"]
impl crate::Resettable for Cmar6Spec {
    const RESET_VALUE: u32 = 0;
}
