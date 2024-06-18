#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WdataSpec>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WdataSpec>;
#[doc = "Field `ARG` reader - ARG"]
pub type ArgR = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - ARG"]
pub type ArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ARG"]
    #[inline(always)]
    pub fn arg(&self) -> ArgR {
        ArgR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDATA").field("arg", &self.arg()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ARG"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ArgW<WdataSpec> {
        ArgW::new(self, 0)
    }
}
#[doc = "FMAC Write Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdataSpec;
impl crate::RegisterSpec for WdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WdataSpec {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WdataSpec {
    const RESET_VALUE: u32 = 0;
}
