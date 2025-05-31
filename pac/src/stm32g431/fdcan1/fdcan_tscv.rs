#[doc = "Register `FDCAN_TSCV` reader"]
pub type R = crate::R<FdcanTscvSpec>;
#[doc = "Register `FDCAN_TSCV` writer"]
pub type W = crate::W<FdcanTscvSpec>;
#[doc = "Field `TSC` reader - Timestamp counter"]
pub type TscR = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp counter"]
pub type TscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TscR {
        TscR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TSCV")
            .field("tsc", &self.tsc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp counter"]
    #[inline(always)]
    pub fn tsc(&mut self) -> TscW<FdcanTscvSpec> {
        TscW::new(self, 0)
    }
}
#[doc = "FDCAN timestamp counter value register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTscvSpec;
impl crate::RegisterSpec for FdcanTscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tscv::R`](R) reader structure"]
impl crate::Readable for FdcanTscvSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tscv::W`](W) writer structure"]
impl crate::Writable for FdcanTscvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TSCV to value 0"]
impl crate::Resettable for FdcanTscvSpec {}
