#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNT` reader - counter value"]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - counter value"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `UIFCPY` reader - UIFCPY"]
pub type UifcpyR = crate::BitReader;
#[doc = "Field `UIFCPY` writer - UIFCPY"]
pub type UifcpyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - UIFCPY"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UifcpyR {
        UifcpyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("uifcpy", &self.uifcpy())
            .field("cnt", &self.cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CntSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 31 - UIFCPY"]
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UifcpyW<CntSpec> {
        UifcpyW::new(self, 31)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {
    const RESET_VALUE: u32 = 0;
}
