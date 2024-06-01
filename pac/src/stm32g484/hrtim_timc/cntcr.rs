#[doc = "Register `CNTCR` reader"]
pub type R = crate::R<CntcrSpec>;
#[doc = "Register `CNTCR` writer"]
pub type W = crate::W<CntcrSpec>;
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub type CntxR = crate::FieldReader<u16>;
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub type CntxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CntxR {
        CntxR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTCR").field("cntx", &self.cntx()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CntxW<CntcrSpec> {
        CntxW::new(self, 0)
    }
}
#[doc = "Timerx Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntcrSpec;
impl crate::RegisterSpec for CntcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcr::R`](R) reader structure"]
impl crate::Readable for CntcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntcr::W`](W) writer structure"]
impl crate::Writable for CntcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CntcrSpec {
    const RESET_VALUE: u32 = 0;
}
