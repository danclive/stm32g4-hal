#[doc = "Register `CNDTR4` reader"]
pub type R = crate::R<Cndtr4Spec>;
#[doc = "Register `CNDTR4` writer"]
pub type W = crate::W<Cndtr4Spec>;
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
        f.debug_struct("CNDTR4").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<'_, Cndtr4Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "channel x number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr4Spec;
impl crate::RegisterSpec for Cndtr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr4::R`](R) reader structure"]
impl crate::Readable for Cndtr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr4::W`](W) writer structure"]
impl crate::Writable for Cndtr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDTR4 to value 0"]
impl crate::Resettable for Cndtr4Spec {}
