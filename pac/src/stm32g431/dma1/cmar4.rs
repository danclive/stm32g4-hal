#[doc = "Register `CMAR4` reader"]
pub type R = crate::R<Cmar4Spec>;
#[doc = "Register `CMAR4` writer"]
pub type W = crate::W<Cmar4Spec>;
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
        f.debug_struct("CMAR4").field("ma", &self.ma()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<Cmar4Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmar4Spec;
impl crate::RegisterSpec for Cmar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar4::R`](R) reader structure"]
impl crate::Readable for Cmar4Spec {}
#[doc = "`write(|w| ..)` method takes [`cmar4::W`](W) writer structure"]
impl crate::Writable for Cmar4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMAR4 to value 0"]
impl crate::Resettable for Cmar4Spec {
    const RESET_VALUE: u32 = 0;
}
