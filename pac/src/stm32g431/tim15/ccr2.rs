#[doc = "Register `CCR2` reader"]
pub type R = crate::R<Ccr2Spec>;
#[doc = "Register `CCR2` writer"]
pub type W = crate::W<Ccr2Spec>;
#[doc = "Field `CCR2` reader - Capture/Compare 1 value"]
pub type Ccr2R = crate::FieldReader<u32>;
#[doc = "Field `CCR2` writer - Capture/Compare 1 value"]
pub type Ccr2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr2(&self) -> Ccr2R {
        Ccr2R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR2").field("ccr2", &self.ccr2()).finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr2(&mut self) -> Ccr2W<Ccr2Spec> {
        Ccr2W::new(self, 0)
    }
}
#[doc = "capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr2Spec;
impl crate::RegisterSpec for Ccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2::R`](R) reader structure"]
impl crate::Readable for Ccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr2::W`](W) writer structure"]
impl crate::Writable for Ccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR2 to value 0"]
impl crate::Resettable for Ccr2Spec {}
