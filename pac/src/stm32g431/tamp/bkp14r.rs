#[doc = "Register `BKP14R` reader"]
pub type R = crate::R<Bkp14rSpec>;
#[doc = "Register `BKP14R` writer"]
pub type W = crate::W<Bkp14rSpec>;
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
        f.debug_struct("BKP14R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<Bkp14rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "TAMP backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp14r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp14r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp14rSpec;
impl crate::RegisterSpec for Bkp14rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp14r::R`](R) reader structure"]
impl crate::Readable for Bkp14rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp14r::W`](W) writer structure"]
impl crate::Writable for Bkp14rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP14R to value 0"]
impl crate::Resettable for Bkp14rSpec {}
