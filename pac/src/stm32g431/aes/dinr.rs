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
    pub fn aes_dinr(&mut self) -> AesDinrW<DinrSpec> {
        AesDinrW::new(self, 0)
    }
}
#[doc = "data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinrSpec;
impl crate::RegisterSpec for DinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinr::R`](R) reader structure"]
impl crate::Readable for DinrSpec {}
#[doc = "`write(|w| ..)` method takes [`dinr::W`](W) writer structure"]
impl crate::Writable for DinrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DinrSpec {}
