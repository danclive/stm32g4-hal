#[doc = "Register `GCOMP` reader"]
pub type R = crate::R<GcompSpec>;
#[doc = "Register `GCOMP` writer"]
pub type W = crate::W<GcompSpec>;
#[doc = "Field `GCOMPCOEFF` reader - GCOMPCOEFF"]
pub type GcompcoeffR = crate::FieldReader<u16>;
#[doc = "Field `GCOMPCOEFF` writer - GCOMPCOEFF"]
pub type GcompcoeffW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - GCOMPCOEFF"]
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GcompcoeffR {
        GcompcoeffR::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCOMP")
            .field("gcompcoeff", &self.gcompcoeff())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - GCOMPCOEFF"]
    #[inline(always)]
    #[must_use]
    pub fn gcompcoeff(&mut self) -> GcompcoeffW<GcompSpec> {
        GcompcoeffW::new(self, 0)
    }
}
#[doc = "Gain compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcompSpec;
impl crate::RegisterSpec for GcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcomp::R`](R) reader structure"]
impl crate::Readable for GcompSpec {}
#[doc = "`write(|w| ..)` method takes [`gcomp::W`](W) writer structure"]
impl crate::Writable for GcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCOMP to value 0"]
impl crate::Resettable for GcompSpec {
    const RESET_VALUE: u32 = 0;
}
