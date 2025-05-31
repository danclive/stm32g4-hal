#[doc = "Register `KEYR3` reader"]
pub type R = crate::R<Keyr3Spec>;
#[doc = "Register `KEYR3` writer"]
pub type W = crate::W<Keyr3Spec>;
#[doc = "Field `AES_KEYR3` reader - AES key register (MSB key \\[127:96\\])"]
pub type AesKeyr3R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR3` writer - AES key register (MSB key \\[127:96\\])"]
pub type AesKeyr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn aes_keyr3(&self) -> AesKeyr3R {
        AesKeyr3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR3")
            .field("aes_keyr3", &self.aes_keyr3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn aes_keyr3(&mut self) -> AesKeyr3W<Keyr3Spec> {
        AesKeyr3W::new(self, 0)
    }
}
#[doc = "key register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr3Spec;
impl crate::RegisterSpec for Keyr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr3::R`](R) reader structure"]
impl crate::Readable for Keyr3Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr3::W`](W) writer structure"]
impl crate::Writable for Keyr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYR3 to value 0"]
impl crate::Resettable for Keyr3Spec {}
