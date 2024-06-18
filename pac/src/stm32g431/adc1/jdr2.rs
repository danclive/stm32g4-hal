#[doc = "Register `JDR2` reader"]
pub type R = crate::R<Jdr2Spec>;
#[doc = "Field `JDATA2` reader - JDATA2"]
pub type Jdata2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA2"]
    #[inline(always)]
    pub fn jdata2(&self) -> Jdata2R {
        Jdata2R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR2")
            .field("jdata2", &self.jdata2())
            .finish()
    }
}
#[doc = "injected data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr2Spec;
impl crate::RegisterSpec for Jdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr2::R`](R) reader structure"]
impl crate::Readable for Jdr2Spec {}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for Jdr2Spec {
    const RESET_VALUE: u32 = 0;
}
