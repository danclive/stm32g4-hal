#[doc = "Register `IVR1` reader"]
pub type R = crate::R<Ivr1Spec>;
#[doc = "Register `IVR1` writer"]
pub type W = crate::W<Ivr1Spec>;
#[doc = "Field `AES_IVR1` reader - Initialization Vector Register (IVR \\[63:32\\])"]
pub type AesIvr1R = crate::FieldReader<u32>;
#[doc = "Field `AES_IVR1` writer - Initialization Vector Register (IVR \\[63:32\\])"]
pub type AesIvr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_ivr1(&self) -> AesIvr1R {
        AesIvr1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR1")
            .field("aes_ivr1", &self.aes_ivr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_ivr1(&mut self) -> AesIvr1W<'_, Ivr1Spec> {
        AesIvr1W::new(self, 0)
    }
}
#[doc = "initialization vector register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ivr1Spec;
impl crate::RegisterSpec for Ivr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr1::R`](R) reader structure"]
impl crate::Readable for Ivr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ivr1::W`](W) writer structure"]
impl crate::Writable for Ivr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IVR1 to value 0"]
impl crate::Resettable for Ivr1Spec {}
