#[doc = "Register `VREFBUF_CCR` reader"]
pub type R = crate::R<VrefbufCcrSpec>;
#[doc = "Register `VREFBUF_CCR` writer"]
pub type W = crate::W<VrefbufCcrSpec>;
#[doc = "Field `TRIM` reader - Trimming code"]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - Trimming code"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Trimming code"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VREFBUF_CCR")
            .field("trim", &self.trim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Trimming code"]
    #[inline(always)]
    pub fn trim(&mut self) -> TrimW<'_, VrefbufCcrSpec> {
        TrimW::new(self, 0)
    }
}
#[doc = "VREF_BUF Calibration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefbuf_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefbuf_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefbufCcrSpec;
impl crate::RegisterSpec for VrefbufCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefbuf_ccr::R`](R) reader structure"]
impl crate::Readable for VrefbufCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vrefbuf_ccr::W`](W) writer structure"]
impl crate::Writable for VrefbufCcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREFBUF_CCR to value 0"]
impl crate::Resettable for VrefbufCcrSpec {}
