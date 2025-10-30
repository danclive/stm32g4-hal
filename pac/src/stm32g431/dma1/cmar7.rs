#[doc = "Register `CMAR7` reader"]
pub type R = crate::R<Cmar7Spec>;
#[doc = "Register `CMAR7` writer"]
pub type W = crate::W<Cmar7Spec>;
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
        f.debug_struct("CMAR7").field("ma", &self.ma()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<'_, Cmar7Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmar7Spec;
impl crate::RegisterSpec for Cmar7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar7::R`](R) reader structure"]
impl crate::Readable for Cmar7Spec {}
#[doc = "`write(|w| ..)` method takes [`cmar7::W`](W) writer structure"]
impl crate::Writable for Cmar7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMAR7 to value 0"]
impl crate::Resettable for Cmar7Spec {}
