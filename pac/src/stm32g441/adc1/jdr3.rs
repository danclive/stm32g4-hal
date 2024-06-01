#[doc = "Register `JDR3` reader"]
pub type R = crate::R<Jdr3Spec>;
#[doc = "Field `JDATA3` reader - JDATA3"]
pub type Jdata3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA3"]
    #[inline(always)]
    pub fn jdata3(&self) -> Jdata3R {
        Jdata3R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR3")
            .field("jdata3", &self.jdata3())
            .finish()
    }
}
#[doc = "injected data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr3Spec;
impl crate::RegisterSpec for Jdr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr3::R`](R) reader structure"]
impl crate::Readable for Jdr3Spec {}
#[doc = "`reset()` method sets JDR3 to value 0"]
impl crate::Resettable for Jdr3Spec {
    const RESET_VALUE: u32 = 0;
}
