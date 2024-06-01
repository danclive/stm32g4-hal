#[doc = "Register `BMCMPR` reader"]
pub type R = crate::R<BmcmprSpec>;
#[doc = "Register `BMCMPR` writer"]
pub type W = crate::W<BmcmprSpec>;
#[doc = "Field `BMCMP` reader - BMCMP"]
pub type BmcmpR = crate::FieldReader<u16>;
#[doc = "Field `BMCMP` writer - BMCMP"]
pub type BmcmpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BmcmpR {
        BmcmpR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMCMPR")
            .field("bmcmp", &self.bmcmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    #[must_use]
    pub fn bmcmp(&mut self) -> BmcmpW<BmcmprSpec> {
        BmcmpW::new(self, 0)
    }
}
#[doc = "BMCMPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmcmprSpec;
impl crate::RegisterSpec for BmcmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcmpr::R`](R) reader structure"]
impl crate::Readable for BmcmprSpec {}
#[doc = "`write(|w| ..)` method takes [`bmcmpr::W`](W) writer structure"]
impl crate::Writable for BmcmprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMCMPR to value 0"]
impl crate::Resettable for BmcmprSpec {
    const RESET_VALUE: u32 = 0;
}
