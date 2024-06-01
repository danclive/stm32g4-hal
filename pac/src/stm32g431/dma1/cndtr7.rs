#[doc = "Register `CNDTR7` reader"]
pub type R = crate::R<Cndtr7Spec>;
#[doc = "Register `CNDTR7` writer"]
pub type W = crate::W<Cndtr7Spec>;
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNDTR7").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NdtW<Cndtr7Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "channel x number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr7Spec;
impl crate::RegisterSpec for Cndtr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr7::R`](R) reader structure"]
impl crate::Readable for Cndtr7Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr7::W`](W) writer structure"]
impl crate::Writable for Cndtr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNDTR7 to value 0"]
impl crate::Resettable for Cndtr7Spec {
    const RESET_VALUE: u32 = 0;
}
