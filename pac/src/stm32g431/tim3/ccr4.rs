#[doc = "Register `CCR4` reader"]
pub type R = crate::R<Ccr4Spec>;
#[doc = "Register `CCR4` writer"]
pub type W = crate::W<Ccr4Spec>;
#[doc = "Field `CCR4` reader - Capture/Compare value"]
pub type Ccr4R = crate::FieldReader<u32>;
#[doc = "Field `CCR4` writer - Capture/Compare value"]
pub type Ccr4W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4(&self) -> Ccr4R {
        Ccr4R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR4").field("ccr4", &self.ccr4()).finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4(&mut self) -> Ccr4W<Ccr4Spec> {
        Ccr4W::new(self, 0)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr4Spec;
impl crate::RegisterSpec for Ccr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr4::R`](R) reader structure"]
impl crate::Readable for Ccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr4::W`](W) writer structure"]
impl crate::Writable for Ccr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for Ccr4Spec {}
