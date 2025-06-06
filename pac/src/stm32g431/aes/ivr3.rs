#[doc = "Register `IVR3` reader"]
pub type R = crate::R<Ivr3Spec>;
#[doc = "Register `IVR3` writer"]
pub type W = crate::W<Ivr3Spec>;
#[doc = "Field `AES_IVR3` reader - Initialization Vector Register (MSB IVR \\[127:96\\])"]
pub type AesIvr3R = crate::FieldReader<u32>;
#[doc = "Field `AES_IVR3` writer - Initialization Vector Register (MSB IVR \\[127:96\\])"]
pub type AesIvr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn aes_ivr3(&self) -> AesIvr3R {
        AesIvr3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR3")
            .field("aes_ivr3", &self.aes_ivr3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn aes_ivr3(&mut self) -> AesIvr3W<Ivr3Spec> {
        AesIvr3W::new(self, 0)
    }
}
#[doc = "initialization vector register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ivr3Spec;
impl crate::RegisterSpec for Ivr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr3::R`](R) reader structure"]
impl crate::Readable for Ivr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ivr3::W`](W) writer structure"]
impl crate::Writable for Ivr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IVR3 to value 0"]
impl crate::Resettable for Ivr3Spec {}
