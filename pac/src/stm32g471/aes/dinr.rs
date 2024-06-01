#[doc = "Register `DINR` reader"]
pub type R = crate::R<DinrSpec>;
#[doc = "Register `DINR` writer"]
pub type W = crate::W<DinrSpec>;
#[doc = "Field `AES_DINR` reader - Data Input Register"]
pub type AesDinrR = crate::FieldReader<u32>;
#[doc = "Field `AES_DINR` writer - Data Input Register"]
pub type AesDinrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Input Register"]
    #[inline(always)]
    pub fn aes_dinr(&self) -> AesDinrR {
        AesDinrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR")
            .field("aes_dinr", &self.aes_dinr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Input Register"]
    #[inline(always)]
    #[must_use]
    pub fn aes_dinr(&mut self) -> AesDinrW<DinrSpec> {
        AesDinrW::new(self, 0)
    }
}
#[doc = "data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinrSpec;
impl crate::RegisterSpec for DinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr::R`](R) reader structure"]
impl crate::Readable for DinrSpec {}
#[doc = "`write(|w| ..)` method takes [`dinr::W`](W) writer structure"]
impl crate::Writable for DinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DinrSpec {
    const RESET_VALUE: u32 = 0;
}
