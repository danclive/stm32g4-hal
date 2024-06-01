#[doc = "Register `JDR4` reader"]
pub type R = crate::R<Jdr4Spec>;
#[doc = "Field `JDATA4` reader - JDATA4"]
pub type Jdata4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA4"]
    #[inline(always)]
    pub fn jdata4(&self) -> Jdata4R {
        Jdata4R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDR4")
            .field("jdata4", &self.jdata4())
            .finish()
    }
}
#[doc = "injected data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr4Spec;
impl crate::RegisterSpec for Jdr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr4::R`](R) reader structure"]
impl crate::Readable for Jdr4Spec {}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for Jdr4Spec {
    const RESET_VALUE: u32 = 0;
}
