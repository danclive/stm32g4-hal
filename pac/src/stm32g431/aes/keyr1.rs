#[doc = "Register `KEYR1` reader"]
pub type R = crate::R<Keyr1Spec>;
#[doc = "Register `KEYR1` writer"]
pub type W = crate::W<Keyr1Spec>;
#[doc = "Field `AES_KEYR1` reader - AES key register (key \\[63:32\\])"]
pub type AesKeyr1R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR1` writer - AES key register (key \\[63:32\\])"]
pub type AesKeyr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key register (key \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_keyr1(&self) -> AesKeyr1R {
        AesKeyr1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR1")
            .field("aes_keyr1", &self.aes_keyr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (key \\[63:32\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr1(&mut self) -> AesKeyr1W<Keyr1Spec> {
        AesKeyr1W::new(self, 0)
    }
}
#[doc = "key register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr1Spec;
impl crate::RegisterSpec for Keyr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr1::R`](R) reader structure"]
impl crate::Readable for Keyr1Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr1::W`](W) writer structure"]
impl crate::Writable for Keyr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR1 to value 0"]
impl crate::Resettable for Keyr1Spec {
    const RESET_VALUE: u32 = 0;
}
