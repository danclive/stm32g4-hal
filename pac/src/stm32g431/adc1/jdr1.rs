#[doc = "Register `JDR1` reader"]
pub type R = crate::R<Jdr1Spec>;
#[doc = "Field `JDATA1` reader - JDATA1"]
pub type Jdata1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA1"]
    #[inline(always)]
    pub fn jdata1(&self) -> Jdata1R {
        Jdata1R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR1")
            .field("jdata1", &self.jdata1())
            .finish()
    }
}
#[doc = "injected data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr1Spec;
impl crate::RegisterSpec for Jdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr1::R`](R) reader structure"]
impl crate::Readable for Jdr1Spec {}
#[doc = "`reset()` method sets JDR1 to value 0"]
impl crate::Resettable for Jdr1Spec {
    const RESET_VALUE: u32 = 0;
}
