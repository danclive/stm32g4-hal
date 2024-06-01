#[doc = "Register `MCNTR` reader"]
pub type R = crate::R<McntrSpec>;
#[doc = "Register `MCNTR` writer"]
pub type W = crate::W<McntrSpec>;
#[doc = "Field `MCNT` reader - Counter value"]
pub type McntR = crate::FieldReader<u16>;
#[doc = "Field `MCNT` writer - Counter value"]
pub type McntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&self) -> McntR {
        McntR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCNTR").field("mcnt", &self.mcnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> McntW<McntrSpec> {
        McntW::new(self, 0)
    }
}
#[doc = "Master Timer Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McntrSpec;
impl crate::RegisterSpec for McntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcntr::R`](R) reader structure"]
impl crate::Readable for McntrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcntr::W`](W) writer structure"]
impl crate::Writable for McntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCNTR to value 0"]
impl crate::Resettable for McntrSpec {
    const RESET_VALUE: u32 = 0;
}
