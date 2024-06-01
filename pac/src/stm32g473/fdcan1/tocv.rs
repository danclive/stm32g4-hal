#[doc = "Register `TOCV` reader"]
pub type R = crate::R<TocvSpec>;
#[doc = "Field `TOC` reader - TOC"]
pub type TocR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOCV").field("toc", &self.toc()).finish()
    }
}
#[doc = "FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TocvSpec;
impl crate::RegisterSpec for TocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocv::R`](R) reader structure"]
impl crate::Readable for TocvSpec {}
#[doc = "`reset()` method sets TOCV to value 0xffff"]
impl crate::Resettable for TocvSpec {
    const RESET_VALUE: u32 = 0xffff;
}
