#[doc = "Register `CCR6` reader"]
pub type R = crate::R<Ccr6Spec>;
#[doc = "Register `CCR6` writer"]
pub type W = crate::W<Ccr6Spec>;
#[doc = "Field `CCR6` reader - Capture/Compare value"]
pub type Ccr6R = crate::FieldReader<u32>;
#[doc = "Field `CCR6` writer - Capture/Compare value"]
pub type Ccr6W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr6(&self) -> Ccr6R {
        Ccr6R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR6").field("ccr6", &self.ccr6()).finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr6(&mut self) -> Ccr6W<Ccr6Spec> {
        Ccr6W::new(self, 0)
    }
}
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr6Spec;
impl crate::RegisterSpec for Ccr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr6::R`](R) reader structure"]
impl crate::Readable for Ccr6Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr6::W`](W) writer structure"]
impl crate::Writable for Ccr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR6 to value 0"]
impl crate::Resettable for Ccr6Spec {}
