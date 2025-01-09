#[doc = "Register `FDCAN_TOCV` reader"]
pub type R = crate::R<FdcanTocvSpec>;
#[doc = "Register `FDCAN_TOCV` writer"]
pub type W = crate::W<FdcanTocvSpec>;
#[doc = "Field `TOC` reader - Timeout counter"]
pub type TocR = crate::FieldReader<u16>;
#[doc = "Field `TOC` writer - Timeout counter"]
pub type TocW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout counter"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TOCV")
            .field("toc", &self.toc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout counter"]
    #[inline(always)]
    pub fn toc(&mut self) -> TocW<FdcanTocvSpec> {
        TocW::new(self, 0)
    }
}
#[doc = "FDCAN timeout counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTocvSpec;
impl crate::RegisterSpec for FdcanTocvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tocv::R`](R) reader structure"]
impl crate::Readable for FdcanTocvSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tocv::W`](W) writer structure"]
impl crate::Writable for FdcanTocvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TOCV to value 0xffff"]
impl crate::Resettable for FdcanTocvSpec {
    const RESET_VALUE: u32 = 0xffff;
}
