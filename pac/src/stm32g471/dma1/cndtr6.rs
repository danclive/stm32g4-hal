#[doc = "Register `CNDTR6` reader"]
pub type R = crate::R<Cndtr6Spec>;
#[doc = "Register `CNDTR6` writer"]
pub type W = crate::W<Cndtr6Spec>;
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
        f.debug_struct("CNDTR6").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NdtW<Cndtr6Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "channel x number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr6Spec;
impl crate::RegisterSpec for Cndtr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr6::R`](R) reader structure"]
impl crate::Readable for Cndtr6Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr6::W`](W) writer structure"]
impl crate::Writable for Cndtr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNDTR6 to value 0"]
impl crate::Resettable for Cndtr6Spec {
    const RESET_VALUE: u32 = 0;
}
