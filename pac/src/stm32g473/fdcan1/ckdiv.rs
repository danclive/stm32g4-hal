#[doc = "Register `CKDIV` reader"]
pub type R = crate::R<CkdivSpec>;
#[doc = "Register `CKDIV` writer"]
pub type W = crate::W<CkdivSpec>;
#[doc = "Field `PDIV` reader - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
pub type PdivR = crate::FieldReader;
#[doc = "Field `PDIV` writer - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKDIV").field("pdiv", &self.pdiv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - input clock divider. the APB clock could be divided prior to be used by the CAN sub"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PdivW<CkdivSpec> {
        PdivW::new(self, 0)
    }
}
#[doc = "FDCAN CFG clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkdivSpec;
impl crate::RegisterSpec for CkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckdiv::R`](R) reader structure"]
impl crate::Readable for CkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`ckdiv::W`](W) writer structure"]
impl crate::Writable for CkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKDIV to value 0"]
impl crate::Resettable for CkdivSpec {
    const RESET_VALUE: u32 = 0;
}
