#[doc = "Register `BKP24R` reader"]
pub type R = crate::R<Bkp24rSpec>;
#[doc = "Register `BKP24R` writer"]
pub type W = crate::W<Bkp24rSpec>;
#[doc = "Field `BKP` reader - BKP"]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKP24R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BkpW<Bkp24rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp24r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp24r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp24rSpec;
impl crate::RegisterSpec for Bkp24rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp24r::R`](R) reader structure"]
impl crate::Readable for Bkp24rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp24r::W`](W) writer structure"]
impl crate::Writable for Bkp24rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP24R to value 0"]
impl crate::Resettable for Bkp24rSpec {
    const RESET_VALUE: u32 = 0;
}
