#[doc = "Register `KEYR0` reader"]
pub type R = crate::R<Keyr0Spec>;
#[doc = "Register `KEYR0` writer"]
pub type W = crate::W<Keyr0Spec>;
#[doc = "Field `AES_KEYR0` reader - Data Output Register (LSB key \\[31:0\\])"]
pub type AesKeyr0R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEYR0` writer - Data Output Register (LSB key \\[31:0\\])"]
pub type AesKeyr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    pub fn aes_keyr0(&self) -> AesKeyr0R {
        AesKeyr0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR0")
            .field("aes_keyr0", &self.aes_keyr0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr0(&mut self) -> AesKeyr0W<Keyr0Spec> {
        AesKeyr0W::new(self, 0)
    }
}
#[doc = "key register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr0Spec;
impl crate::RegisterSpec for Keyr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr0::R`](R) reader structure"]
impl crate::Readable for Keyr0Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr0::W`](W) writer structure"]
impl crate::Writable for Keyr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR0 to value 0"]
impl crate::Resettable for Keyr0Spec {
    const RESET_VALUE: u32 = 0;
}
