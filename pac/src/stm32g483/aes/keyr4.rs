#[doc = "Register `KEYR4` reader"]
pub type R = crate::R<Keyr4Spec>;
#[doc = "Register `KEYR4` writer"]
pub type W = crate::W<Keyr4Spec>;
#[doc = "Field `KEY` reader - AES key"]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - AES key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR4").field("key", &self.key()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<Keyr4Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "key register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr4Spec;
impl crate::RegisterSpec for Keyr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr4::R`](R) reader structure"]
impl crate::Readable for Keyr4Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr4::W`](W) writer structure"]
impl crate::Writable for Keyr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR4 to value 0"]
impl crate::Resettable for Keyr4Spec {
    const RESET_VALUE: u32 = 0;
}
