#[doc = "Register `BKP0R` reader"]
pub type R = crate::R<Bkp0rSpec>;
#[doc = "Register `BKP0R` writer"]
pub type W = crate::W<Bkp0rSpec>;
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
        f.debug_struct("BKP0R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BkpW<Bkp0rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp0rSpec;
impl crate::RegisterSpec for Bkp0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp0r::R`](R) reader structure"]
impl crate::Readable for Bkp0rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp0r::W`](W) writer structure"]
impl crate::Writable for Bkp0rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP0R to value 0"]
impl crate::Resettable for Bkp0rSpec {
    const RESET_VALUE: u32 = 0;
}
