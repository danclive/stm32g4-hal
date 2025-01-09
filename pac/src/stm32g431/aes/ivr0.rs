#[doc = "Register `IVR0` reader"]
pub type R = crate::R<Ivr0Spec>;
#[doc = "Register `IVR0` writer"]
pub type W = crate::W<Ivr0Spec>;
#[doc = "Field `AES_IVR0` reader - initialization vector register (LSB IVR \\[31:0\\])"]
pub type AesIvr0R = crate::FieldReader<u32>;
#[doc = "Field `AES_IVR0` writer - initialization vector register (LSB IVR \\[31:0\\])"]
pub type AesIvr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - initialization vector register (LSB IVR \\[31:0\\])"]
    #[inline(always)]
    pub fn aes_ivr0(&self) -> AesIvr0R {
        AesIvr0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IVR0")
            .field("aes_ivr0", &self.aes_ivr0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - initialization vector register (LSB IVR \\[31:0\\])"]
    #[inline(always)]
    pub fn aes_ivr0(&mut self) -> AesIvr0W<Ivr0Spec> {
        AesIvr0W::new(self, 0)
    }
}
#[doc = "initialization vector register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ivr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ivr0Spec;
impl crate::RegisterSpec for Ivr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivr0::R`](R) reader structure"]
impl crate::Readable for Ivr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ivr0::W`](W) writer structure"]
impl crate::Writable for Ivr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVR0 to value 0"]
impl crate::Resettable for Ivr0Spec {
    const RESET_VALUE: u32 = 0;
}
