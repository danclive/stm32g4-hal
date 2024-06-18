#[doc = "Register `KEYR2` reader"]
pub type R = crate::R<Keyr2Spec>;
#[doc = "Register `KEYR2` writer"]
pub type W = crate::W<Keyr2Spec>;
#[doc = "Field `AES_KEYR2` reader - AES key register (key \\[95:64\\])"]
pub type AesKeyr2R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR2` writer - AES key register (key \\[95:64\\])"]
pub type AesKeyr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key register (key \\[95:64\\])"]
    #[inline(always)]
    pub fn aes_keyr2(&self) -> AesKeyr2R {
        AesKeyr2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR2")
            .field("aes_keyr2", &self.aes_keyr2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (key \\[95:64\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr2(&mut self) -> AesKeyr2W<Keyr2Spec> {
        AesKeyr2W::new(self, 0)
    }
}
#[doc = "key register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr2Spec;
impl crate::RegisterSpec for Keyr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr2::R`](R) reader structure"]
impl crate::Readable for Keyr2Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr2::W`](W) writer structure"]
impl crate::Writable for Keyr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR2 to value 0"]
impl crate::Resettable for Keyr2Spec {
    const RESET_VALUE: u32 = 0;
}
