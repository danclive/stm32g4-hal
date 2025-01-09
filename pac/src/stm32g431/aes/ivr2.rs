#[doc = "Register `IVR2` reader"]
pub type R = crate::R<Ivr2Spec>;
#[doc = "Register `IVR2` writer"]
pub type W = crate::W<Ivr2Spec>;
#[doc = "Field `AES_IVR2` reader - Initialization Vector Register (IVR \\[95:64\\])"]
pub type AesIvr2R = crate::FieldReader<u32>;
#[doc = "Field `AES_IVR2` writer - Initialization Vector Register (IVR \\[95:64\\])"]
pub type AesIvr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[95:64\\])"]
    #[inline(always)]
    pub fn aes_ivr2(&self) -> AesIvr2R {
        AesIvr2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR2")
            .field("aes_ivr2", &self.aes_ivr2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[95:64\\])"]
    #[inline(always)]
    pub fn aes_ivr2(&mut self) -> AesIvr2W<Ivr2Spec> {
        AesIvr2W::new(self, 0)
    }
}
#[doc = "initialization vector register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ivr2Spec;
impl crate::RegisterSpec for Ivr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr2::R`](R) reader structure"]
impl crate::Readable for Ivr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ivr2::W`](W) writer structure"]
impl crate::Writable for Ivr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVR2 to value 0"]
impl crate::Resettable for Ivr2Spec {
    const RESET_VALUE: u32 = 0;
}
