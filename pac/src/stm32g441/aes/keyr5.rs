#[doc = "Register `KEYR5` reader"]
pub type R = crate::R<Keyr5Spec>;
#[doc = "Register `KEYR5` writer"]
pub type W = crate::W<Keyr5Spec>;
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
        f.debug_struct("KEYR5").field("key", &self.key()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<Keyr5Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "key register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr5Spec;
impl crate::RegisterSpec for Keyr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr5::R`](R) reader structure"]
impl crate::Readable for Keyr5Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr5::W`](W) writer structure"]
impl crate::Writable for Keyr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYR5 to value 0"]
impl crate::Resettable for Keyr5Spec {
    const RESET_VALUE: u32 = 0;
}
